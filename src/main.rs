use crate::gameboy_core::{
    components::screen::{Screen, TOTAL_WINDOW_HEIGHT, TOTAL_WINDOW_WIDTH},
    constants::{COLORS, GAME_SECTION_HEIGHT, GAME_SECTION_WIDTH, SCREEN_SCALE, TILE_SIZE},
    cpu_utils,
    ppu_components::{Tile, TilePixelValue},
};
use minifb::{Key, Window};

pub mod gameboy_core;

fn main() {
    // let rom_file = "games/Tetris.gb";
    let rom_file = "games/Super Mario Land.gb";
    let rom_binary = cpu_utils::read_rom(format!("files/roms/{}", rom_file).as_str()).unwrap();

    let debug_mode = false;
    let mut cpu = gameboy_core::cpu::Cpu::start(rom_binary, debug_mode);

    if debug_mode {
        // clear previous logs
        cpu_utils::clear_logs().unwrap();
        cpu_utils::clear_dr_gameboy_log().unwrap();
    }

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

/// Renders a tile to the buffer starting at row and col (0-indexed)
fn render_tile_to_buffer(tile: &Tile, buffer: &mut [u32], start_row: usize, start_col: usize) {
    for row in 0..8 {
        for col in 0..8 {
            let pixel_value = tile.pixels[row][col];
            let color = COLORS[pixel_value as usize];

            let screen_row = start_row + row;
            let screen_col = start_col + col;

            // Write to buffer (each pixel is scaled by SCREEN_SCALE)
            for scale_row in 0..SCREEN_SCALE {
                for scale_col in 0..SCREEN_SCALE {
                    let buffer_row = screen_row * SCREEN_SCALE + scale_row;
                    let buffer_col = screen_col * SCREEN_SCALE + scale_col;
                    let buffer_idx = buffer_row * (GAME_SECTION_WIDTH * SCREEN_SCALE) + buffer_col;

                    if buffer_idx < buffer.len() {
                        buffer[buffer_idx] = color;
                    }
                }
            }
        }
    }
}

fn run_gameboy(cpu: &mut gameboy_core::cpu::Cpu) {
    let mut screen = Screen::new("Gameboy Emulator")
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let mut buffer: Vec<u32> = vec![0x000080; TOTAL_WINDOW_WIDTH * TOTAL_WINDOW_HEIGHT];

    cpu.set_debug_mode(true);

    while screen.window.is_open() && !screen.window.is_key_down(Key::Escape) {
        for _ in 0..70224 {
            cpu.tick();
        }

        cpu.ppu.update_screen_buffer(&cpu.memory_bus);

        Screen::render_tile_data_to_screen_buffer(cpu, &mut buffer);
        Screen::render_game_to_screen_buffer(cpu, &mut buffer);

        screen.window
            .update_with_buffer(&buffer, TOTAL_WINDOW_WIDTH, TOTAL_WINDOW_HEIGHT)
            .unwrap();
    }
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
                GAME_SECTION_WIDTH * SCREEN_SCALE,
                GAME_SECTION_HEIGHT * SCREEN_SCALE,
            )
            .unwrap();
    }
}
