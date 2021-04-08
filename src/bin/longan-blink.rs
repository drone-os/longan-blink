#![warn(unsafe_op_in_unsafe_fn)]
#![no_main]
#![no_std]

use drone_core::{mem, token::Token};
use drone_riscv::processor;
use longan_blink::{tasks, thr::ThrsInit, Regs};

/// The entry point.
///
/// # Safety
///
/// This function should only be called by hardware.
#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    // Set the processor stack pointer to the top of the stack region. This is
    // safe because the stack hasn't been in use before this line.
    unsafe { processor::stack_pointer_init() };
    // Fill the memory region allocated for initially zeroed mutable static
    // variables with zeros. This is safe because none of these variables have
    // been in use before this line.
    unsafe { mem::bss_init() };
    // Fill the memory region for other mutable static variables with initial
    // values from flash memory. This is safe because none of these variables
    // have been in use before this line.
    unsafe { mem::data_init() };
    // Run the root task.
    tasks::root(
        // Instantiate a zero-sized collection of tokens for memory-mapped
        // registers. Safe only if this is the only instance.
        unsafe { Regs::take() },
        // Instantiate a zero-sized token needed to initialize the threading
        // system later. Safe only if this is the only instance.
        unsafe { ThrsInit::take() },
    );
    // If the root task returned, always sleep between interrupts.
    loop {
        processor::wait_for_int();
    }
}
