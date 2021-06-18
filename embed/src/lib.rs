#![cfg_attr(not(test), no_std)]

use core::panic::PanicInfo;

/// Macro defines the entry point for the program and must be invoked from the
/// root.
///
/// # Usage
/// ```rust
/// # use embed::entry;
/// #
/// # entry!(main);
/// #
/// # pub fn main() -> ! {
/// #     loop {}
/// # }
/// #
/// ```
///
#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> ! {
            // type check the given path
            let f: fn() -> ! = $path;

            f()
        }
    };
}

/// # Introduction
///
/// This function in invoked on the reset of the microcontroller. The function
/// performs some initialization of the hardware and then invokes the main function
/// defined in the binary.
/// Please note: the main function is not the same as regular main function and.
///
/// # Safety
///
/// The function invokes an externally defined rust function. Since the compile
/// would already be doing safety check of the function, invoking it should be
/// safe.
#[no_mangle]
pub unsafe extern "C" fn OnReset() -> ! {
    extern "Rust" {
        fn main() -> !;
    }

    main()
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = OnReset;
