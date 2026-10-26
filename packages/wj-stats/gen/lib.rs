use std::cmp::Ordering;
/// Sum of the samples.
#[inline]
pub fn sum(xs: Vec<f64>) -> f64 {
    let mut total = 0.0_f64;
    for x in xs {
        total += x;
    }
    total
}

/// Smallest sample (0.0 for no samples).
#[inline]
pub fn min(xs: &Vec<f64>) -> f64 {
    if xs.is_empty() {
        return 0.0_f64;
    }
    let mut best = xs[0];
    for x in xs.iter().copied() {
        if x < best {
            best = x;
        }
    }
    best
}

/// Largest sample (0.0 for no samples).
#[inline]
pub fn max(xs: &Vec<f64>) -> f64 {
    if xs.is_empty() {
        return 0.0_f64;
    }
    let mut best = xs[0];
    for x in xs.iter().copied() {
        if x > best {
            best = x;
        }
    }
    best
}

/// The samples in ascending order.
#[inline]
pub fn sorted(xs: Vec<f64>) -> Vec<f64> {
    let mut out: Vec<f64> = xs;
    out.sort_by(move |a, b| a.partial_cmp(&b).unwrap_or(Ordering::Equal));
    out
}

/// Median: the middle sample, or the mean of the two middle samples (Python's `statistics.median`).
#[inline]
pub fn median(xs: Vec<f64>) -> f64 {
    let n = xs.len();
    if n == 0_usize {
        return 0.0_f64;
    }
    let s: Vec<f64> = sorted(xs.clone());
    if n % 2_usize == 1_usize {
        return s[n / 2];
    }
    (s[n / 2 - 1] + s[n / 2]) / 2.0_f64
}

/// Arithmetic mean (0.0 for no samples).
#[inline]
pub fn mean(xs: Vec<f64>) -> f64 {
    if xs.is_empty() {
        return 0.0_f64;
    }
    let n = xs.len() as f64;
    sum(xs.clone()) / n
}

/// Population standard deviation (Python's `statistics.pstdev`).
pub fn pstdev(xs: Vec<f64>) -> f64 {
    let n = xs.len();
    if n == 0_usize {
        return 0.0_f64;
    }
    let m = mean(xs.clone());
    let mut acc = 0.0_f64;
    for x in xs {
        acc += (x - m) * (x - m);
    }
    (acc / n as f64).sqrt()
}

/// Sample standard deviation (Python's `statistics.stdev`; 0.0 below two samples).
pub fn stdev(xs: Vec<f64>) -> f64 {
    let n = xs.len();
    if n < 2_usize {
        return 0.0_f64;
    }
    let m = mean(xs.clone());
    let mut acc = 0.0_f64;
    for x in xs {
        acc += (x - m) * (x - m);
    }
    (acc / (n - 1_usize) as f64).sqrt()
}

