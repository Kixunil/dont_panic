//! This crate provides macros that look just like `panic!()` but instead of panicking, they cause a
//! linking error if their calls are not optimized-out. This can be used to ensure the compiler
//! optimizes away some code.
//!
//! # Example
//!
//! ```no_compile
//! #[macro_use]
//! extern crate dont_panic;
//!
//! fn main() {
//! /*
//!     let x = 6 * 9;
//!     if x == 42 {
//!         dont_panic!("6 * 9 == 42");
//!     }
//! */
//! let x = false;
//! if x {
//!     dont_panic!("42");
//! }
//! }
//! ```
//!
//! Compile with `--release` or `--features=panic`

#![no_std]

extern "C" {
    /// This function doesn't actually exist. It ensures a linking error if it isn't optimized-out.
    pub fn rust_panic_called_where_shouldnt() -> !;
}

/// This macro doesn't panic. Instead it tries to call a non-existing function. If the compiler can
/// prove it can't be called and optimizes it away, the code will compile just fine. Otherwise you get
/// a linking error.
///
/// This should be used only in cases you are absolutely sure are OK and optimizable by compiler.
#[cfg(not(feature = "panic"))]
#[macro_export]
macro_rules! dont_panic {
    ($($x:tt)*) => ({
        unsafe { $crate::rust_panic_called_where_shouldnt(); }
    })
}

/// This macro is active only with `panic` feature turned on and it will really panic, instead of
/// causing a linking error. The purpose is to make development easier. (E.g. in debug mode.)
#[cfg(feature = "panic")]
#[macro_export]
macro_rules! dont_panic {
    ($($x:tt)*) => ({
        panic!($($x)*);
    })
}

/// Like assert but calls `dont_panic!()` instead of `panic!()`
#[macro_export]
macro_rules! dp_assert {
    ($cond:expr) => (
        if !$cond {
            dont_panic!(concat!("assertion failed: ", stringify!($cond)))
        }
    );

    ($cond:expr, $($arg:tt)+) => (
        if !$cond {
            dont_panic!($($arg)+)
        }
    );
}

/// This function calls the given closure, asserting that there's no possibility of panicking.
/// If the compiler can't prove this, the code will be left with a `dont_panic!` linking error.
#[cfg(not(feature = "panic"))]
pub fn call<T, F: FnOnce() -> T>(f: F) -> T {
    struct DontPanic;
    impl Drop for DontPanic {
        fn drop(&mut self) {
            dont_panic!();
        }
    }

    let guard = DontPanic;
    let result = f();
    core::mem::forget(guard);
    result
}

/// With the `panic` feature turned on, this function just calls the closure directly, letting
/// it panic or not on its own.
#[cfg(feature = "panic")]
pub fn call<T, F: FnOnce() -> T>(f: F) -> T {
    f()
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

    #[test]
    fn call_slice_index() {
        let foo = [1, 2, 3];
        super::call(|| assert_eq!(foo[0] + foo[1] + foo[2], 6));
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

    #[cfg(feature = "panic")]
    #[test]
    #[should_panic]
    fn call_slice_index_panic() {
        let foo = [1, 2, 3];
        super::call(|| assert_eq!(foo[1] + foo[2] + foo[3], 6));
    }
}
