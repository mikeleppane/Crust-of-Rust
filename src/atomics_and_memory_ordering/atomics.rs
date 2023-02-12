use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

const LOCKED: bool = true;
const UNLOCKED: bool = false;

pub struct MyMutex<T> {
    locked: AtomicBool,
    v: UnsafeCell<T>,
}

unsafe impl<T> Sync for MyMutex<T> where T: Send {}

impl<T> MyMutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            locked: AtomicBool::new(UNLOCKED),
            v: UnsafeCell::new(t),
        }
    }

    fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self.locked.load(Ordering::Relaxed) != UNLOCKED {}
        self.locked.store(LOCKED, Ordering::Relaxed);
        // SAFETY: we hold the lock
        let ret = f(unsafe { &mut *self.v.get() });
        self.locked.store(UNLOCKED, Ordering::Relaxed);
        ret
    }
}

#[cfg(test)]
mod tests {
    use std::thread::spawn;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_works() {
        // let l = Box::leak(Box::new(MyMutex::new(0)));
        // let handles: Vec<_> = (0..10)
        //     .map(|_| {
        //         spawn(move || {
        //             for _ in 0..100 {
        //                 l.with_lock(|v| {
        //                     *v += 1;
        //                 })
        //             }
        //         })
        //     })
        //     .collect();
        // for handle in handles {
        //     handle.join().unwrap();
        // }
    }
}
