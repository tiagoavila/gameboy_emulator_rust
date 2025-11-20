use crate::gameboy_core::{constants::{SCREEN_HEIGHT, SCREEN_WIDTH}, cpu_utils, ppu_components::{Tile, TilePixelValue}};
use minifb::{Key, Window, WindowOptions};

pub mod gameboy_core;

const SCREEN_SCALE: usize = 5;

fn main() {
    // let rom_file = "tests/cpu_instrs.gb";
    let rom_file = "tests/nintendo_logo.gb";
    let rom_binary = cpu_utils::read_rom(format!("files/roms/{}", rom_file).as_str()).unwrap();

    let mut cpu = gameboy_core::cpu::Cpu::start(rom_binary, true);

    let mut window = Window::new(
        "Gameboy Emulator - Press SPACE to tick CPU",
        SCREEN_WIDTH * SCREEN_SCALE,
        SCREEN_HEIGHT * SCREEN_SCALE,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut buffer: Vec<u32> = vec![0xFFFFFF; SCREEN_WIDTH * SCREEN_HEIGHT * SCREEN_SCALE * SCREEN_SCALE];

    // Create a tile with the letter "T" and render it to the buffer
    let tile_t = create_tile_of_colored_square();
    render_tile_to_buffer(&tile_t, &mut buffer, 0, 0);

    // Run the event loop
    run_cpu_with_keyboard(&mut cpu, &mut window, &mut buffer);
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
        window.update_with_buffer(buffer, SCREEN_WIDTH * SCREEN_SCALE, SCREEN_HEIGHT * SCREEN_SCALE).unwrap();
    }
}
