use crate::gameboy_core::{
    constants::{BG_AND_WINDOW_MAP_SCREEN_SIZE, BG_AND_WINDOW_TILE_COUNT_PER_ROW_COL, SCREEN_HEIGHT, SCREEN_WIDTH},
    cpu_components,
    ppu_components::{self, Tile, TilePixelValue},
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
    
    pub fn update_screen(&mut self, memory_bus: &cpu_components::MemoryBus) {
        self.screen = self.get_screen_buffer(memory_bus);
    }

    /// Generates the screen buffer representing the visible 160x144 pixel screen.
    /// This will build the Background first, then apply the Window (if enabled), and finally render the Objects - Sprites (if enabled).
    pub fn get_screen_buffer(&self, memory_bus: &cpu_components::MemoryBus) -> [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT] {
        let tiles = self.read_tiles(memory_bus);
        let lcdc_register = ppu_components::LcdcRegister::get_lcdc_register(memory_bus);

        //bg setup
        let bg_buffer = self.get_bg_buffer(memory_bus, &tiles, &lcdc_register);
        let screen_buffer = self.get_visible_bg_buffer(&bg_buffer, memory_bus);

        screen_buffer
    }
    
    /// Returns the visible portion of the background buffer based on the SCX and SCY scroll values and to fit the 160x144 screen.
    /// The PPU calculates the bottom-right coordinates of the viewport with those formulas:
    /// bottom := (SCY + 143) % 256 and right := (SCX + 159) % 256.
    /// As suggested by the modulo operations, in case the values are larger than 255 they will “wrap around” towards the top-left corner of the tilemap.
    fn get_visible_bg_buffer(&self, bg_buffer: &[[u8; BG_AND_WINDOW_MAP_SCREEN_SIZE]; BG_AND_WINDOW_MAP_SCREEN_SIZE], memory_bus: &cpu_components::MemoryBus) -> [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT] {
        let scy = memory_bus.get_scy_register() as usize;
        let scx = memory_bus.get_scx_register() as usize;
        let mut visible_bg_buffer = [[0u8; SCREEN_WIDTH]; SCREEN_HEIGHT];

        for screen_row in 0..SCREEN_HEIGHT {
            for screen_col in 0..SCREEN_WIDTH {
                let bg_row = (scy + screen_row) % BG_AND_WINDOW_MAP_SCREEN_SIZE;
                let bg_col = (scx + screen_col) % BG_AND_WINDOW_MAP_SCREEN_SIZE;
                visible_bg_buffer[screen_row][screen_col] = bg_buffer[bg_row][bg_col];
            }
        }

        visible_bg_buffer
    }

    /// Generates the background buffer representing the entire 256x256 pixel background.
    /// This buffer is constructed by reading the tile data and the background tile map from VRAM, parses the background tile map 
    /// into a 32x32 grid, and then mapping each tile's pixels into the correct positions in the 256x256 buffer.
    pub fn get_bg_buffer(&self, memory_bus: &cpu_components::MemoryBus, tiles: &[Tile; 384], lcdc_register: &ppu_components::LcdcRegister) -> [[u8; BG_AND_WINDOW_MAP_SCREEN_SIZE]; BG_AND_WINDOW_MAP_SCREEN_SIZE] {
        let bg_tile_map = self.get_bg_tile_map_as_grid_32x32(memory_bus, &lcdc_register);
        let mut bg_buffer = [[0u8; BG_AND_WINDOW_MAP_SCREEN_SIZE]; BG_AND_WINDOW_MAP_SCREEN_SIZE];
    
        // print background tile map to console
        for tile_map_row in 0..BG_AND_WINDOW_TILE_COUNT_PER_ROW_COL {
            for tile_map_col in 0..BG_AND_WINDOW_TILE_COUNT_PER_ROW_COL {
                let tile_index = bg_tile_map[tile_map_row][tile_map_col] as usize;
                let tile = &tiles[tile_index];
    
                for tile_row in 0..8 {
                    for tile_col in 0..8 {
                        let pixel_value = match tile.pixels[tile_row][tile_col] {
                            TilePixelValue::Zero => 0,
                            TilePixelValue::One => 1,
                            TilePixelValue::Two => 2,
                            TilePixelValue::Three => 3,
                        };
                        let buffer_row = tile_map_row * 8 + tile_row;
                        let buffer_col = tile_map_col * 8 + tile_col;
                        bg_buffer[buffer_row][buffer_col] = pixel_value;
                    }
                }
            }
        }
        bg_buffer
    }
    
    /// Get the Tiles from VRAM. Tiles are used to build the background, window, and objects (sprites).
    /// A tile (or character) has 8×8 pixels and has a color depth of 2 bits per pixel,
    /// allowing each pixel to use one of 4 colors or gray shades.
    /// Tiles can be displayed as part of the Background/Window maps, and/or as objects (movable sprites).
    pub fn read_tiles(&self, memory_bus: &cpu_components::MemoryBus) -> [Tile; 384] {
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
        // for i in 0..8 {
        //     println!("Tile {}:", i);
        //     for row in 0..8 {
        //         for col in 0..8 {
        //             let pixel = match tiles[i].pixels[row][col] {
        //                 TilePixelValue::Zero => '0',
        //                 TilePixelValue::One => '1',
        //                 TilePixelValue::Two => '2',
        //                 TilePixelValue::Three => '3',
        //             };
        //             print!("{}", pixel);
        //         }
        //         println!();
        //     }
        //     println!();
        // }

        tiles
    }

    /// Converts the background tile map from a flat vector to a 32x32 grid.
    /// To accomplish this, it reads the tile map from memory and then parses to a 2D array by calculating row and column indices.
    fn get_bg_tile_map_as_grid_32x32(
        &self,
        memory_bus: &cpu_components::MemoryBus,
        lcdc: &ppu_components::LcdcRegister,
    ) -> [[u8; 32]; 32] {
        let tile_map_vec = memory_bus.get_bg_tile_map(lcdc).to_vec();
        let mut tile_map_grid = [[0u8; 32]; 32];
        for (i, &value) in tile_map_vec.iter().enumerate() {
            let row = i / 32;
            let col = i % 32;
            tile_map_grid[row][col] = value;
        }
        tile_map_grid
    }
}
