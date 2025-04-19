/// This function takes a vector of u32s and a function f: u32 -> u64,
/// maps f to the vector, and computes the sum using N threads,
/// without using rayon, channels, atomics, or scoped threads.
pub fn map_sum1(v: Vec<u32>, f: impl Fn(u32) -> u64 + Send + Copy + 'static, n: usize) -> u64 {
    let mut threads = Vec::new();
    let chunk_size = v.len().div_ceil(n);
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
    let chunk_size = v.len().div_ceil(n);
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
    let chunk_size = v.len().div_ceil(n);
    let mut threads = Vec::new();

    for i in 0..n {
        let start = i * chunk_size;
        let end = std::cmp::min(start + chunk_size, v.len());
        if start >= end {
            break;
        }
        let chunk = v[start..end].to_vec();
        let tx_clone: mpsc::Sender<u64> = tx.clone();
        threads.push(thread::spawn(move || {
            tx_clone.send(chunk.into_iter().map(&f).sum()).unwrap();
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
    n: usize,
) -> u64 {
    use rayon::prelude::*;
    rayon::ThreadPoolBuilder::new()
        .num_threads(n)
        .build_global()
        .unwrap();

    v.into_par_iter().map(f).sum()
}

/// This function takes a mutable slice of u32s and a function f: u32 -> u32,
/// maps f to the slice and writes the results in the input buffer,
/// and computes the sum using N scoped threads.
pub fn map_sum5(
    v: &mut [u32],
    f: impl Fn(u32) -> u32 + Send + Copy + Sync + 'static,
    n: usize,
) -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;
    use std::thread;

    let counter = Arc::new(AtomicU32::new(0));
    let chunk_size = v.len().div_ceil(n);

    thread::scope(|s| {
        for chunk in v.chunks_mut(chunk_size) {
            let counter_clone = Arc::clone(&counter);
            s.spawn(move || {
                for x in chunk.iter_mut() {
                    *x = f(*x);
                    counter_clone.fetch_add(*x, Ordering::SeqCst);
                }
            });
        }
    });
    counter.load(Ordering::SeqCst)
}
