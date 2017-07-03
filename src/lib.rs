#![no_std]

extern "C" {
    pub fn rust_panic_called_where_shouldnt() -> !;
}

#[cfg(not(feature = "panic"))]
#[macro_export]
macro_rules! dont_panic {
    () => ({
        unsafe { $crate::rust_panic_called_where_shouldnt(); }
    });
    ($msg:expr) => ({
        unsafe { $crate::rust_panic_called_where_shouldnt(); }
    });
    ($fmt:expr, $($arg:tt)+) => ({
        unsafe { $crate::rust_panic_called_where_shouldnt(); }
    });
}

#[cfg(feature = "panic")]
#[macro_export]
macro_rules! dont_panic {
    () => ({
        panic!();
    });
    ($msg:expr) => ({
        panic!($msg);
    });
    ($fmt:expr, $($arg:tt)+) => ({
        panic!($fmt, $($arg)+);
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let should_panic = false;
        if should_panic {
            dont_panic!();
        }
    }

    #[cfg(feature = "panic")]
    #[test]
    #[should_panic]
    fn panic() {
        let should_panic = true;
        if should_panic {
            dont_panic!();
        }
    }

    #[cfg(feature = "panic")]
    #[test]
    fn no_panic() {
        let should_panic = false;
        if should_panic {
            dont_panic!();
        }
    }
}
