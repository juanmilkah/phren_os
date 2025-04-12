#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(phren_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use phren_os::{halt_cpu, init, println};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init();

    println!("Hello from {}", "PhrenOS");

    #[cfg(test)]
    test_main();
    halt_cpu()
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    halt_cpu()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    phren_os::test_panic_handler(info);
}
