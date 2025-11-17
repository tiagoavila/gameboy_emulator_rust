use crate::gameboy_core::{
    cpu::Cpu,
    ppu::{BGP, LCDC, TILE_DATA_START, TILE_MAP_0_START},
};

// Simple 8x8 font tiles for letters N, I, T, E, N, D, O
// Each tile is 16 bytes (2 bytes per row, 8 rows)
const TILE_N: [u8; 16] = [
    0xFF, 0x00, // ████████
    0xC3, 0x00, // ██    ██
    0xC7, 0x00, // ██   ███
    0xCF, 0x00, // ██  ████
    0xDB, 0x00, // ██ ██ ██
    0xF3, 0x00, // ████  ██
    0xE3, 0x00, // ███   ██
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

pub fn setup_nintendo_display(cpu: &mut Cpu) {
    // Enable LCD and BG display
    // Bit 7: LCD Enable, Bit 0: BG Display Enable
    cpu.memory_bus.write_byte(LCDC, 0x91);

    // Set background palette (11 10 01 00 - darkest to lightest)
    cpu.memory_bus.write_byte(BGP, 0xE4);

    // Load tile data into VRAM
    // Tile 1: N
    for (i, &byte) in TILE_N.iter().enumerate() {
        cpu.memory_bus.write_byte(TILE_DATA_START + i as u16, byte);
    }

    // Tile 2: I
    for (i, &byte) in TILE_I.iter().enumerate() {
        cpu.memory_bus
            .write_byte(TILE_DATA_START + 16 + i as u16, byte);
    }

    // Tile 3: N (reuse tile 1)
    // for (i, &byte) in TILE_N.iter().enumerate() {
    //     cpu.memory_bus.write_byte(TILE_DATA_START + 32 + i as u16, byte);
    // }

    // Tile 4: T
    for (i, &byte) in TILE_T.iter().enumerate() {
        cpu.memory_bus
            .write_byte(TILE_DATA_START + 48 + i as u16, byte);
    }

    // Tile 5: E
    for (i, &byte) in TILE_E.iter().enumerate() {
        cpu.memory_bus
            .write_byte(TILE_DATA_START + 64 + i as u16, byte);
    }

    // Tile 6: N (reuse tile 1)

    // Tile 7: D
    for (i, &byte) in TILE_D.iter().enumerate() {
        cpu.memory_bus
            .write_byte(TILE_DATA_START + 96 + i as u16, byte);
    }

    // Tile 8: O
    for (i, &byte) in TILE_O.iter().enumerate() {
        cpu.memory_bus
            .write_byte(TILE_DATA_START + 112 + i as u16, byte);
    }

    // Write tile indices to background map to spell "NINTENDO"
    // Center it roughly on screen (row 8, starting at column 6)
    let start_pos = TILE_MAP_0_START + (8 * 32) + 6;

    cpu.memory_bus.write_byte(start_pos + 0, 0); // N (tile 0)
    cpu.memory_bus.write_byte(start_pos + 1, 1); // I (tile 1)
    cpu.memory_bus.write_byte(start_pos + 2, 0); // N (tile 0)
    cpu.memory_bus.write_byte(start_pos + 3, 3); // T (tile 3)
    cpu.memory_bus.write_byte(start_pos + 4, 4); // E (tile 4)
    cpu.memory_bus.write_byte(start_pos + 5, 0); // N (tile 0)
    cpu.memory_bus.write_byte(start_pos + 6, 6); // D (tile 6)
    cpu.memory_bus.write_byte(start_pos + 7, 7); // O (tile 7)
}

#[cfg(test)]
mod tests {
    use crate::gameboy_core::{ppu::{BGP, LCDC, TILE_DATA_START, TILE_MAP_0_START}, ppu_test::setup_nintendo_display};

    #[test]
    fn render_nintendo_logo_tiles() {
        let mut cpu = crate::gameboy_core::cpu::Cpu::start(
            crate::cpu_utils::read_rom("files/roms/tests/nintendo_logo.gb").unwrap(),
            true,
        );
        setup_nintendo_display(&mut cpu);

        // Verify LCD is enabled
        assert_eq!(cpu.memory_bus.read_byte(LCDC) & 0x80, 0x80);

        // Verify palette is set
        assert_eq!(cpu.memory_bus.read_byte(BGP), 0xE4);

        // Verify first tile (N) is loaded
        assert_eq!(cpu.memory_bus.read_byte(TILE_DATA_START), 0xFF);

        // Verify background map has correct tile indices
        let start_pos = TILE_MAP_0_START + (8 * 32) + 6;
        assert_eq!(cpu.memory_bus.read_byte(start_pos + 0), 0); // N
        assert_eq!(cpu.memory_bus.read_byte(start_pos + 1), 1); // I
        assert_eq!(cpu.memory_bus.read_byte(start_pos + 7), 7); // O

        cpu.render_screen();
    }
}
