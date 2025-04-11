#![feature(custom_test_frameworks)]
#![test_runner(phren_os::test_runner)]
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod serial;
mod vga_buf;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello from {}", "PhrenOS");

    #[cfg(test)]
    test_main();
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use phren_os::test_panic_handler;

    test_panic_handler(info);
}
