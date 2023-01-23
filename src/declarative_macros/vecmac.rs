#[macro_export]
macro_rules! avec {
    ($($element:expr),* ) => {{
        #[allow(unused_mut)]
        let mut vs = Vec::with_capacity($crate::avec![@COUNT; $($element),*]);
        $(vs.push($element);)*
        vs
    }};

    ($($element:expr,)*) => {{
        $crate::avec![$($element),*]
    }};

    ($element:expr; $count:expr) => {{
        let mut vs = Vec::new();
        vs.resize($count, $element);
        vs
    }};

    (@COUNT; $($element:expr),*) => {
        <[()]>::len(&[$($crate::avec![@SUBST; $element]),*]);
    };

    (@SUBST; $_element:expr) => {
        ()
    };
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    #[test]
    fn empty_vec() {
        let x: Vec<u32> = avec![];
        assert!(x.is_empty());
    }

    #[test]
    fn single() {
        let x: Vec<u32> = avec![42];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 1);
        assert_eq!(x[0], 42);
    }

    #[test]
    fn double() {
        let x: Vec<u32> = avec![42, 43];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 2);
        assert_eq!(x[0], 42);
    }

    #[test]
    fn trailing() {
        let x: Vec<u32> =
            avec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,];
        assert!(!x.is_empty());
    }

    #[test]
    fn clone_2() {
        let x: Vec<u32> = avec![42;2];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 2);
        assert_eq!(x[0], 42);
        assert_eq!(x[1], 42);
    }
}
