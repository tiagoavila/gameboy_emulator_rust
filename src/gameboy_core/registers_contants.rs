/// The IE flag is used to control interrupts.
pub const IE: u16 = 0xFFFF;

/// The IF flag is used to indicate which interrupts could be requested.
pub const IF: u16 = 0xFF0F;

/// Divider Register (DIV) 
pub const DIV: u16 = 0xFF04;

/// Timer Counter (TIMA)
pub const TIMA: u16 = 0xFF05;

/// Timer Modulo (TMA)
pub const TMA: u16 = 0xFF06;

/// Timer Control (TAC)
pub const TAC: u16 = 0xFF07;
