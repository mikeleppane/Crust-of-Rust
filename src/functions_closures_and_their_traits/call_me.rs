pub fn main() {
    println!("call me!!!");
    let x = bar::<i32>;
    println!("{}", std::mem::size_of_val(&x));
    baz(bar::<u32>);
    baz(bar::<i32>);
    quox(&mut bar::<u32>);
    let mut z = String::new();
    let mut f = || {
        z.clear();
    };
    //baz(f);
    quox(&mut f);
}

fn bar<T>() {}

fn baz(f: fn()) {
    println!("{}", std::mem::size_of_val(&f));
}

pub fn quox<F>(f: &mut F)
where
    F: FnMut(),
{
    (f)()
}

// const fn foo<F: FnOnce()>(f: ~const F) {
//     f()
// }
