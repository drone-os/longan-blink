//! The threads.

pub use drone_riscv::thr::init;

use drone_riscv::thr;

thr::clint! {
    /// Thread-safe storage.
    thread => pub Thr {};

    /// Thread-local storage.
    local => pub ThrLocal {};

    /// Thread token set.
    index => pub Thrs;

    /// Threads initialization token.
    init => pub ThrsInit;

    threads => {
        /// Timer interrupt.
        timer => pub timer;
        software => {
            /// Main thread.
            pub main;
        };
    };

    timer_base => 0xD100_0000;

    plic_base => 0; // PLIC seems to be unimplemented in the Bumblebee Core
}
