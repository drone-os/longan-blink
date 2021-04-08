use crate::reg::timer;
use drone_riscv::reg::prelude::*;

/// Machine mode timer.
pub struct Mtime {
    pub timer_mtime_low: timer::MtimeLow<Srt>,
    pub timer_mtime_high: timer::MtimeHigh<Srt>,
    pub timer_mtimecmp_low: timer::MtimecmpLow<Srt>,
    pub timer_mtimecmp_high: timer::MtimecmpHigh<Srt>,
}

impl Mtime {
    /// Enables the timer interrupt.
    pub fn enable(&self) {
        unsafe {
            asm!(
                "csrr {0}, mie",
                "ori {0}, {0}, 128",
                "csrw mie, {0}",
                "csrr {0}, mstatus",
                "ori {0}, {0}, 8",
                "csrw mstatus, {0}",
                out(reg) _,
            );
        }
    }

    /// Sets the timer.
    pub fn set(&self, delay: u64) {
        let mut mtime = loop {
            let high = self.timer_mtime_high.load().mtime();
            let low = self.timer_mtime_low.load().mtime();
            if high == self.timer_mtime_high.load().mtime() {
                break ((high as u64) << 32) | low as u64;
            }
        };
        mtime += delay;
        self.timer_mtimecmp_low
            .store(|r| r.write_mtimecmp(0xFFFF_FFFF));
        self.timer_mtimecmp_high
            .store(|r| r.write_mtimecmp((mtime >> 32) as u32));
        self.timer_mtimecmp_low
            .store(|r| r.write_mtimecmp(mtime as u32));
    }
}
