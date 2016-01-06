struct Return<T>(T);

impl<T> Into<Option<T>> for Return<T> {
    fn into(self) -> Option<T> {
        Some(self.0)
    }
}

impl<T, E> Into<Result<T, E>> for Return<T> {
    fn into(self) -> Result<T, E> {
        Ok(self.0)
    }
}

impl<T> Into<()> for Return<T> {
    fn into(self) {}
}

macro_rules! do_it {
    () => { return Return(()).into(); };
    (return $var:ident) => { return $crate::Return($var).into(); };
    ($opt:expr; $($rest:tt)*) => {
        $opt.and_then(|_| {
            do_it!($($rest)*)
        })
    };
    ($var:ident <- $opt:expr; $($rest:tt)*) => {
        $opt.and_then(|$var| {
            do_it!($($rest)*)
        })
    };
}

#[test]
fn it_works() {
    let a = do_it! {
        a <- Some(0usize);
        a <- Some(1usize);
        return a
    };

    assert_eq!(a, Some(1));
}

#[test]
fn it_some_call() {
    let mut called = false;

    let option: Option<()> = Some(());

    {
        let mut foo = || {
            called = true;
            Some(())
        };

        do_it! {
            option;
            foo();
        };
    }

    assert!(called);
}

#[test]
fn it_none_not_call() {
    let mut called = false;

    let option: Option<()> = None;

    {
        let mut foo = || {
            called = true;
            Some(())
        };

        do_it! {
            option;
            foo();
        };
    }

    assert!(!called);
}
