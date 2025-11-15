/// The maximum address is 65,535 (decimal) = 0xFFFF (hexadecimal).
/// The memory can address 65,536 locations, which equals 64 KB.
pub const MEMORY_SIZE: usize = 0xFFFF + 1; // 0xFFFF - u16::MAX

/// The initial value of the Program Counter (PC) at CPU start-up.
pub const INITIAL_PC: u16 = 0x0100;

/// Screen width in pixels.
pub const SCREEN_WIDTH: usize = 160;

/// Screen height in pixels.
pub const SCREEN_HEIGHT: usize = 144;

/// Start address for load instructions involving I/O ports.
pub const START_ADDRESS_FOR_LOAD_INSTRUCTIONS: u16 = 0xFF00;

/// 8-Bit register codes used in instruction encoding.
pub const EIGHT_BIT_REGISTERS: [u8; 7] = [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b111];

/// 16-Bit register codes used in instruction encoding.
pub const SIXTEEN_BIT_REGISTERS: [u8; 4] = [0b00, 0b01, 0b10, 0b11];

/// Start of the Video RAM (VRAM) region in the Gameboy memory map.
pub const VRAM_START: u16 = 0x8000;

/// End of the Video RAM (VRAM) region in the Gameboy memory map.
pub const VRAM_END: u16 = 0x9FFF;

/// Start of the Object Attribute Memory (OAM) region in the Gameboy memory map.i
pub const OAM_START: u16 = 0xFE00;

/// End of the Object Attribute Memory (OAM) region in the Gameboy memory map.
pub const OAM_END: u16 = 0xFE9F;
