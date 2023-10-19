#[macro_export]
macro_rules! impl_yew_debugger {
    ($model:ty,$message:ty) => {
        use yew_debugger::YewComponentDebug;
        impl YewComponentDebug<$model, $message> for $model {}
    };
}
