use crate::{
    TILE_SIZE,
    gameboy_core::constants::{COLORS, GAME_SECTION_HEIGHT},
};
use minifb::{Window, WindowOptions};

use crate::gameboy_core::{
    self,
    constants::{GAME_SECTION_WIDTH, SCREEN_SCALE},
    ppu_components::Tile,
};

const TILES_PER_ROW: usize = 16; // 16 tiles wide
const TILES_PER_COL: usize = 24; // 24 tiles tall
const MARGIN: usize = 20; // Margin between game screen and tile data
const TILE_MARGIN: usize = 2; // Margin between each tile in the tile data section
const TILE_DATA_WIDTH: usize =
    TILES_PER_ROW * TILE_SIZE * SCREEN_SCALE + (TILES_PER_ROW - 1) * TILE_MARGIN * SCREEN_SCALE;
const TILE_DATA_HEIGHT: usize =
    TILES_PER_COL * TILE_SIZE * SCREEN_SCALE + (TILES_PER_COL - 1) * TILE_MARGIN * SCREEN_SCALE;

pub(crate) const TOTAL_WINDOW_WIDTH: usize =
    (GAME_SECTION_WIDTH * SCREEN_SCALE) + MARGIN + TILE_DATA_WIDTH;
pub(crate) const TOTAL_WINDOW_HEIGHT: usize = TILE_DATA_HEIGHT;

pub struct Screen {
    pub window: Window,
    pub buffer: Box<[[u32; TOTAL_WINDOW_WIDTH]; TOTAL_WINDOW_HEIGHT]>,
}

impl Screen {
    pub fn new(title: String) -> Result<Screen, minifb::Error> {
        let window: Window = Self::create_screen(title)?;

        Ok(Self {
            window,
            buffer: Box::new([[0x000080; TOTAL_WINDOW_WIDTH]; TOTAL_WINDOW_HEIGHT]),
        })
    }

    /// Updates the minifb window with the current buffer content.
    pub fn update_window_with_buffer(&mut self) {
        let buffer: Vec<u32> = self.parse_2d_vector_to_1d();
        self.window
            .update_with_buffer(&buffer, TOTAL_WINDOW_WIDTH, TOTAL_WINDOW_HEIGHT)
            .unwrap();
    }

    /// Render the current Game to the screen buffer with scaling applied.
    pub fn render_game_to_screen_buffer(&mut self, cpu: &gameboy_core::cpu::Cpu) {
        let lcdc = cpu.memory_bus.get_lcdc_register();
        println!("Ppu is enabled: {}", lcdc & 0b1000_0000 != 0);
        println!("Window is enabled: {}", lcdc & 0b0010_0000 != 0);
        println!("Objects are enabled: {}", lcdc & 0b0000_0010 != 0);
        println!("Bg & Window are enabled/priority: {}", lcdc & 0b0000_0001 != 0);
        println!("===============================");
        for row in 0..GAME_SECTION_HEIGHT {
            for col in 0..GAME_SECTION_WIDTH {
                // Apply scaling
                for scale_row in 0..SCREEN_SCALE {
                    for scale_col in 0..SCREEN_SCALE {
                        let buffer_row = row * SCREEN_SCALE + scale_row;
                        let buffer_col = col * SCREEN_SCALE + scale_col;

                        if buffer_row < TOTAL_WINDOW_HEIGHT && buffer_col < TOTAL_WINDOW_WIDTH {
                            self.buffer[buffer_row][buffer_col] = cpu.ppu.screen[row][col];
                        }
                    }
                }
            }
        }

        println!("Rendering frame to window.");
    }

    /// Renders tile data to the screen buffer for visualization and debugging purposes.
    /// Takes all 384 tiles from memory and arranges them in a grid (16 tiles wide × 24 tiles tall).
    /// Each tile is 8×8 pixels and rendered with the Game Boy color palette.
    pub fn render_tile_data_to_screen_buffer(&mut self, cpu: &gameboy_core::cpu::Cpu) {
        let tiles: [Tile; 384] = cpu.ppu.get_tiles_data(&cpu.memory_bus);

        // Starting position for tile data (next to the game screen with margin)
        let start_col_offset = GAME_SECTION_WIDTH * SCREEN_SCALE + MARGIN;

        for tile_index in 0..384 {
            // Calculate the grid position of this tile (16 tiles per row)
            let grid_row = tile_index / TILES_PER_ROW;
            let grid_col = tile_index % TILES_PER_ROW;

            let tile = &tiles[tile_index];
            let pixels_block = Screen::parse_tile_to_8x8_pixels_block_color(tile);

            // Render each pixel of the tile
            for tile_row in 0..TILE_SIZE {
                for tile_col in 0..TILE_SIZE {
                    let color = pixels_block[tile_row][tile_col];

                    // Calculate the screen position with margins between tiles
                    let screen_row = grid_row * (TILE_SIZE + TILE_MARGIN);
                    let screen_col = grid_col * (TILE_SIZE + TILE_MARGIN);

                    // Apply scaling and offset
                    for scale_row in 0..SCREEN_SCALE {
                        for scale_col in 0..SCREEN_SCALE {
                            let buffer_row =
                                screen_row * SCREEN_SCALE + tile_row * SCREEN_SCALE + scale_row;
                            let buffer_col = start_col_offset
                                + screen_col * SCREEN_SCALE
                                + tile_col * SCREEN_SCALE
                                + scale_col;

                            if buffer_row < TOTAL_WINDOW_HEIGHT && buffer_col < TOTAL_WINDOW_WIDTH {
                                self.buffer[buffer_row][buffer_col] = color;
                            }
                        }
                    }
                }
            }
        }
    }

    /// Parses the 2D buffer into a 1D vector for minifb window update.
    fn parse_2d_vector_to_1d(&mut self) -> Vec<u32> {
        self.buffer
            .iter()
            .flat_map(|row| row.iter())
            .cloned()
            .collect::<Vec<u32>>()
    }

    /// Creates a new window for the Gameboy emulator screen using minifb.
    fn create_screen(title: String) -> Result<Window, minifb::Error> {
        Window::new(
            &title,
            TOTAL_WINDOW_WIDTH,
            TOTAL_WINDOW_HEIGHT,
            WindowOptions::default(),
        )
    }

    /// Parses a Tile into an 8x8 block of u32 pixels, where the pixel value is then parsed to an actual color from the COLORS palette array.
    fn parse_tile_to_8x8_pixels_block_color(tile: &Tile) -> [[u32; 8]; 8] {
        let mut pixels_block = [[0u32; 8]; 8];
        for row in 0..TILE_SIZE {
            for col in 0..TILE_SIZE {
                let pixel_value = tile.pixels[row][col];
                let color = COLORS[pixel_value as usize];

                pixels_block[row][col] = color;
            }
        }

        pixels_block
    }
}
