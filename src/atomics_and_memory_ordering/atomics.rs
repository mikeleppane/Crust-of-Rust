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
        while self
            .locked
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            while self.locked.load(Ordering::Relaxed) == LOCKED {}
        }
        self.locked.store(LOCKED, Ordering::Relaxed);
        // SAFETY: we hold the lock
        let ret = f(unsafe { &mut *self.v.get() });
        self.locked.store(UNLOCKED, Ordering::Release);
        ret
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicUsize;
    use std::thread::spawn;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_works() {
        let l: &'static _ = Box::leak(Box::new(MyMutex::new(0)));
        let handles: Vec<_> = (0..100)
            .map(|_| {
                spawn(move || {
                    for _ in 0..1000 {
                        l.with_lock(|v| *v += 1)
                    }
                })
            })
            .collect();
        for handle in handles {
            handle.join().unwrap();
        }
        assert_eq!(l.with_lock(|v| *v), 100 * 1000)
    }

    #[test]
    fn too_relaxed() {
        let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
        let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
        let t1 = spawn(move || {
            let r1 = y.load(Ordering::Acquire);
            x.store(r1, Ordering::Release);
            r1
        });
        let t2 = spawn(move || {
            let r2 = x.load(Ordering::Acquire);
            y.store(42, Ordering::Release);
            r2
        });

        let r1 = t1.join().unwrap();
        let r2 = t2.join().unwrap();

        assert_eq!(r1, r2)
    }

    #[test]
    fn mutex_test() {
        let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
        let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
        let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
        spawn(move || {
            x.store(true, Ordering::SeqCst);
        });
        spawn(move || {
            y.store(true, Ordering::SeqCst);
        });
        let t1 = spawn(move || {
            while !x.load(Ordering::SeqCst) {}
            if y.load(Ordering::SeqCst) {
                z.fetch_add(1, Ordering::Relaxed);
            }
        });
        let t2 = spawn(move || {
            while !y.load(Ordering::SeqCst) {}
            if x.load(Ordering::SeqCst) {
                z.fetch_add(1, Ordering::Relaxed);
            }
        });
        t1.join().unwrap();
        t2.join().unwrap();
        let z = z.load(Ordering::SeqCst);
        assert_eq!(z, 2)
    }
}
