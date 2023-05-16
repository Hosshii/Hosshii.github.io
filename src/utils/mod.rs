pub use unique::*;

mod unique {
    use std::{cell::Cell, fmt::Display};

    struct Unique(u32);
    impl Unique {
        pub fn new() -> Self {
            thread_local! {
                static COUNTER: Cell<u32> = Cell::new(0);
            }
            let v = COUNTER.with(|v| {
                let r = v.get();
                v.set(r + 1);
                r
            });
            Self(v)
        }
    }

    impl Display for Unique {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "id_uniq_{}", self.0)
        }
    }

    pub fn unique() -> String {
        Unique::new().to_string()
    }
}
