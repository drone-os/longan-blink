//! The root task.

use crate::{
    mtime::Mtime,
    reg::{gpioc, rcu},
    thr,
    thr::ThrsInit,
    Regs,
};
use drone_core::fib;
use drone_riscv::{reg::prelude::*, thr::prelude::*};
use futures::prelude::*;

/// An error returned when a receiver has missed too many ticks.
#[derive(Debug)]
pub struct TickOverflow;

/// The root task handler.
#[inline(never)]
pub fn handler(reg: Regs, thr_init: ThrsInit) {
    let thr = thr::init(thr_init);

    let Regs {
        timer_mtime_low,
        timer_mtime_high,
        timer_mtimecmp_low,
        timer_mtimecmp_high,
        rcu_apb2en,
        gpioc_ctl1,
        gpioc_bop,
        ..
    } = reg;

    let timer = Mtime {
        timer_mtime_low,
        timer_mtime_high,
        timer_mtimecmp_low,
        timer_mtimecmp_high,
    };

    thr.main.exec(async move {
        beacon(timer, rcu_apb2en, gpioc_ctl1, gpioc_bop, thr.timer)
            .await
            .expect("beacon fail");
    });
}

async fn beacon(
    timer: Mtime,
    rcu_apb2en: rcu::Apb2En<Srt>,
    gpioc_ctl1: gpioc::Ctl1<Srt>,
    gpioc_bop: gpioc::Bop<Srt>,
    thr_timer: thr::Timer,
) -> Result<(), TickOverflow> {
    rcu_apb2en.store(|r| r.set_pcen()); // GPIO port C clock enable
    gpioc_ctl1.store(|r| {
        r.write_md13(0b11) // Output mode, max speed 2 MHz
            .write_ctl13(0b00) // General purpose output push-pull
    });

    // Attach a listener that will notify us on each interrupt trigger.
    let mut tick_stream = thr_timer.add_pulse_try_stream(
        // This closure will be called when a receiver no longer can store the
        // number of ticks since the last stream poll. If this happens, a
        // `TickOverflow` error will be sent over the stream as is final value.
        || Err(TickOverflow),
        // A fiber that will be called on each interrupt trigger. It sends a
        // single tick over the stream.
        fib::new_fn(|| fib::Yielded(Some(1))),
    );

    // Enable the timer interrupt.
    timer.enable();
    // Set the first tick.
    timer.set(100000);

    // A value cycling from 0 to 7.
    let mut counter = 0;
    while let Some(tick) = tick_stream.next().await {
        for _ in 0..tick?.get() {
            match counter {
                0 | 2 => {
                    gpioc_bop.store(|r| r.set_cr13());
                }
                _ => {
                    gpioc_bop.store(|r| r.set_bop13());
                }
            }
            counter = (counter + 1) % 8;
            // Set the next tick.
            timer.set(100000);
        }
    }

    Ok(())
}
