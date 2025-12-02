/// The memory can address 65,536 locations, which equals 64 KB.
pub const MEMORY_SIZE: usize = 0x10000; // 65536 in decimal which is 64KB

/// The initial value of the Program Counter (PC) at CPU start-up.
pub const INITIAL_PC: u16 = 0x0100;

/// Screen width in pixels.
pub const SCREEN_WIDTH: usize = 160;

/// Screen height in pixels.
pub const SCREEN_HEIGHT: usize = 144;

/// The size of the map used for background and window rendering in pixels (256x256).
pub const BG_AND_WINDOW_MAP_SCREEN_SIZE: usize = 256;
/// Number of tiles per row and column in the background and window tile map (32x32).
pub const BG_AND_WINDOW_TILE_COUNT_PER_ROW_COL: usize = 32;

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

// Tile data is stored in VRAM in the memory area at $8000-$97FF;
pub const TILE_DATA_START: u16 = 0x8000;
pub const TILE_DATA_END: u16 = 0x97FF;

// There are two possible addressing modes for BG and Window data:
// 1. $8000-$8FFF: Unsigned indices (0 to 255)
// 2. $8800-$97FF: Signed indices (-128 to 127)
// Tiles are always indexed using an 8-bit integer, but the addressing method may differ: 
// The “$8000 method” uses $8000 as its base pointer and uses an unsigned addressing, meaning that tiles 0-127 are in block 0, and tiles 128-255 are in block 1.
// The “$8800 method” uses $9000 as its base pointer and uses a signed addressing, meaning that tiles 0-127 are in block 2, and tiles -128 to -1 are in block 1; or, to put it differently, “$8800 addressing” takes tiles 0-127 from block 2 and tiles 128-255 from block 1.
// (You can notice that block 1 is shared by both addressing methods)
// 
// Objects always use “$8000 addressing”, but the BG and Window can use either mode, controlled by LCDC bit 4.
pub const BG_WINDOW_DATA_AREA_0_START: u16 = 0x8000;
pub const BG_WINDOW_DATA_AREA_0_END: u16 = 0x8FFF;
pub const BG_WINDOW_DATA_AREA_1_START: u16 = 0x8800;
pub const BG_WINDOW_DATA_AREA_1_END: u16 = 0x97FF;

// The Game Boy contains two 32×32 tile maps in VRAM at the memory areas $9800-$9BFF and $9C00-$9FFF.
// Any of these maps can be used to display the Background or the Window.
pub const TILE_MAP_AREA_0_START: u16 = 0x9800;
pub const TILE_MAP_AREA_0_END: u16 = 0x9BFF;
pub const TILE_MAP_AREA_1_START: u16 = 0x9C00;
pub const TILE_MAP_AREA_1_END: u16 = 0x9FFF;

/// LCDC is the main LCD Control register. Its bits toggle what elements are displayed on the screen, and how.
pub const LCDC: u16 = 0xFF40; 

///This register assigns gray shades to the color indices of the BG and Window tiles. 
pub const BGP: u16 = 0xFF47; 

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