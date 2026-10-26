use windjammer_runtime::fs;
use windjammer_runtime::fs::DirEntry;
use windjammer_runtime::strings;
/// `Ok(name)` when `name` is a hidden dotfile name, otherwise `Err`.
#[inline]
pub fn require_hidden(name: String) -> Result<String, String> {
    if is_hidden_name(name.clone()) {
        return Ok(name);
    }
    Err(String::from("expected hidden name"))
}

/// Final path segment of `path`.
#[inline]
pub fn basename_of(path: String) -> String {
    path_basename(path)
}

/// True when `name` starts with `.` (dotfile / hidden directory name).
#[inline]
pub fn is_hidden_name(name: String) -> bool {
    strings::starts_with(&name, ".")
}

/// Depth-first list of every file and directory path under `root` (not including `root`).
#[inline]
pub fn walk(root: &str) -> Result<Vec<String>, String> {
    walk_into(root, true, true)
}

/// Depth-first list of file paths under `root`.
#[inline]
pub fn walk_files(root: &str) -> Result<Vec<String>, String> {
    walk_into(root, true, false)
}

/// Depth-first list of directory paths under `root` (not including `root`).
#[inline]
pub fn walk_dirs(root: &str) -> Result<Vec<String>, String> {
    walk_into(root, false, true)
}

/// Files under `root` whose path ends with `suffix` (e.g. `.md`).
#[inline]
pub fn walk_files_with_suffix(root: &str, suffix: String) -> Result<Vec<String>, String> {
    match walk_files(root) {
        Ok(paths) => {
            let mut filtered = Vec::new();
            for path in paths {
                if strings::ends_with(&path, &suffix) {
                    let _ = filtered.push(path);
                }
            }
            Ok(filtered)
        },
        Err(e) => Err(e),
    }
}

/// Number of files under `root` (includes hidden files).
#[inline]
pub fn count_files(root: &str) -> Result<i64, String> {
    match walk_files(root) {
        Ok(paths) => {
            let mut n = 0_i64;
            for __path in paths {
                n += 1;
            }
            Ok(n)
        },
        Err(e) => Err(e),
    }
}

/// True when `root` contains at least one file.
#[inline]
pub fn has_files(root: &str) -> Result<bool, String> {
    match count_files(root) {
        Ok(n) => Ok(n > 0_i64),
        Err(e) => Err(e),
    }
}

/// Number of directories under `root` (not including `root`).
#[inline]
pub fn count_dirs(root: &str) -> Result<i64, String> {
    match walk_dirs(root) {
        Ok(paths) => {
            let mut n = 0_i64;
            for __path in paths {
                n += 1;
            }
            Ok(n)
        },
        Err(e) => Err(e),
    }
}

/// True when `root` contains at least one subdirectory.
#[inline]
pub fn has_dirs(root: &str) -> Result<bool, String> {
    match count_dirs(root) {
        Ok(n) => Ok(n > 0_i64),
        Err(e) => Err(e),
    }
}

/// Number of files plus directories under `root`.
#[inline]
pub fn count_entries(root: &str) -> Result<i64, String> {
    match count_files(root) {
        Ok(files) => match count_dirs(root) {
            Ok(dirs) => Ok(files + dirs),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

/// True when `root` contains at least one file or directory.
#[inline]
pub fn has_entries(root: &str) -> Result<bool, String> {
    match count_entries(root) {
        Ok(n) => Ok(n > 0_i64),
        Err(e) => Err(e),
    }
}

/// `Ok(1)` when `root` contains a file or directory, otherwise `Err`.
#[inline]
pub fn require_entries(root: &str) -> Result<i64, String> {
    match has_entries(root) {
        Ok(yes) => {
            if yes {
                return Ok(1);
            }
            Err(String::from("expected entries"))
        },
        Err(e) => Err(e),
    }
}

/// Like `walk_files`, but skips entries whose basename starts with `.`.
#[inline]
pub fn walk_files_skip_hidden(root: &str) -> Result<Vec<String>, String> {
    match walk_files(root) {
        Ok(paths) => {
            let mut filtered = Vec::new();
            for path in paths {
                if !is_hidden_name(path_basename(path.clone())) {
                    let _ = filtered.push(path.clone());
                }
            }
            Ok(filtered)
        },
        Err(e) => Err(e),
    }
}

/// Number of non-hidden files under `root`.
#[inline]
pub fn count_visible_files(root: &str) -> Result<i64, String> {
    match walk_files_skip_hidden(root) {
        Ok(paths) => {
            let mut n = 0_i64;
            for __path in paths {
                n += 1;
            }
            Ok(n)
        },
        Err(e) => Err(e),
    }
}

/// True when `root` contains at least one file ending with `suffix`.
#[inline]
pub fn has_suffix_files(root: &str, suffix: String) -> Result<bool, String> {
    match walk_files_with_suffix(root, suffix) {
        Ok(paths) => Ok(!paths.is_empty()),
        Err(e) => Err(e),
    }
}

/// `Ok(1)` when `root` has a file ending in `suffix`, otherwise `Err`.
#[inline]
pub fn require_suffix(root: &str, suffix: String) -> Result<i64, String> {
    match has_suffix_files(root, suffix) {
        Ok(yes) => {
            if yes {
                return Ok(1);
            }
            Err(String::from("expected suffix"))
        },
        Err(e) => Err(e),
    }
}

/// File count under `root`, or `Err` when there are none.
#[inline]
pub fn require_files(root: &str) -> Result<i64, String> {
    match count_files(root) {
        Ok(n) => {
            if n > 0_i64 {
                return Ok(n);
            }
            Err(String::from("expected files"))
        },
        Err(e) => Err(e),
    }
}

/// Directory count under `root`, or `Err` when there are none.
#[inline]
pub fn require_dirs(root: &str) -> Result<i64, String> {
    match count_dirs(root) {
        Ok(n) => {
            if n > 0_i64 {
                return Ok(n);
            }
            Err(String::from("expected dirs"))
        },
        Err(e) => Err(e),
    }
}

/// True when `root` has no files or subdirectories.
#[inline]
pub fn is_empty_tree(root: &str) -> Result<bool, String> {
    match has_entries(root) {
        Ok(yes) => Ok(!yes),
        Err(e) => Err(e),
    }
}

/// `Ok(0)` when `root` has no files or subdirectories.
#[inline]
pub fn require_empty_tree(root: &str) -> Result<i64, String> {
    match is_empty_tree(root) {
        Ok(yes) => {
            if yes {
                return Ok(0);
            }
            Err(String::from("expected empty"))
        },
        Err(e) => Err(e),
    }
}

#[inline]
fn path_basename(path: String) -> String {
    let parts = strings::split(&path, "/");
    if parts.is_empty() {
        return path;
    }
    (&parts[parts.len() - 1]).clone()
}


fn walk_into(dir: &str, include_files: bool, include_dirs: bool) -> Result<Vec<String>, String> {
    match fs::read_dir(dir) {
        Ok(entries) => {
            let mut out: Vec<String> = Vec::new();
            for entry in entries {
                let path = entry.path();
                if entry.is_dir() {
                    if include_dirs {
                        let _ = out.push(path.to_string());
                    }
                    match walk_into(&path, include_files, include_dirs) {
                        Ok(nested) => {
                            for p in &nested {
                                out.push(p.clone());
                            }
                        },
                        Err(e) => {
                            return Err(e);
                        },
                    }
                } else {
                    if entry.is_file() {
                        if include_files {
                            let _ = out.push(path);
                        }
                    }
                }
            }
            Ok(out)
        },
        Err(e) => Err(e),
    }
}

