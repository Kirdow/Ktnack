pub const IS_DEBUG: bool = false;

#[macro_export]
macro_rules! debugln {
    () => {
        if IS_DEBUG {
            println!()
        }
    };
    ($($arg:tt)*) => {{
        if IS_DEBUG {
            println!($($arg)*);
        }
    }};
}
