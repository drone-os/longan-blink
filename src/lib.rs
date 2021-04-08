#![feature(allocator_api)]
#![feature(asm)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(never_type)]
#![feature(prelude_import)]
#![feature(proc_macro_hygiene)]
#![feature(slice_ptr_get)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod mtime;
pub mod reg;
pub mod tasks;
pub mod thr;

#[prelude_import]
#[allow(unused_imports)]
use drone_core::prelude::*;

use drone_core::heap;

reg_tokens! {
    /// A set of tokens for all memory-mapped registers.
    index => pub Regs;
}

heap! {
    // Heap configuration key in `Drone.toml`.
    config => main;
    /// The main heap allocator generated from the `Drone.toml`.
    metadata => pub Heap;
    // Use this heap as the global allocator.
    global => true;
    // Uncomment the following line to enable heap tracing feature:
    // trace_port => 31;
}

/// The global allocator.
#[cfg_attr(not(feature = "std"), global_allocator)]
pub static HEAP: Heap = Heap::new();
