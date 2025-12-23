use crate::gameboy_core::{
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    cpu, cpu_utils,
    ppu_components::{Tile, TilePixelValue},
};
use minifb::{Key, Window, WindowOptions};

pub mod gameboy_core;

const SCREEN_SCALE: usize = 3;
const TILES_PER_ROW: usize = 16; // 16 tiles wide
const TILES_PER_COL: usize = 24; // 24 tiles tall
const TILE_SIZE: usize = 8; // 8x8 pixels per tile
const MARGIN: usize = 20; // Margin between game screen and tile data
const TILE_MARGIN: usize = 2; // Margin between each tile in the tile data section
const TILE_DATA_WIDTH: usize = TILES_PER_ROW * TILE_SIZE * SCREEN_SCALE + (TILES_PER_ROW - 1) * TILE_MARGIN * SCREEN_SCALE;
const TILE_DATA_HEIGHT: usize = TILES_PER_COL * TILE_SIZE * SCREEN_SCALE + (TILES_PER_COL - 1) * TILE_MARGIN * SCREEN_SCALE;
const TOTAL_WINDOW_WIDTH: usize = (SCREEN_WIDTH * SCREEN_SCALE) + MARGIN + TILE_DATA_WIDTH;
const TOTAL_WINDOW_HEIGHT: usize = SCREEN_HEIGHT * SCREEN_SCALE;

fn main() {
    // let rom_file = "games/Tetris.gb";
    let rom_file = "games/Super Mario Land.gb";

    // let rom_file = "tests/nintendo_logo.gb";

    let rom_binary = cpu_utils::read_rom(format!("files/roms/{}", rom_file).as_str()).unwrap();

    // clear previous logs
    cpu_utils::clear_logs().unwrap();
    cpu_utils::clear_dr_gameboy_log().unwrap();

    let mut cpu = gameboy_core::cpu::Cpu::start(rom_binary, false);

    // Create a tile with the letter "T" and render it to the buffer
    // let tile_t = create_tile_of_colored_square();
    // render_tile_to_buffer(&tile_t, &mut buffer, 0, 0);

    // Run the event loop
    run_gameboy(&mut cpu);
}

/// Creates a tile with a colored square: outer border, middle frame, and inner square
fn create_tile_of_colored_square() -> Tile {
    let mut tile = Tile::new();

    // Color scheme:
    // Outside (empty): TilePixelValue::Zero (white)
    // Border: TilePixelValue::Three (black)
    // Middle frame: TilePixelValue::Two (dark gray)
    // Inner square: TilePixelValue::One (light gray)

    for row in 0..8 {
        for col in 0..8 {
            let pixel = if row == 0 || row == 7 || col == 0 || col == 7 {
                // Border (outermost)
                TilePixelValue::Three
            } else if row == 1 || row == 6 || col == 1 || col == 6 {
                // Middle frame
                TilePixelValue::Two
            } else {
                // Inner square
                TilePixelValue::One
            };

            tile.pixels[row][col] = pixel;
        }
    }

    tile
}

/// Renders tile data to the screen buffer for visualization and debugging purposes.
/// Takes all 384 tiles from memory and arranges them in a grid (16 tiles wide × 24 tiles tall).
/// Each tile is 8×8 pixels and rendered with the Game Boy color palette.
fn render_tile_data_to_screen(tiles: &[Tile; 384], buffer: &mut [u32]) {
    // Game Boy color palette: 0=white, 1=light gray, 2=dark gray, 3=black
    let colors = [0xFFFFFF, 0xAAAAAA, 0x555555, 0x000000];
    let dark_gray = 0x555555;

    // Fill margin area with dark color
    let margin_start = SCREEN_WIDTH * SCREEN_SCALE;
    let margin_end = margin_start + MARGIN;

    for row in 0..TOTAL_WINDOW_HEIGHT {
        for col in margin_start..margin_end {
            let buffer_idx = row * TOTAL_WINDOW_WIDTH + col;
            if buffer_idx < buffer.len() {
                buffer[buffer_idx] = dark_gray;
            }
        }
    }

    // Starting position for tile data (next to the game screen with margin)
    let start_col_offset = SCREEN_WIDTH * SCREEN_SCALE + MARGIN;

    for tile_index in 0..384 {
        // Calculate the grid position of this tile (16 tiles per row)
        let grid_row = tile_index / TILES_PER_ROW;
        let grid_col = tile_index % TILES_PER_ROW;

        let tile = &tiles[tile_index];

        // Render each pixel of the tile
        for tile_row in 0..TILE_SIZE {
            for tile_col in 0..TILE_SIZE {
                let pixel_value = tile.pixels[tile_row][tile_col];
                let color = colors[pixel_value as usize];

                // Calculate the screen position (in tiles, not pixels)
                let screen_row = grid_row * TILE_SIZE;
                let screen_col = grid_col * TILE_SIZE;

                // Apply scaling and offset
                for scale_row in 0..SCREEN_SCALE {
                    for scale_col in 0..SCREEN_SCALE {
                        let buffer_row =
                            screen_row * SCREEN_SCALE + tile_row * SCREEN_SCALE + scale_row;
                        let buffer_col = start_col_offset
                            + screen_col * SCREEN_SCALE
                            + tile_col * SCREEN_SCALE
                            + scale_col;
                        let buffer_idx = buffer_row * TOTAL_WINDOW_WIDTH + buffer_col;

                        if buffer_idx < buffer.len() {
                            buffer[buffer_idx] = color;
                        }
                    }
                }
            }
        }
    }
}

/// Renders a tile to the buffer starting at row and col (0-indexed)
fn render_tile_to_buffer(tile: &Tile, buffer: &mut [u32], start_row: usize, start_col: usize) {
    // Game Boy color palette: 0=white, 1=light gray, 2=dark gray, 3=black
    let colors = [0xFFFFFF, 0xAAAAAA, 0x555555, 0x000000];

    for row in 0..8 {
        for col in 0..8 {
            let pixel_value = tile.pixels[row][col];
            let color = colors[pixel_value as usize];

            let screen_row = start_row + row;
            let screen_col = start_col + col;

            // Write to buffer (each pixel is scaled by SCREEN_SCALE)
            for scale_row in 0..SCREEN_SCALE {
                for scale_col in 0..SCREEN_SCALE {
                    let buffer_row = screen_row * SCREEN_SCALE + scale_row;
                    let buffer_col = screen_col * SCREEN_SCALE + scale_col;
                    let buffer_idx = buffer_row * (SCREEN_WIDTH * SCREEN_SCALE) + buffer_col;

                    if buffer_idx < buffer.len() {
                        buffer[buffer_idx] = color;
                    }
                }
            }
        }
    }
}

fn run_gameboy(cpu: &mut gameboy_core::cpu::Cpu) {
    let mut window = Window::new(
        "Gameboy Emulator",
        TOTAL_WINDOW_WIDTH,
        TOTAL_WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut buffer: Vec<u32> = vec![0xFFFFFF; TOTAL_WINDOW_WIDTH * TOTAL_WINDOW_HEIGHT];

    cpu.set_debug_mode(true);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for _ in 0..70224 {
            cpu.tick();
        }

        cpu.ppu.update_screen_buffer(&cpu.memory_bus);

        let tiles = cpu.ppu.get_tiles_data(&cpu.memory_bus);
        render_tile_data_to_screen(&tiles, &mut buffer);
        render_display(cpu, &mut window, &mut buffer);
    }
}

fn render_display(cpu: &gameboy_core::cpu::Cpu, window: &mut Window, buffer: &mut [u32]) {
    const BUFFER_WIDTH: usize = SCREEN_WIDTH * SCREEN_SCALE;

    // Game Boy color palette: 0=white, 1=light gray, 2=dark gray, 3=black
    let colors = [0xFFFFFF, 0xAAAAAA, 0x555555, 0x000000];

    for row in 0..SCREEN_HEIGHT {
        for col in 0..SCREEN_WIDTH {
            let pixel_value = cpu.ppu.screen[row][col];
            let color = colors[pixel_value as usize];

            // Apply scaling
            for scale_row in 0..SCREEN_SCALE {
                for scale_col in 0..SCREEN_SCALE {
                    let buffer_row = row * SCREEN_SCALE + scale_row;
                    let buffer_col = col * SCREEN_SCALE + scale_col;
                    let buffer_idx = buffer_row * BUFFER_WIDTH + buffer_col;

                    if buffer_idx < buffer.len() {
                        buffer[buffer_idx] = color;
                    }
                }
            }
        }
    }

    println!("Rendering frame to window.");

    window
        .update_with_buffer(buffer, TOTAL_WINDOW_WIDTH, TOTAL_WINDOW_HEIGHT)
        .unwrap();
}

/// Runs the CPU with a minifb window. Press SPACE to execute a CPU tick.
fn run_cpu_with_keyboard(
    cpu: &mut gameboy_core::cpu::Cpu,
    window: &mut Window,
    buffer: &mut Vec<u32>,
) {
    let mut space_pressed: bool = false;
    let mut r_pressed: bool = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Check if SPACE is pressed. The below logic ensures that we only tick once per key press.
        let space_down = window.is_key_down(Key::Space);
        if space_down && !space_pressed {
            cpu.tick();
        }
        space_pressed = space_down;

        // Check if R is pressed to call render screen once per key press.
        let r_down = window.is_key_down(Key::R);
        if r_down && !r_pressed {
            cpu.get_screen_buffer();
        }
        r_pressed = r_down;

        // if window.is_key_down(Key::Space) {
        //     cpu.tick();
        //     println!("CPU tick executed!");
        //     std::thread::sleep(std::time::Duration::from_millis(100));
        // }

        // Update the window with the buffer
        window
            .update_with_buffer(
                buffer,
                SCREEN_WIDTH * SCREEN_SCALE,
                SCREEN_HEIGHT * SCREEN_SCALE,
            )
            .unwrap();
    }
}
