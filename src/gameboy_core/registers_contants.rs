/// P1 is the joypad input register. It is used to read the state of the buttons and directional pad.
pub const P1: u16 = 0xFF00;

/// SB is the serial transfer data register (read/write).
pub const SB: u16 = 0xFF01;

/// SC is the serial transfer control register (read/write).
pub const SC: u16 = 0xFF02;

/// Divider Register (DIV) 
pub const DIV: u16 = 0xFF04;

/// Timer Counter (TIMA)
pub const TIMA: u16 = 0xFF05;

/// Timer Modulo (TMA)
pub const TMA: u16 = 0xFF06;

/// Timer Control (TAC)
pub const TAC: u16 = 0xFF07;

/// The IF flag is used to indicate which interrupts could be requested.
pub const IF: u16 = 0xFF0F;

/// NR10 to NR52 are sound registers used to control the Gameboy's audio hardware.
pub const NR10: u16 = 0xFF10;
pub const NR11: u16 = 0xFF11;
pub const NR12: u16 = 0xFF12;
pub const NR13: u16 = 0xFF13;
pub const NR14: u16 = 0xFF14;
pub const NR21: u16 = 0xFF16;
pub const NR22: u16 = 0xFF17;
pub const NR23: u16 = 0xFF18;
pub const NR24: u16 = 0xFF19;
pub const NR30: u16 = 0xFF1A;
pub const NR31: u16 = 0xFF1B;
pub const NR32: u16 = 0xFF1C;
pub const NR33: u16 = 0xFF1D;
pub const NR34: u16 = 0xFF1E;
pub const NR41: u16 = 0xFF20;
pub const NR42: u16 = 0xFF21;
pub const NR43: u16 = 0xFF22;
pub const NR44: u16 = 0xFF23;
pub const NR50: u16 = 0xFF24;
pub const NR51: u16 = 0xFF25;
pub const NR52: u16 = 0xFF26;

/// LCDC is the main LCD Control register. Its bits toggle what elements are displayed on the screen, and how.
pub const LCDC: u16 = 0xFF40; 

/// STAT is the LCD Status register. It provides information about the current state of the LCD controller 
/// and can also generate interrupts based on certain conditions.
pub const STAT: u16 = 0xFF41;

/// SCY specifies the vertical scroll position of the background.
/// The value ranges from 0 to 255, where 0 means no vertical scrolling,
/// and 255 means the background is scrolled up by 255 pixels.
pub const SCY: u16 = 0xFF42;

/// SCX specifies the horizontal scroll position of the background.
/// The value ranges from 0 to 255, where 0 means no horizontal scrolling,
/// and 255 means the background is scrolled left by 255 pixels.
pub const SCX: u16 = 0xFF43;

/// LY indicates the current horizontal line, which might be about to be drawn, being drawn, or just been drawn.
/// LY can hold any value from 0 to 153, with values from 144 to 153 indicating the VBlank period. 
pub const LY: u16 = 0xFF44;

/// The Game Boy constantly compares the value of the LYC and LY registers. When both values are identical,
/// the “LYC=LY” flag in the STAT register is set, and (if enabled) a STAT interrupt is requested.;
pub const LYC: u16 = 0xFF45;

/// DMA is used to initiate a DMA transfer from memory to OAM (Object Attribute Memory) for sprite data.
pub const DMA: u16 = 0xFF46;

///This register assigns gray shades to the color indices of the BG and Window tiles. 
pub const BGP: u16 = 0xFF47; 

/// These registers assign gray shades to the color indices of the sprites.
pub const OBP0: u16 = 0xFF48;
pub const OBP1: u16 = 0xFF49;

/// WY specifies the Y position of the window on the screen.
pub const WY: u16 = 0xFF4A;

/// WX specifies the X position of the window on the screen.
pub const WX: u16 = 0xFF4B;

/// The IE flag is used to control interrupts.
pub const IE: u16 = 0xFFFF;


