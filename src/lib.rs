#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod vga_buf;

pub trait Testable {
    fn run(&self) -> ();
}

#[derive(Debug)]
#[repr(u32)] // 4 bytes remember -> iosize
pub enum QemuExitCode {
    Success = 0x10, //  (0x10 << 1) | 1 = 33
    Failed = 0x11,  // (0x11 << 1) | 1 = 35
}

// When a value is written to the I/O port specified by iobase,
// it causes QEMU to exit with exit status (value << 1) | 1.
// So when we write 0 to the port,
// QEMU will exit with exit status (0 << 1) | 1 = 1,
// and when we write 1 to the port,
// it will exit with exit status (1 << 1) | 1 = 3.
pub fn exit_qemu(code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        // 0xf4, is the iobase of the isa-debug-exit device.
        let mut port = Port::new(0xf4);
        port.write(code as u32); // 4 bytes remember!!!
    }
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]");
    serial_println!("{}", info);

    exit_qemu(QemuExitCode::Failed);

    halt_cpu();
}

// Entry point for cargo test
#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    halt_cpu();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
}

// initialise Interrupt Descriptor table
pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe {
        interrupts::PICS.lock().initialize();
    }
    x86_64::instructions::interrupts::enable();
}

pub fn halt_cpu() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
