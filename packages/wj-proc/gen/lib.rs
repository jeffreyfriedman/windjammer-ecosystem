use windjammer_runtime::process;
use windjammer_runtime::process::ProcessOutput;
use windjammer_runtime::strings;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct Output {
    pub status: i64,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Timed {
    pub output: Output,
    pub user: f64,
    pub sys: f64,
}

/// Run a command line through `sh -c`.
#[inline]
pub fn sh(command: String) -> Result<Output, String> {
    capture("sh".to_string(), vec!["-c".to_string(), command])
}

/// Run `program` and report its user and system CPU seconds (child processes included).
pub fn timed(program: String, args: Vec<String>) -> Result<Timed, String> {
    let mut all = vec!["-p".to_string(), program];
    for a in args {
        all.push(a);
    }
    let out = capture("/usr/bin/time".to_string(), all.clone())?;
    let mut user = 0.0_f64;
    let mut sys = 0.0_f64;
    let mut kept = Vec::new();
    for line in strings::split_lines(&out.stderr) {
        let parts = strings::split_whitespace(&line);
        if parts.len() == 2_usize && parts[0] == "user" {
            user = parts[1].parse::<f64>().unwrap_or(0.0_f64);
        } else {
            if parts.len() == 2_usize && parts[0] == "sys" {
                sys = parts[1].parse::<f64>().unwrap_or(0.0_f64);
            } else {
                if parts.len() == 2_usize && parts[0] == "real" {
                    continue;
                } else {
                    kept.push(line)
                }
            }
        }
    }
    let err_text = {
        if kept.is_empty() {
            String::new()
        } else {
            format!("{}{}", kept.join("\n"), "\n")
        }
    };
    Ok(Timed { output: Output { status: out.status, stdout: out.stdout, stderr: err_text }, user, sys })
}

/// The 1-minute load average (0.0 when `uptime` cannot be read or parsed).
#[inline]
pub fn load_avg() -> f64 {
    match sh("uptime".to_string()) {
        Ok(o) => parse_load(&o.stdout),
        Err(_) => 0.0_f64,
    }
}

/// The 1, 5 and 15 minute load averages (zeros when `uptime` cannot be read or parsed).
#[inline]
pub fn load_avgs() -> Vec<f64> {
    match sh("uptime".to_string()) {
        Ok(o) => parse_loads(&o.stdout),
        Err(_) => [0.0_f64, 0.0_f64, 0.0_f64],
    }
}

/// All load-average figures in `uptime` text, padded with zeros to three.
pub fn parse_loads(text: &str) -> Vec<f64> {
    let mut out: Vec<f64> = Vec::new();
    match text.find("load average") {
        Some(i) => {
            let rest = &text[i..text.len()];
            match rest.find(":") {
                Some(c) => {
                    let tail = strings::replace(&((strings::trim(&(((rest[((c + 1_i32)) as usize..rest.len() as usize]).to_string()))))), ",", " ");
                    for tok in strings::split_whitespace(&tail) {
                        let _ = out.push(tok.parse::<f64>().unwrap_or(0.0_f64));
                    }
                },
                None => {
                },
            }
        },
        None => {
        },
    }
    while out.len() < 3_usize {
        let _ = out.push(0.0_f64);
    }
    out
}

/// First load-average figure in `uptime` text (`load averages: 1.5 2.0 2.5` or `load average: 1.50, 2.00, 2.50`).

pub fn parse_load(text: &str) -> f64 {
    let marker: String = String::from("load average");
    let at = text.find(&marker);
    match at {
        Some(i) => {
            let rest = &text[i..text.len()];
            let colon = rest.find(":");
            match colon {
                Some(c) => {
                    let tail = strings::trim(&(((rest[((c + 1)) as usize..rest.len() as usize]).to_string())));
                    let cleaned = strings::replace(&tail, ",", " ");
                    let parts = strings::split_whitespace(&cleaned);
                    if parts.is_empty() {
                        return 0.0_f64;
                    }
                    parts[0].parse::<f64>().unwrap_or(0.0_f64)
                },
                None => 0.0_f64,
            }
        },
        None => 0.0_f64,
    }
}

/// The repository root containing `dir`.
#[inline]
pub fn git_toplevel(dir: String) -> Result<String, String> {
    let o = capture("git".to_string(), vec!["-C".to_string(), dir, "rev-parse".to_string(), "--show-toplevel".to_string()])?;
    if o.status != 0_i64 {
        return Err(strings::trim(&o.stderr));
    }
    Ok(strings::trim(&o.stdout))
}

/// Tracked files under `dir` matching the git pathspecs (all tracked files when `specs` is empty), sorted.
pub fn git_ls_files(dir: String, specs: Vec<String>) -> Result<Vec<String>, String> {
    let mut args = vec!["-C".to_string(), dir, "ls-files".to_string(), "--".to_string()];
    for s in specs {
        args.push(s);
    }
    let o = capture("git".to_string(), args.clone())?;
    if o.status != 0_i64 {
        return Err(strings::trim(&o.stderr));
    }
    let mut files = Vec::new();
    for line in strings::split_lines(&o.stdout) {
        if strings::len(&line) > 0_usize {
            let _ = files.push(line);
        }
    }
    files.sort();
    Ok(files)
}

/// Paths with changes not yet staged (`git diff --name-only`), sorted.
pub fn git_changed(dir: &str) -> Result<Vec<String>, String> {
    let o = capture("git".to_string(), vec!["-C".to_string(), dir.to_string(), "diff".to_string(), "--name-only".to_string()])?;
    if o.status != 0_i64 {
        return Err(strings::trim(&o.stderr));
    }
    let mut files = Vec::new();
    for line in strings::split_lines(&o.stdout) {
        if strings::len(&line) > 0_usize {
            let _ = files.push(line);
        }
    }
    files.sort();
    Ok(files)
}

/// `s` in single quotes, safe to splice into a `sh -c` command line.
#[inline]
pub fn quote(s: &str) -> String {
    String::from("'") + &strings::replace(s, "'", "'\\''") + &String::from("'")
}

/// Run `program` with `args`; Err when it cannot be started. A non-zero exit is an Ok(Output) with the status.
#[inline]
pub fn capture(program: String, args: Vec<String>) -> Result<Output, String> {
    match process::run_with_output(&program, &args) {
        Ok(o) => Ok(Output { status: o.status as i64, stdout: o.stdout, stderr: o.stderr }),
        Err(e) => Err(e),
    }
}

/// Run a command line through `sh -c` inside `dir`.
#[inline]
pub fn sh_in(dir: &str, command: &str) -> Result<Output, String> {
    { let _temp0 = format!("{}{}{}{}", "cd ", quote(dir), " && ", command); sh(_temp0) }
}

