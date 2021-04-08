use drone_core::{reg, reg::prelude::*};

reg::tokens! {
    #[doc(hidden)]
    pub macro reg_tokens;
    super;
    crate::reg;

    /// Timer.
    pub mod TIMER {
        MTIMECMP_LOW;
        MTIMECMP_HIGH;
        MTIME_LOW;
        MTIME_HIGH;
    }

    /// Reset and clock unit.
    pub mod RCU {
        APB2EN;
    }

    /// General-purpose I/O port C.
    pub mod GPIOC {
        CTL1;
        BOP;
    }
}

reg! {
    /// M-Mode timer (low 32 bits).
    pub TIMER MTIME_LOW => {
        address => 0xD100_0000;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Timer for M-Mode.
            MTIME => { offset => 0; width => 32; traits => { RRRegField WWRegField } };
        };
    };

    /// M-Mode timer (high 32 bits).
    pub TIMER MTIME_HIGH => {
        address => 0xD100_0004;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Timer for M-Mode.
            MTIME => { offset => 0; width => 32; traits => { RRRegField WWRegField } };
        };
    };

    /// M-Mode timer compare (low 32 bits).
    pub TIMER MTIMECMP_LOW => {
        address => 0xD100_0008;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Timer compare for M-Mode.
            MTIMECMP => { offset => 0; width => 32; traits => { RRRegField WWRegField } };
        };
    };

    /// M-Mode timer compare (high 32 bits).
    pub TIMER MTIMECMP_HIGH => {
        address => 0xD100_000C;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// Timer compare for M-Mode.
            MTIMECMP => { offset => 0; width => 32; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// APB2 enable register.
    pub RCU APB2EN => {
        address => 0x4002_1018;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { RReg WReg };
        fields => {
            /// GPIO port C clock enable.
            PCEN => { offset => 4; width => 1; traits => { RRRegField WWRegField } };
        };
    };
}

reg! {
    /// Port C control register 1.
    pub GPIOC CTL1 => {
        address => 0x4001_1004;
        size => 0x20;
        reset => 0x4444_4444;
        traits => { RReg WReg };
        fields => {
            /// Pin 13 configuration bits.
            CTL13 => { offset => 22; width => 2; traits => { RRRegField WWRegField } };
            /// Pin 13 mode bits.
            MD13 => { offset => 20; width => 2; traits => { RRRegField WWRegField } };
        };
    };

    /// Port C operate register.
    pub GPIOC BOP => {
        address => 0x4001_1010;
        size => 0x20;
        reset => 0x0000_0000;
        traits => { WReg WoReg };
        fields => {
            /// Pin 13 clear bit.
            CR13 => { offset => 29; width => 1; traits => { WWRegField WoWRegField } };
            /// Pin 13 set bit.
            BOP13 => { offset => 13; width => 1; traits => { WWRegField WoWRegField } };
        };
    };
}
