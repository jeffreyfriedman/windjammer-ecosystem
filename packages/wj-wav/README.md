# wj-wav

Read and write RIFF/WAVE audio from byte slices. No file system access: pass bytes in, get bytes or samples out.

- Parse a header into `WavInfo` (channels, sample rate, bit depth, PCM or float, frame count, data offset).
- Decode PCM16, PCM24, PCM32 and float32 to interleaved `f32` samples in -1.0 to 1.0.
- Encode `f32` or `i16` samples to a canonical 44-byte-header PCM16 WAV; the output is deterministic.
- Skips unknown chunks (`LIST`, `fact`, ...), honours the odd-size pad byte, recognises `WAVE_FORMAT_EXTENSIBLE`.
- Bad input returns a `WavError`; nothing panics.

## API

```windjammer
pub fn parse(bytes: Vec<u8>) -> Result<WavInfo, WavError>
pub fn parse_header(prefix: Vec<u8>) -> Result<WavInfo, WavError>
pub fn decode(bytes: Vec<u8>) -> Result<Vec<f32>, WavError>
pub fn encode_f32(samples: Vec<f32>, channels: int, sample_rate: int) -> Result<Vec<u8>, WavError>
pub fn encode_i16(samples: Vec<i16>, channels: int, sample_rate: int) -> Result<Vec<u8>, WavError>
pub fn quantize(sample: f32) -> i16
pub fn duration_seconds(info: WavInfo) -> f64
pub fn describe(error: WavError) -> string
```

- `parse` needs the whole data chunk. A data size of `0xFFFFFFFF` (a streamed file) means the data runs to the end of the input.
- `parse_header` reads only the header from the first N bytes of a file; the sample bytes need not be present. `data_len` and `frame_count` come from the size the header declares.
- `WavInfo` fields: `channels`, `sample_rate`, `bits_per_sample`, `format` (`SampleFormat::Pcm` or `SampleFormat::Float`), `extensible`, `block_align`, `data_offset`, `data_len`, `frame_count`.
- `WavError`: `TooShort`, `BadMagic`, `Truncated`, `MissingFormat`, `MissingData`, `BadFormat`, `Unsupported`.
- Samples convert with a scale of 32768 in both directions (`quantize` rounds, then clamps to -32768..32767), so a round trip is within half a 16-bit step, except that 1.0 encodes as 32767.
- A trailing partial frame in the data chunk is ignored.

## Example

Generate a 0.1 s, 440 Hz tone, write it, read it back:

```windjammer
use std::fs
use std::math
use wj_wav::encode_f32
use wj_wav::parse
use wj_wav::duration_seconds
use wj_wav::describe

fn main() {
    let rate = 8000
    let mut samples: Vec<f32> = []
    let mut n = 0
    while n < 800 {
        let phase = 2.0 * math.PI * 440.0 * (n as f64) / (rate as f64)
        samples.push((0.5 * phase.sin()) as f32)
        n = n + 1
    }
    match encode_f32(samples, 1, rate) {
        Ok(wav) => {
            match fs.write_bytes("tone.wav", wav) {
                Ok(_) => println("wrote tone.wav"),
                Err(e) => println("write failed: ${e}"),
            }
        },
        Err(e) => println("encode failed: ${describe(e)}"),
    }
    match fs.read("tone.wav") {
        Ok(bytes) => match parse(bytes) {
            Ok(info) => println("${info.channels} ch, ${info.sample_rate} Hz, ${info.frame_count} frames, ${duration_seconds(info)} s"),
            Err(e) => println("not a WAV: ${describe(e)}"),
        },
        Err(e) => println("read failed: ${e}"),
    }
}
```

Run with `wj run --interpret --allow fs.write --allow fs.read --allow console main.wj`. It prints `wrote tone.wav` and `1 ch, 8000 Hz, 800 frames, 0.1 s`, and the 1644-byte file opens in any audio tool.

## Layout

```
src/lib.wj
tests/wav_test.wj
```

## Build / test

```bash
cd packages/wj-wav
wj test --interpret
```

## License

MIT OR Apache-2.0
