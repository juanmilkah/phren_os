#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

use lazy_static::lazy_static;
use phren_os::{QemuExitCode, exit_qemu, gdt, serial_print, serial_println};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(t_double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

extern "x86-interrupt" fn t_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn t_init_idt() {
    TEST_IDT.load();
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();

    volatile::Volatile::new(0).read(); // prevent tail optimizations
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    phren_os::test_panic_handler(info);
}

#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    gdt::init();
    t_init_idt();

    serial_print!("stack_overflow::stack_overflow...\t");
    stack_overflow();
    panic!("Process not exited!");
}
