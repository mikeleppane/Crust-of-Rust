use std::ops::{Deref, DerefMut};

pub struct Boks<T> {
    p: *mut T,
}

impl<T> Drop for Boks<T> {
    fn drop(&mut self) {
        // SAFETY:
        unsafe { Box::from_raw(self.p) };
    }
}

impl<T> Deref for Boks<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: is valid since it was constructed from a valid T, and turned into a pointer
        // through Box which creates aligned pointers, and hasn't need freed, since self is alive.
        unsafe { &*self.p }
    }
}

impl<T> DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: is valid since it was constructed from a valid T, and turned into a pointer
        // through Box which creates aligned pointers, and hasn't need freed, since self is alive.
        unsafe { &mut *self.p }
    }
}

impl<T> Boks<T> {
    pub fn ny(t: T) -> Self {
        Boks {
            p: Box::into_raw(Box::new(t)),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_works() {
        let x = 42;
        let boks = Boks::ny(x);
        println!("{:?}", *boks);

        let mut y = 42;
        let boks = Boks::ny(&mut y);
        //println!("{:?}", y);
    }
}
