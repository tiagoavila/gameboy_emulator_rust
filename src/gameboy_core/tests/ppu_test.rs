use crate::gameboy_core::{
    constants::{TILE_DATA_START, TILE_MAP_AREA_0_START}, cpu::Cpu, registers_contants::{BGP, LCDC}
};

// Simple 8x8 font tiles for letters N, I, T, E, N, D, O
// Each tile is 16 bytes (2 bytes per row, 8 rows)
const TILE_N: [u8; 16] = [
    0xC3, 0x00, // ██    ██
    0xC3, 0x00, // ██    ██
    0xE3, 0x00, // ███   ██
    0xF3, 0x00, // ████  ██
    0xDB, 0x00, // ██ ██ ██
    0xCF, 0x00, // ██  ████
    0xC7, 0x00, // ██   ███
    0xC3, 0x00, // ██    ██
];

const TILE_I: [u8; 16] = [
    0xFF, 0x00, // ████████
    0x18, 0x00, //    ██
    0x18, 0x00, //    ██
    0x18, 0x00, //    ██
    0x18, 0x00, //    ██
    0x18, 0x00, //    ██
    0x18, 0x00, //    ██
    0xFF, 0x00, // ████████
];

const TILE_T: [u8; 16] = [
    0xFF, 0x00, // ████████
    0x18, 0x00, //    ██
    0x18, 0x00, //    ██
    0x18, 0x00, //    ██
    0x18, 0x00, //    ██
    0x18, 0x00, //    ██
    0x18, 0x00, //    ██
    0x18, 0x00, //    ██
];

const TILE_E: [u8; 16] = [
    0xFF, 0x00, // ████████
    0xC0, 0x00, // ██
    0xC0, 0x00, // ██
    0xFE, 0x00, // ███████
    0xC0, 0x00, // ██
    0xC0, 0x00, // ██
    0xC0, 0x00, // ██
    0xFF, 0x00, // ████████
];

const TILE_D: [u8; 16] = [
    0xFC, 0x00, // ██████
    0xC6, 0x00, // ██   ██
    0xC3, 0x00, // ██    ██
    0xC3, 0x00, // ██    ██
    0xC3, 0x00, // ██    ██
    0xC3, 0x00, // ██    ██
    0xC6, 0x00, // ██   ██
    0xFC, 0x00, // ██████
];

const TILE_O: [u8; 16] = [
    0x7E, 0x00, //  ██████
    0xC3, 0x00, // ██    ██
    0xC3, 0x00, // ██    ██
    0xC3, 0x00, // ██    ██
    0xC3, 0x00, // ██    ██
    0xC3, 0x00, // ██    ██
    0xC3, 0x00, // ██    ██
    0x7E, 0x00, //  ██████
];

// Empty tile (all white/zero pixels)
const TILE_EMPTY: [u8; 16] = [
    0x00, 0x00, // (empty)
    0x00, 0x00, // (empty)
    0x00, 0x00, // (empty)
    0x00, 0x00, // (empty)
    0x00, 0x00, // (empty)
    0x00, 0x00, // (empty)
    0x00, 0x00, // (empty)
    0x00, 0x00, // (empty)
];

// A representation of the Game Boy "draw"
const TILE_GAME_BOY_DRAW: [u8; 16] = [ 0x3C, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0x5E, 0x7E, 0x0A, 0x7C, 0x56, 0x38, 0x7C];

pub fn setup_nintendo_display(cpu: &mut Cpu) {
    // Enable LCD and BG display
    // Bit 7: LCD Enable, Bit 0: BG Display Enable
    cpu.memory_bus.write_byte(LCDC, 0x91);

    // Set background palette (11 10 01 00 - darkest to lightest)
    cpu.memory_bus.write_byte(BGP, 0xE4);

    // Load tile data into VRAM
    // Tile 0: Empty (background)
    for (i, &byte) in TILE_EMPTY.iter().enumerate() {
        cpu.memory_bus.write_byte(TILE_DATA_START + i as u16, byte);
    }

    // Tile 1: N
    let mut tile_offset = 16;
    for (i, &byte) in TILE_N.iter().enumerate() {
        cpu.memory_bus
            .write_byte(TILE_DATA_START + tile_offset + i as u16, byte);
    }

    // Tile 2: I
    tile_offset += 16;
    for (i, &byte) in TILE_I.iter().enumerate() {
        cpu.memory_bus
            .write_byte(TILE_DATA_START + tile_offset + i as u16, byte);
    }

    // Tile 3: T
    tile_offset += 16;
    for (i, &byte) in TILE_T.iter().enumerate() {
        cpu.memory_bus
            .write_byte(TILE_DATA_START + tile_offset + i as u16, byte);
    }

    // Tile 4: E
    tile_offset += 16;
    for (i, &byte) in TILE_E.iter().enumerate() {
        cpu.memory_bus
            .write_byte(TILE_DATA_START + tile_offset + i as u16, byte);
    }

    // Tile 5: D
    tile_offset += 16;
    for (i, &byte) in TILE_D.iter().enumerate() {
        cpu.memory_bus
            .write_byte(TILE_DATA_START + tile_offset + i as u16, byte);
    }

    // Tile 6: O
    tile_offset += 16;
    for (i, &byte) in TILE_O.iter().enumerate() {
        cpu.memory_bus
            .write_byte(TILE_DATA_START + tile_offset + i as u16, byte);
    }
    
    // Tile 7: Game Boy Draw
    tile_offset += 16;
    for (i, &byte) in TILE_GAME_BOY_DRAW.iter().enumerate() {
        cpu.memory_bus
            .write_byte(TILE_DATA_START + tile_offset + i as u16, byte);
    }

    // Write tile indices to background map to spell "NINTENDO"
    // Center it roughly on screen (row 8, starting at column 6)
    let start_pos = TILE_MAP_AREA_0_START + (8 * 32) + 6;

    cpu.memory_bus.write_byte(start_pos + 0, 1);   // N (tile 1)
    cpu.memory_bus.write_byte(start_pos + 2, 2);   // I (tile 2)
    cpu.memory_bus.write_byte(start_pos + 4, 1);   // N (tile 1)
    cpu.memory_bus.write_byte(start_pos + 6, 3);   // T (tile 4)
    cpu.memory_bus.write_byte(start_pos + 8, 4);   // E (tile 5)
    cpu.memory_bus.write_byte(start_pos + 10, 1);  // N (tile 1)
    cpu.memory_bus.write_byte(start_pos + 12, 5);  // D (tile 5)
    cpu.memory_bus.write_byte(start_pos + 14, 6);  // O (tile 6)
    cpu.memory_bus.write_byte(start_pos + 16, 7);  // Game Boy Draw (tile 7)
}

#[cfg(test)]
mod tests {
    use crate::gameboy_core::{constants::{BG_AND_WINDOW_MAP_SCREEN_SIZE, GAME_SECTION_HEIGHT, GAME_SECTION_WIDTH, TILE_MAP_AREA_0_START}, ppu_components, registers_contants::{BGP, LCDC}};
    use minifb::{Key, Window, WindowOptions};

    #[test]
    fn render_nintendo_logo_tiles_in_bg_screen() {
        let mut cpu = crate::gameboy_core::cpu::Cpu::start(
            crate::cpu_utils::read_rom("files/roms/tests/nintendo_logo.gb").unwrap(),
            true,
        );
        super::setup_nintendo_display(&mut cpu);

        // Verify LCD is enabled
        assert_eq!(cpu.memory_bus.read_byte(LCDC) & 0x80, 0x80);

        // Verify palette is set
        assert_eq!(cpu.memory_bus.read_byte(BGP), 0xE4);

        // Verify first tile (N) is loaded
        // assert_eq!(cpu.memory_bus.read_byte(TILE_DATA_START), 0xFF);

        // Verify background map has correct tile indices
        let start_pos = TILE_MAP_AREA_0_START + (8 * 32) + 6;
        assert_eq!(cpu.memory_bus.read_byte(start_pos + 0), 1); // N
        assert_eq!(cpu.memory_bus.read_byte(start_pos + 1), 0); // Empty
        assert_eq!(cpu.memory_bus.read_byte(start_pos + 2), 2); // I
        assert_eq!(cpu.memory_bus.read_byte(start_pos + 3), 0); // Empty
        assert_eq!(cpu.memory_bus.read_byte(start_pos + 14), 6); // O

        // Get screen buffer and render it
        let tiles = cpu.ppu.get_tiles(&cpu.memory_bus);
        let lcdc_register = ppu_components::LcdcRegister::get_lcdc_register(&cpu.memory_bus);
        let bg_screen_buffer = cpu.ppu.get_bg_buffer(&cpu.memory_bus, &tiles, &lcdc_register);
        render_bg_screen_with_minifb(&bg_screen_buffer);
    }

    #[test]
    fn render_nintendo_logo_tiles_in_visible_screen() {
        let mut cpu = crate::gameboy_core::cpu::Cpu::start(
            crate::cpu_utils::read_rom("files/roms/tests/nintendo_logo.gb").unwrap(),
            true,
        );
        cpu.memory_bus.set_scx_register(40);
        super::setup_nintendo_display(&mut cpu);

        // Verify LCD is enabled
        assert_eq!(cpu.memory_bus.read_byte(LCDC) & 0x80, 0x80);

        // Verify palette is set
        assert_eq!(cpu.memory_bus.read_byte(BGP), 0xE4);

        // Verify background map has correct tile indices
        let start_pos = TILE_MAP_AREA_0_START + (8 * 32) + 6;
        assert_eq!(cpu.memory_bus.read_byte(start_pos + 0), 1); // N
        assert_eq!(cpu.memory_bus.read_byte(start_pos + 1), 0); // Empty
        assert_eq!(cpu.memory_bus.read_byte(start_pos + 2), 2); // I
        assert_eq!(cpu.memory_bus.read_byte(start_pos + 3), 0); // Empty
        assert_eq!(cpu.memory_bus.read_byte(start_pos + 14), 6); // O

        // Get screen buffer and render it
        let screen_buffer = cpu.get_screen_buffer();
        render_visible_screen_with_minifb(&screen_buffer);
    }

    fn render_bg_screen_with_minifb(screen_buffer: &[[u8; BG_AND_WINDOW_MAP_SCREEN_SIZE]; BG_AND_WINDOW_MAP_SCREEN_SIZE]) {
        const SCREEN_SCALE: usize = 3;
        const BUFFER_WIDTH: usize = BG_AND_WINDOW_MAP_SCREEN_SIZE * SCREEN_SCALE;
        const BUFFER_HEIGHT: usize = BG_AND_WINDOW_MAP_SCREEN_SIZE * SCREEN_SCALE;

        let mut window = Window::new(
            "Nintendo Logo - Printing the background screen of 256x256 pixels", 
            BUFFER_WIDTH,
            BUFFER_HEIGHT,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("Failed to create window: {}", e);
        });

        // Game Boy color palette: 0=white, 1=light gray, 2=dark gray, 3=black
        let colors = [0xFFFFFF, 0xAAAAAA, 0x555555, 0x000000];

        let mut buffer: Vec<u32> = vec![0xFFFFFF; BUFFER_WIDTH * BUFFER_HEIGHT];

        // Convert 2D screen buffer to 1D buffer with scaling
        for row in 0..BG_AND_WINDOW_MAP_SCREEN_SIZE {
            for col in 0..BG_AND_WINDOW_MAP_SCREEN_SIZE {
                let pixel_value = screen_buffer[row][col];
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

        // Display the window until closed
        while window.is_open() && !window.is_key_down(Key::Escape) {
            window
                .update_with_buffer(&buffer, BUFFER_WIDTH, BUFFER_HEIGHT)
                .unwrap();
        }
    }

    fn render_visible_screen_with_minifb(screen_buffer: &[[u8; GAME_SECTION_WIDTH]; GAME_SECTION_HEIGHT]) {
        const SCREEN_SCALE: usize = 3;
        const BUFFER_WIDTH: usize = GAME_SECTION_WIDTH * SCREEN_SCALE;
        const BUFFER_HEIGHT: usize = GAME_SECTION_HEIGHT * SCREEN_SCALE;

        let mut window = Window::new(
            "Nintendo Logo - Printing the visible screen of 160x144 pixels", 
            BUFFER_WIDTH,
            BUFFER_HEIGHT,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("Failed to create window: {}", e);
        });

        // Game Boy color palette: 0=white, 1=light gray, 2=dark gray, 3=black
        let colors = [0xFFFFFF, 0xAAAAAA, 0x555555, 0x000000];

        let mut buffer: Vec<u32> = vec![0xFFFFFF; BUFFER_WIDTH * BUFFER_HEIGHT];

        // Convert 2D screen buffer to 1D buffer with scaling
        for row in 0..GAME_SECTION_HEIGHT {
            for col in 0..GAME_SECTION_WIDTH {
                let pixel_value = screen_buffer[row][col];
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

        // Display the window until closed
        while window.is_open() && !window.is_key_down(Key::Escape) {
            window
                .update_with_buffer(&buffer, BUFFER_WIDTH, BUFFER_HEIGHT)
                .unwrap();
        }
    }
}
