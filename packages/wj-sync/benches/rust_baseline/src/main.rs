//! Minimal Rust baselines for wj-sync wall-clock comparison (mpsc / Mutex / AtomicI64 / threads).
//! Run: `cargo run --release` from this directory.

use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn channel_sum(n: i64) -> (i64, i64) {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    for i in 0..n {
        tx.send(i).unwrap();
    }
    drop(tx);
    tx2.send(-1).ok();
    let mut count = 0i64;
    let mut sum = 0i64;
    while let Ok(v) = rx.recv() {
        if v < 0 {
            break;
        }
        count += 1;
        sum += v;
    }
    (count, sum)
}

fn shared_incs(n: i64) -> i64 {
    let cell = Arc::new(Mutex::new(0i64));
    for _ in 0..n {
        let mut g = cell.lock().unwrap();
        *g += 1;
    }
    let g = cell.lock().unwrap();
    *g
}

fn counter_incs(n: i64) -> i64 {
    let cell = Arc::new(AtomicI64::new(0));
    for _ in 0..n {
        cell.fetch_add(1, Ordering::Relaxed);
    }
    cell.load(Ordering::Relaxed)
}

fn pool_shared_inbox_sum(workers: usize, n: i64) -> (i64, i64) {
    let (res_tx, res_rx) = mpsc::channel::<i64>();
    let mut job_txs = Vec::with_capacity(workers);
    let mut handles = Vec::with_capacity(workers);
    for _ in 0..workers {
        let (job_tx, job_rx) = mpsc::channel::<i64>();
        job_txs.push(job_tx);
        let tx = res_tx.clone();
        handles.push(thread::spawn(move || loop {
            match job_rx.recv() {
                Ok(v) if v < 0 => break,
                Ok(v) => {
                    let _ = tx.send(v * 2);
                }
                Err(_) => break,
            }
        }));
    }
    drop(res_tx);
    for i in 0..n {
        job_txs[(i as usize) % workers].send(i).unwrap();
    }
    for tx in &job_txs {
        tx.send(-1).unwrap();
    }
    drop(job_txs);
    let mut sum = 0i64;
    let mut got = 0i64;
    while got < n {
        if let Ok(v) = res_rx.recv() {
            sum += v;
            got += 1;
        } else {
            break;
        }
    }
    for h in handles {
        let _ = h.join();
    }
    (got, sum)
}

fn main() {
    let n_chan = 1_000_000i64;
    let t0 = Instant::now();
    let c = channel_sum(n_chan);
    let chan_ms = t0.elapsed().as_millis();
    println!("rust_channel_sum n={n_chan} count={} sum={} elapsed_ms={chan_ms}", c.0, c.1);

    let n_shared = 1_000_000i64;
    let t1 = Instant::now();
    let s = shared_incs(n_shared);
    let shared_ms = t1.elapsed().as_millis();
    println!("rust_shared_incs n={n_shared} value={s} elapsed_ms={shared_ms}");

    let n_counter = 1_000_000i64;
    let t1b = Instant::now();
    let ctr = counter_incs(n_counter);
    let counter_ms = t1b.elapsed().as_millis();
    println!("rust_counter_incs n={n_counter} value={ctr} elapsed_ms={counter_ms}");

    let n_pool = 100_000i64;
    let t2 = Instant::now();
    let p = pool_shared_inbox_sum(4, n_pool);
    let pool_ms = t2.elapsed().as_millis();
    println!(
        "rust_pool_shared_inbox n={n_pool} count={} sum={} elapsed_ms={pool_ms}",
        p.0, p.1
    );
}
