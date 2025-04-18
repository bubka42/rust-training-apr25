/// This function takes a vector of u32s and a function f: u32 -> u64,
/// maps f to the vector, and computes the sum using N threads,
/// without using rayon, channels, atomics, or scoped threads.
pub fn map_sum1(v: Vec<u32>, f: impl Fn(u32) -> u64 + Send + Copy + 'static, n: usize) -> u64 {
    let mut threads = Vec::new();
    let chunk_size = (v.len() + n - 1) / n;
    let mut results = vec![0; n];

    for i in 0..n {
        let start = i * chunk_size;
        let end = std::cmp::min(start + chunk_size, v.len());
        if start >= end {
            break;
        }
        let chunk = v[start..end].to_vec();
        threads.push(std::thread::spawn(move || chunk.into_iter().map(&f).sum()));
    }

    for (i, thread) in threads.into_iter().enumerate() {
        results[i] = thread.join().unwrap();
    }

    results.into_iter().sum()
}

/// This function takes a vector of u32s and a function f: u32 -> u64,
/// maps f to the vector, and computes the sum using N threads and AtomicU64 counter,
/// without using rayon, channels, or scoped threads.
pub fn map_sum2(v: Vec<u32>, f: impl Fn(u32) -> u64 + Send + Copy + 'static, n: usize) -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Arc;

    let counter = Arc::new(AtomicU64::new(0));
    let chunk_size = (v.len() + n - 1) / n;
    let mut threads = Vec::new();

    for i in 0..n {
        let start = i * chunk_size;
        let end = std::cmp::min(start + chunk_size, v.len());
        if start >= end {
            break;
        }
        let chunk = v[start..end].to_vec();
        let counter_clone = Arc::clone(&counter);
        threads.push(std::thread::spawn(move || {
            for &x in &chunk {
                counter_clone.fetch_add(f(x), Ordering::SeqCst);
            }
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    counter.load(Ordering::SeqCst)
}

/// This function takes a vector of u32s and a function f: u32 -> u64,
/// maps f to the vector, and computes the sum using N threads and MPSC channels,
/// without using rayon, atomics, or scoped threads.
pub fn map_sum3(v: Vec<u32>, f: impl Fn(u32) -> u64 + Send + Copy + 'static, n: usize) -> u64 {
    use std::sync::mpsc;
    use std::thread;

    let (tx, rx) = mpsc::channel();
    let chunk_size = (v.len() + n - 1) / n;
    let mut threads = Vec::new();

    for i in 0..n {
        let start = i * chunk_size;
        let end = std::cmp::min(start + chunk_size, v.len());
        if start >= end {
            break;
        }
        let chunk = v[start..end].to_vec();
        let tx_clone = tx.clone();
        threads.push(thread::spawn(move || {
            for &x in &chunk {
                tx_clone.send(f(x)).unwrap();
            }
        }));
    }

    drop(tx); // Close the sending end

    let mut sum = 0;
    for received in rx {
        sum += received;
    }

    for thread in threads {
        thread.join().unwrap();
    }

    sum
}

/// This function takes a vector of u32s and a function f: u32 -> u64,
/// maps f to the vector, and computes the sum using N threads and rayon,
/// without using atomics, channels, or scoped threads.
pub fn map_sum4(
    v: Vec<u32>,
    f: impl Fn(u32) -> u64 + Send + Sync + Copy + 'static,
    _n: usize,
) -> u64 {
    use rayon::prelude::*;

    v.into_par_iter().map(f).sum()
}
