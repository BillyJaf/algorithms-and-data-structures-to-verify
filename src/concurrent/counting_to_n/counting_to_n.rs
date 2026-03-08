#![allow(dead_code)]

use std::{sync::{atomic::{AtomicU32, Ordering}, Arc}, thread};

pub fn count_to_n(n: u32) -> u32 {
    let atomic = AtomicU32::new(0);
    let shared_atomic = Arc::from(atomic);

    let mut handles = Vec::new();

    for _ in 0..n {
        let shared_atomic_clone = shared_atomic.clone();
        handles.push(
            thread::spawn(move || {
                shared_atomic_clone.fetch_add(1, Ordering::SeqCst);
            })
        );
    }

    for handle in handles.into_iter() {
        if let Err(_) = handle.join() { return 0 };
    }

    return shared_atomic.load(Ordering::SeqCst);
}