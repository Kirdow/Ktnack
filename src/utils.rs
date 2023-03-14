pub const IS_DEBUG: bool = false;

use std::fs;

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

pub fn file_exists(file_path: &String) -> bool {
    if let Ok(metadata) = fs::metadata(file_path) {
        metadata.is_file()
    } else {
        false
    }
}
