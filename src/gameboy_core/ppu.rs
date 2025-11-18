use crate::gameboy_core::{
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH}, cpu_components, ppu, ppu_components::{self, Tile, TilePixelValue}
};

pub struct Ppu {
    pub screen: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT], // 144 rows of 160 pixels
}

impl Ppu {
    pub(crate) fn new() -> Self {
        Self {
            screen: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT],
        }
    }

    pub fn render_screen(&self, memory_bus: &cpu_components::MemoryBus) {
        let tiles = self.read_tiles(memory_bus);
        let lcdc_register = ppu_components::LcdcRegister::get_lcdc_register(memory_bus);
        self.read_tile_map(memory_bus, &lcdc_register);
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
    
    fn read_tile_map(&self, memory_bus: &cpu_components::MemoryBus, lcdc: &ppu_components::LcdcRegister) -> [u8; 1024] {
        [0; 1024]
    }
    
    pub fn get_lcdc_register(&self) {
        
    }
}
