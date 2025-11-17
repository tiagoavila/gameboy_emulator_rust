use crate::gameboy_core::{
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    cpu_components,
    ppu_components::{Tile, TilePixelValue},
};

// Tile data is stored in VRAM in the memory area at $8000-$97FF;
pub const TILE_DATA_START: u16 = 0x8000;
pub const TILE_DATA_END: u16 = 0x97FF;

// The Game Boy contains two 32×32 tile maps in VRAM at the memory areas $9800-$9BFF and $9C00-$9FFF.
// Any of these maps can be used to display the Background or the Window.
pub const TILE_MAP_0_START: u16 = 0x9800;
pub const TILE_MAP_0_END: u16 = 0x9BFF;
pub const TILE_MAP_1_START: u16 = 0x9C00;
pub const TILE_MAP_1_END: u16 = 0x9FFF;

pub const LCDC: u16 = 0xFF40; // LCD Control register
pub const BGP: u16 = 0xFF47; // Background palette

pub struct Ppu {
    pub screen: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT], // 144 rows of 160 pixels
}

impl Ppu {
    pub(crate) fn new() -> Self {
        Self {
            screen: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT],
        }
    }

    pub fn render_screen(&self, memory_bus: &mut cpu_components::MemoryBus) {
        self.read_tiles(memory_bus);
    }

    fn read_tiles(&self, memory_bus: &cpu_components::MemoryBus) -> [Tile; 384] {
        let mut tiles: [Tile; 384] = [Tile::new(); 384];
        // Tile data is stored in VRAM in the memory area at $8000-$97FF;
        // Each tile is 16 bytes (2 bytes per row, 8 rows)
        let tile_slice = &memory_bus.get_vram_tile_data();

        // Process tiles in chunks of 16 bytes (one tile per iteration)
        for (tile_index, tile_bytes) in tile_slice.chunks(16).enumerate() {
            // Process each row of the tile (2 bytes per row, 8 rows total)
            let mut tile = Tile::new();

            for (row_index, row_bytes) in tile_bytes.chunks(2).enumerate() {
                if row_bytes.len() == 2 {
                    let low_byte = row_bytes[0];
                    let high_byte = row_bytes[1];

                    for pixel in 0..8 {
                        let bit_pos = 7 - pixel;
                        let lsb = (low_byte >> bit_pos) & 0x01;
                        let msb = (high_byte >> bit_pos) & 0x01;
                        let pixel_value = match (msb << 1) | lsb {
                            0 => TilePixelValue::Zero,
                            1 => TilePixelValue::One,
                            2 => TilePixelValue::Two,
                            3 => TilePixelValue::Three,
                            _ => unreachable!(),
                        };
                        tile.pixels[row_index][pixel] = pixel_value;
                    }

                    tiles[tile_index] = tile;
                }
            }
        }

        // print the first 8 tiles for verification
        for i in 0..8 {
            println!("Tile {}:", i);
            for row in 0..8 {
                for col in 0..8 {
                    let pixel = match tiles[i].pixels[row][col] {
                        TilePixelValue::Zero => '0',
                        TilePixelValue::One => '1',
                        TilePixelValue::Two => '2',
                        TilePixelValue::Three => '3',
                    };
                    print!("{}", pixel);
                }
                println!();
            }
            println!();
        }

        tiles
    }
}
