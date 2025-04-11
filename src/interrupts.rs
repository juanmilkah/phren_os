use crate::println;

use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

// A Breakpoint exception occurs at the execution of the INT3 instruction.
// Some debug software replace an instruction by the INT3 instruction.
// When the breakpoint is trapped,
// it replaces the INT3 instruction with the original instruction,
// and decrements the instruction pointer by one.
// The saved instruction pointer points to the byte after the INT3 instruction.
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

#[test_case]
fn t_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
