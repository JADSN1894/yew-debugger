#[macro_export]
macro_rules! impl_name {
    ($struct_name:ty) => {
        impl $crate::traits::Name for $struct_name {
            const NAME: &'static str = stringify!($struct_name);
        }
    };
}
