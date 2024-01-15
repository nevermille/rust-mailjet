/// Logs an info if log feature is enabled
macro_rules! info {
    ($($x:tt)*) => (
        #[cfg(feature = "log")] {
            log::info!($($x)*)
        }
    )
}

pub(crate) use info;
