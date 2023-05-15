pub mod common;
pub mod experiences;
pub mod skills;
pub mod works;

#[macro_export]
macro_rules! define_asset_loader {
    ($file:expr, $ser_type:ty, $in_type:ty) => {
        define_asset_loader!($file, $ser_type, $in_type, Into::into);
    };
    ($file:expr, $ser_type:ty, $in_type:ty, $transform_fn:expr) => {
        pub use __define_asset_loader::get_data;

        mod __define_asset_loader {
            use super::*;
            use once_cell::sync::Lazy;
            use serde_json;
            use std::rc::Rc;

            const DATA_RAW: &str = include_str!($file);

            thread_local! {
                static DATA: Lazy<Rc<$in_type>> = Lazy::new(|| Rc::new($transform_fn(load_data())));
            }

            fn load_data() -> $ser_type {
                serde_json::from_str(DATA_RAW).expect("cannot parse to json")
            }

            pub fn get_data() -> Rc<$in_type> {
                DATA.with(|v| Rc::clone(v))
            }
        }
    };
}
