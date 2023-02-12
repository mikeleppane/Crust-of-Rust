pub trait Hei {
    fn hei(&self);

    fn not_ok()
    where
        Self: Sized,
    {
    }
}

impl Hei for &str {
    fn hei(&self) {
        println!("hei {}", self);
    }

    fn not_ok() {
        todo!()
    }
}

impl Hei for String {
    fn hei(&self) {
        println!("hei {}", self);
    }

    fn not_ok() {
        todo!()
    }
}

fn strlen(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

fn strlen_dyn(s: Box<dyn AsRef<str>>) -> usize {
    s.as_ref().as_ref().len()
}

pub fn bool_then<T>(b: bool, f: impl FnOnce() -> T) -> Option<T> {
    if b {
        Some(f())
    } else {
        None
    }
}

pub trait HeiAsRef: Hei + AsRef<str> {}

pub fn baz(s: &dyn HeiAsRef) {
    s.hei();
    let s = s.as_ref();
    s.len();
}

pub fn slice_of_trait_object(v: &[&dyn AsRef<str>]) {
    for s in v {
        s.as_ref();
    }
}

pub fn main() {
    let x = Box::new(String::from("hello"));
    strlen_dyn(x);
    slice_of_trait_object(&[&"hello"]);
    slice_of_trait_object(&[&String::from("hello")]);
    slice_of_trait_object(&[Box::new("hello").as_ref()])
}

// =====================================

// pub fn add_true(v: &mut dyn Extend<bool>) {
//     v.extend(std::iter::once(true));
// }
//
// struct MyVec<T>(Vec<T>);
//
// impl<T> Extend<T> for MyVec<T> {
//     fn extend<I>(&mut self, iter: I)
//     where
//         I: IntoIterator<Item = T>,
//     {
//         // **
//     }
// }
