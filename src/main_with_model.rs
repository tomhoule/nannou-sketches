#[macro_export]
macro_rules! main {
    ($model:ty) => {
        fn main() -> Result<(), failure::Error> {
            nannou::app(<$model>::new)
                .view(<$model>::view)
                .update(<$model>::update)
                .run();

            Ok(())
        }
    };
}
