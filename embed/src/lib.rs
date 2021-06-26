#![no_std]

use core::panic::PanicInfo;
use core::ptr;

mod arch;

/// Macro defines the entry point for the program and must be invoked from the
/// root.
///
/// # Usage
/// ```rust
/// # #![no_main]
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
pub unsafe extern "C" fn Reset() -> ! {
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;

        static mut _sdata: u8;
        static mut _edata: u8;
        static _sidata: u8;
    }

    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

    let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
    ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);

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
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;
