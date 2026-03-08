#![allow(dead_code)]

use std::{sync::{atomic::{AtomicU32, Ordering}, Arc}, thread};

pub fn count_to_2() -> u32 {
    let atomic = AtomicU32::new(0);
    let shared_atomic = Arc::from(atomic);

    let handle1 = {
        let shared_atomic_clone = shared_atomic.clone();
        thread::spawn(move || {
            shared_atomic_clone.fetch_add(1, Ordering::SeqCst);
        })
    };

    let handle2 = {
        let shared_atomic_clone = shared_atomic.clone();
        thread::spawn(move || {
            shared_atomic_clone.fetch_add(1, Ordering::SeqCst);
        })
    };

    if let Err(_) = handle1.join() { return 0 };
    if let Err(_) = handle2.join() { return 0 };

    return shared_atomic.load(Ordering::SeqCst);
}