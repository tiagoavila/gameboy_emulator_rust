use crate::gameboy_core::{
    constants::{
        BG_AND_WINDOW_MAP_SCREEN_SIZE, BG_AND_WINDOW_TILE_COUNT_PER_ROW_COL, COLORS,
        GAME_SECTION_HEIGHT, GAME_SECTION_WIDTH,
    },
    cpu, cpu_components,
    interrupts::InterruptType,
    ppu_components::{self, Tile, TilePixelValue},
    registers_contants::{BGP, LY, LYC, OBP0, OBP1, STAT},
};

/// Number of T-cycles per scanline (or LCD line). LY increments every 456 T-cycles.
const T_CYCLES_PER_SCANLINE: u16 = 456;

/// The line where V-Blank starts.
const VBLANK_START_LINE: u8 = 144;

/// LY holds values from 0 to 153, so total lines are 154.
const LY_MAX_LINES: u8 = 154;

#[derive(PartialEq)]
enum PpuMode {
    HBlank = 0,
    VBlank = 1,
    OamSearch = 2,
    PixelTransfer = 3,
}

#[derive(Copy, Clone, Debug)]
pub struct Object {
    pub y: u8,
    pub x: u8,
    pub tile_index: usize,
    pub attributes: ObjectAttributes,
}

#[derive(Copy, Clone, Debug)]
pub struct ObjectAttributes {
    pub priority: bool,
    pub y_flip: bool,
    pub x_flip: bool,
    pub pallete: ObjectPallete,
}

#[derive(Copy, Clone, Debug)]
pub enum ObjectPallete {
    OBP0 = 0,
    OBP1 = 1,
}

pub struct Ppu {
    pub screen: [[u32; GAME_SECTION_WIDTH]; GAME_SECTION_HEIGHT], // 144 rows of 160 pixels
    pub dots: u16,
    pub objects_to_be_rendered: Vec<Object>,
    pub bg_screen_buffer: [[u32; GAME_SECTION_WIDTH]; GAME_SECTION_HEIGHT],
    pub need_to_render_line: bool,
}

impl Ppu {
    pub(crate) fn new() -> Self {
        Self {
            screen: [[0; GAME_SECTION_WIDTH]; GAME_SECTION_HEIGHT],
            dots: 0,
            objects_to_be_rendered: Vec::new(),
            bg_screen_buffer: [[0; GAME_SECTION_WIDTH]; GAME_SECTION_HEIGHT],
            need_to_render_line: false,
        }
    }

    /// Generates the screen buffer representing the visible 160x144 pixel screen.
    /// This will build the Background first, then apply the Window (if enabled), and finally render the Objects - Sprites (if enabled).
    pub fn update_screen_buffer(&mut self, memory_bus: &cpu_components::MemoryBus) {
        self.screen = self.get_bg_screen_buffer_as_colors(memory_bus);
    }

    /// Generates the background screen buffer representing the visible 160x144 pixel screen in color values.
    /// Where the color is an u32 representing the RGB value.
    pub fn get_bg_screen_buffer_as_colors(
        &self,
        memory_bus: &cpu_components::MemoryBus,
    ) -> [[u32; GAME_SECTION_WIDTH]; GAME_SECTION_HEIGHT] {
        let lcdc_register = ppu_components::LcdcRegister::get_lcdc_register(memory_bus);

        // When Bit 0 is cleared, both background and window become blank (white), and the Window Display Bit is ignored in that case.
        // Only objects may still be displayed (if enabled in Bit 1).
        if lcdc_register.bg_window_enable == false {
            return [[0xFFFFFF; GAME_SECTION_WIDTH]; GAME_SECTION_HEIGHT];
        }

        let bg_screen_buffer = self.get_bg_screen_buffer(memory_bus);
        let mut color_screen_buffer = [[0u32; GAME_SECTION_WIDTH]; GAME_SECTION_HEIGHT];

        for row in 0..GAME_SECTION_HEIGHT {
            for col in 0..GAME_SECTION_WIDTH {
                let pixel_value = bg_screen_buffer[row][col];
                let color = COLORS[pixel_value as usize];
                color_screen_buffer[row][col] = color;
            }
        }

        color_screen_buffer
    }

    /// Generates the background screen buffer representing the visible 160x144 pixel screen.
    /// This will build the Background only returning it in a color pallete value only.
    pub fn get_bg_screen_buffer(
        &self,
        memory_bus: &cpu_components::MemoryBus,
    ) -> [[u8; GAME_SECTION_WIDTH]; GAME_SECTION_HEIGHT] {
        let lcdc_register = ppu_components::LcdcRegister::get_lcdc_register(memory_bus);
        let tiles = self.get_tiles(memory_bus);

        let bg_buffer = self.get_entire_bg_buffer(memory_bus, &tiles, &lcdc_register);
        let screen_buffer = self.get_visible_bg_buffer(&bg_buffer, memory_bus);

        screen_buffer
    }

    /// Returns the entire set of Tiles from VRAM.
    /// Tiles are used to build the background, window, and objects (sprites).
    pub fn get_tiles_data(&self, memory_bus: &cpu_components::MemoryBus) -> [Tile; 384] {
        self.get_tiles(memory_bus)
    }

    /// Returns the visible portion of the background buffer based on the SCX and SCY scroll values and to fit the 160x144 screen.
    /// The PPU calculates the bottom-right coordinates of the viewport with those formulas:
    /// bottom := (SCY + 143) % 256 and right := (SCX + 159) % 256.
    /// As suggested by the modulo operations, in case the values are larger than 255 they will “wrap around” towards the top-left corner of the tilemap.
    fn get_visible_bg_buffer(
        &self,
        bg_buffer: &[[u8; BG_AND_WINDOW_MAP_SCREEN_SIZE]; BG_AND_WINDOW_MAP_SCREEN_SIZE],
        memory_bus: &cpu_components::MemoryBus,
    ) -> [[u8; GAME_SECTION_WIDTH]; GAME_SECTION_HEIGHT] {
        let scy = memory_bus.get_scy_register() as usize;
        let scx = memory_bus.get_scx_register() as usize;
        let mut visible_bg_buffer = [[0u8; GAME_SECTION_WIDTH]; GAME_SECTION_HEIGHT];

        for screen_row in 0..GAME_SECTION_HEIGHT {
            for screen_col in 0..GAME_SECTION_WIDTH {
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
    pub fn get_entire_bg_buffer(
        &self,
        memory_bus: &cpu_components::MemoryBus,
        tiles: &[Tile; 384],
        lcdc_register: &ppu_components::LcdcRegister,
    ) -> [[u8; BG_AND_WINDOW_MAP_SCREEN_SIZE]; BG_AND_WINDOW_MAP_SCREEN_SIZE] {
        let bg_tile_map = self.get_bg_tile_map_as_grid_32x32(memory_bus, &lcdc_register);
        let mut bg_buffer = [[0u8; BG_AND_WINDOW_MAP_SCREEN_SIZE]; BG_AND_WINDOW_MAP_SCREEN_SIZE];
        let bg_tiles = self.get_bg_and_window_tiles(tiles, &lcdc_register);

        for tile_map_row in 0..BG_AND_WINDOW_TILE_COUNT_PER_ROW_COL {
            for tile_map_col in 0..BG_AND_WINDOW_TILE_COUNT_PER_ROW_COL {
                let tile_index = bg_tile_map[tile_map_row][tile_map_col] as usize;
                let tile = &bg_tiles[tile_index];

                for tile_row in 0..8 {
                    for tile_col in 0..8 {
                        let color_pallete_value = match tile.pixels[tile_row][tile_col] {
                            TilePixelValue::Zero => 0,
                            TilePixelValue::One => 1,
                            TilePixelValue::Two => 2,
                            TilePixelValue::Three => 3,
                        };
                        let buffer_row = tile_map_row * 8 + tile_row;
                        let buffer_col = tile_map_col * 8 + tile_col;
                        bg_buffer[buffer_row][buffer_col] = color_pallete_value;
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
    pub fn get_tiles(&self, memory_bus: &cpu_components::MemoryBus) -> [Tile; 384] {
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

    /// Get the Tiles used for Background and Window based on the LCDC register settings.
    /// If the bg_window_tile_data_area (bit 4 in LCDC) is set to 1, it uses the tile data from $8000-$8FFF (tiles 0-255).
    /// If it is set to 0, it uses the tile data from $8800-$97FF, with tiles from index 0-127 in range $9000-$97FF
    /// and tiles from index 128-255 in range $8800-$8FFF.
    fn get_bg_and_window_tiles(
        &self,
        tiles: &[Tile; 384],
        lcdc: &ppu_components::LcdcRegister,
    ) -> [Tile; 256] {
        if lcdc.bg_window_tile_data_area {
            tiles[0..256].try_into().unwrap()
        } else {
            let block2: [Tile; 128] = tiles[256..].try_into().unwrap();
            let block1: [Tile; 128] = tiles[128..256].try_into().unwrap(); // End index of a slice is exclusive
            return [block2, block1].concat().try_into().unwrap();
        }
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

    /// Increases the LY register based on the number of T-cycles (dots) executed and, updates PPU mode and interrupts accordingly.
    /// This method should be called every instruction execution to keep the PPU state updated.
    pub(crate) fn update_state(cpu: &mut cpu::Cpu) {
        cpu.ppu.dots += 4;

        if cpu.ppu.dots >= T_CYCLES_PER_SCANLINE {
            cpu.ppu.dots -= T_CYCLES_PER_SCANLINE;

            // Ensures LY wraps around after reaching the maximum number of lines. So it goes from 0 to 153 and then back to 0.
            // To do this we increment LY and then apply modulo operation with LY_MAX_LINES (154). If LY after incrementing is 154, it becomes 0.
            let ly: u8 = (cpu.memory_bus.read_byte(LY) + 1) % LY_MAX_LINES;
            cpu.memory_bus.write_byte(LY, ly);

            if ly == VBLANK_START_LINE {
                // Trigger V-Blank interrupt
                cpu.memory_bus
                    .update_flag_in_if_register(InterruptType::VBlank, true);

                // Set mode to 1 (V-Blank)
                Ppu::set_ppu_mode_flag_in_stat(cpu, PpuMode::VBlank);
                
                // FIX: Update the cached background buffer at V-Blank
                // This ensures the screen is ready for the next frame and helps with any pending changes
                cpu.ppu.update_screen_buffer(&cpu.memory_bus);
            } else if ly == 0 {
                // This handles V-Blank Exit (transition from V-Blank to OAM Search)
                Ppu::set_ppu_mode_flag_in_stat(cpu, PpuMode::OamSearch);
            }
            
            // FIX: Define objects for the NEW scanline now that LY has been incremented
            // This ensures that when render_line() is called during Pixel Transfer,
            // the correct objects for the current scanline are available
            if ly < VBLANK_START_LINE {
                Ppu::define_objects_to_be_rendered(cpu, ly);
            }
        } else {
            // Update the PPU mode based on the current number of dots (T-cycles) in the scanline.
            // each dot represents a T-cycle
            match cpu.ppu.dots {
                // OAM Search lasts 80 dots
                0..=79 => {
                    Ppu::set_ppu_mode_flag_in_stat(cpu, PpuMode::OamSearch);
                    cpu.ppu.need_to_render_line = false;
                }
                // Drawing pixels lasts at least 172 dots, there are cases where it can take longer but I'm using the simplest approach for now
                80..=251 => {
                    let current_ppu_mode = Ppu::get_ppu_mode_flag_from_stat(cpu);
                    if current_ppu_mode != PpuMode::PixelTransfer {
                        Ppu::render_line(cpu);
                        Ppu::set_ppu_mode_flag_in_stat(cpu, PpuMode::PixelTransfer);
                        cpu.ppu.need_to_render_line = true;
                    }
                }
                // Greater than or equal to 252 dots means the rest of the scanline (H-Blank)
                _ => {
                    Ppu::set_ppu_mode_flag_in_stat(cpu, PpuMode::HBlank);
                }
            }
        }

        Self::compare_lyc(cpu);
    }

    /// Compares the LY and LYC registers and sets or clears the matching flag in the STAT register (bit 2).
    fn compare_lyc(cpu: &mut cpu::Cpu) {
        let ly = cpu.memory_bus.read_byte(LY);
        let lyc: u8 = cpu.memory_bus.read_byte(LYC);
        let mut stat = cpu.memory_bus.read_byte(STAT);
        if ly == lyc {
            stat |= 0b00000100; // Set the LY=LYC flag
            cpu.memory_bus
                .update_flag_in_if_register(InterruptType::LCD, true);
        } else {
            stat &= 0b11111011; // Clear the LY=LYC flag
        }

        cpu.memory_bus.write_byte(STAT, stat);
    }

    /// Sets the PPU mode flag in the STAT register.
    fn set_ppu_mode_flag_in_stat(cpu: &mut cpu::Cpu, mode: PpuMode) {
        let mut stat = cpu.memory_bus.read_byte(STAT);
        stat = (stat & 0b11111100) | (mode as u8);
        cpu.memory_bus.write_byte(STAT, stat);
    }

    /// Gets the current PPU mode flag from the STAT register.
    fn get_ppu_mode_flag_from_stat(cpu: &cpu::Cpu) -> PpuMode {
        let stat = cpu.memory_bus.read_byte(STAT);
        match stat & 0b00000011 {
            0 => PpuMode::HBlank,
            1 => PpuMode::VBlank,
            2 => PpuMode::OamSearch,
            3 => PpuMode::PixelTransfer,
            _ => unreachable!(),
        }
    }

    const Y_OFFSET: isize = 16;

    /// Sets the 10 objects (sprites) to be rendered for the current scanline (LY)
    /// and stores them in the PPU.
    fn define_objects_to_be_rendered(cpu: &mut cpu::Cpu, ly: u8) {
        let ly_isize = ly as isize;
        let objects = Ppu::get_all_40_objects(&cpu.memory_bus);
        let lcdc_register = ppu_components::LcdcRegister::get_lcdc_register(&cpu.memory_bus);
        let object_height: isize = if lcdc_register.obj_size { 16 } else { 8 };
        
        let objects_to_be_rendered = objects
            .iter()
            .filter_map(|object| {
                let start_object_screen_y = (object.y as isize) - Self::Y_OFFSET;
                let end_object_screen_y: isize = start_object_screen_y + object_height;
                let start_object_screen_x = (object.x as isize) - 8;
                let end_object_screen_x: isize = start_object_screen_x + 8;
                
                if (start_object_screen_y <= ly_isize && ly_isize < end_object_screen_y)
                    && (end_object_screen_x >= 0 && start_object_screen_x < GAME_SECTION_WIDTH as isize)
                {
                    Some(*object)
                } else {
                    None
                }
            })
            .collect::<Vec<Object>>();

        cpu.ppu.objects_to_be_rendered = if objects_to_be_rendered.len() <= 10 {
            objects_to_be_rendered
        } else {
            objects_to_be_rendered[..10].to_vec()
        };
    }

    /// Get all 40 objects (sprites) from OAM (Object Attribute Memory).
    fn get_all_40_objects(memory_bus: &cpu_components::MemoryBus) -> [Object; 40] {
        let oam_memory = memory_bus.get_object_attribute_memory();

        let objects = oam_memory
            .chunks(4)
            .map(|obj| Object {
                y: obj[0],
                x: obj[1],
                tile_index: obj[2] as usize,
                attributes: ObjectAttributes {
                    priority: (obj[3] & 0b1000_0000) != 0,
                    y_flip: (obj[3] & 0b0100_0000) != 0,
                    x_flip: (obj[3] & 0b0010_0000) != 0,
                    // FIX: Bit 4 selects the object palette, not bit 6 (which is y_flip)
                    // Bit 7 = priority, Bit 6 = y_flip, Bit 5 = x_flip, Bit 4 = palette selection
                    pallete: if (obj[3] & 0b0001_0000) != 0 {
                        ObjectPallete::OBP1
                    } else {
                        ObjectPallete::OBP0
                    },
                },
            })
            .collect::<Vec<Object>>()
            .try_into()
            .unwrap();

        objects
    }

    fn render_line(cpu: &mut cpu::Cpu) {
        let ly = cpu.memory_bus.read_byte(LY);
        if ly >= 144 {
            return;
        }

        let ly_usize = ly as usize;

        Ppu::render_background_line_to_screen_buffer(cpu, ly_usize);

        let lcdc = ppu_components::LcdcRegister::get_lcdc_register(&cpu.memory_bus);

        Ppu::render_window_line_to_screen_buffer(cpu, ly_usize, &lcdc);

        Ppu::render_objects_line_to_screen_buffer(cpu, ly_usize, &lcdc);
    }

    fn render_background_line_to_screen_buffer(cpu: &mut cpu::Cpu, ly_usize: usize) {
        // FIX: Only calculate the single scanline instead of the entire 144x160 background buffer
        // This is much more efficient than regenerating all 23,040 pixels just to render 160
        let lcdc_register = ppu_components::LcdcRegister::get_lcdc_register(&cpu.memory_bus);

        // When Bit 0 is cleared, both background and window become blank (white)
        if lcdc_register.bg_window_enable == false {
            cpu.ppu.screen[ly_usize] = [0xFFFFFF; GAME_SECTION_WIDTH];
            return;
        }

        let tiles = cpu.ppu.get_tiles(&cpu.memory_bus);
        let bg_tile_map = cpu.ppu.get_bg_tile_map_as_grid_32x32(&cpu.memory_bus, &lcdc_register);
        let bg_tiles = cpu.ppu.get_bg_and_window_tiles(&tiles, &lcdc_register);
        let scy = cpu.memory_bus.get_scy_register() as usize;
        let scx = cpu.memory_bus.get_scx_register() as usize;

        // Calculate which row in the 256x256 background map we need
        let bg_row = (scy + ly_usize) % BG_AND_WINDOW_MAP_SCREEN_SIZE;
        let tile_map_row = bg_row / 8;
        let tile_pixel_row = bg_row % 8;

        // Render this scanline pixel by pixel
        for screen_col in 0..GAME_SECTION_WIDTH {
            let bg_col = (scx + screen_col) % BG_AND_WINDOW_MAP_SCREEN_SIZE;
            let tile_map_col = bg_col / 8;
            let tile_pixel_col = bg_col % 8;

            let tile_index = bg_tile_map[tile_map_row][tile_map_col] as usize;
            let tile = &bg_tiles[tile_index];

            let color_pallete_value = match tile.pixels[tile_pixel_row][tile_pixel_col] {
                TilePixelValue::Zero => 0,
                TilePixelValue::One => 1,
                TilePixelValue::Two => 2,
                TilePixelValue::Three => 3,
            };

            let bgp_register = cpu.memory_bus.read_byte(BGP);
            let palette_index = ((bgp_register >> (color_pallete_value * 2)) & 0b11) as usize;
            let color = COLORS[palette_index];
            cpu.ppu.screen[ly_usize][screen_col] = color;
        }
    }
        
    fn render_window_line_to_screen_buffer(_cpu: &mut cpu::Cpu, _ly_usize: usize, lcdc: &ppu_components::LcdcRegister) {
        if lcdc.window_enable {
        }
    }
    
    fn render_objects_line_to_screen_buffer(cpu: &mut cpu::Cpu, ly_usize: usize, lcdc: &ppu_components::LcdcRegister) {
        if lcdc.obj_enable {
            let tiles = cpu.ppu.get_tiles(&cpu.memory_bus);
            let objects = &cpu.ppu.objects_to_be_rendered;
            let obp0_register = cpu.memory_bus.read_byte(OBP0);
            let obp1_register = cpu.memory_bus.read_byte(OBP1);
            
            for object in objects {
                // FIX: Use signed arithmetic to properly handle sprites with y < Y_OFFSET (partially off-screen top)
                let start_object_screen_y = (object.y as isize) - Self::Y_OFFSET;
                // FIX: Use signed arithmetic to properly handle sprites with x < 8 (partially off-screen left)
                let start_object_screen_x = (object.x as isize) - 8;
                
                // Determine the object height (8x8 or 8x16 based on LCDC)
                let object_height: isize = if lcdc.obj_size { 16 } else { 8 };
                
                // Calculate which row of the sprite we're on
                let ly_isize = ly_usize as isize;
                let mut tile_row = (ly_isize - start_object_screen_y) as usize;
                
                // Handle y-flip: if y_flip is set, flip the tile row index
                if object.attributes.y_flip {
                    tile_row = (object_height as usize) - 1 - tile_row;
                }
                
                // Get the correct tile for 8x16 objects
                let tile_index = if lcdc.obj_size && tile_row >= 8 {
                    // For 8x16 sprites, if we're in the lower half (rows 8-15), use the next tile
                    (object.tile_index | 1) as usize // Set bit 0 to get the second tile
                } else {
                    object.tile_index as usize
                };
                
                let actual_tile_row = tile_row % 8; // Get row within the tile (0-7)
                
                // FIX: Bounds check to prevent out-of-bounds access
                // Sprite tiles are stored in 0x8000-0x8FFF (384 tiles max)
                if tile_index >= tiles.len() {
                    continue;  // Skip invalid sprite tile
                }
                
                let tile = tiles[tile_index];
                
                for tile_col in 0..8 {
                    let screen_x = start_object_screen_x + tile_col as isize;
                    if screen_x < 0 || screen_x >= GAME_SECTION_WIDTH as isize {
                        continue; // Skip pixels outside the screen bounds
                    }

                    // Handle x-flip: if x_flip is set, flip the tile column index
                    let actual_tile_col = if object.attributes.x_flip {
                        7 - tile_col
                    } else {
                        tile_col
                    };
                    
                    let pixel_value = match tile.pixels[actual_tile_row][actual_tile_col] {
                        TilePixelValue::Zero => 0,
                        TilePixelValue::One => 1,
                        TilePixelValue::Two => 2,
                        TilePixelValue::Three => 3,
                    };
                    
                    if pixel_value == 0 {
                        continue; // Color 0 is transparent for sprites
                    }

                    // FIX: Apply the object palette register to get the actual color
                    // Palette registers map 2-bit pixel values (0-3) to actual colors (0-3)
                    let palette_register = match object.attributes.pallete {
                        ObjectPallete::OBP0 => obp0_register,
                        ObjectPallete::OBP1 => obp1_register,
                    };
                    
                    // Extract the color from the palette: each 2 bits represent one color mapping
                    // Bits 7-6 = color 3, Bits 5-4 = color 2, Bits 3-2 = color 1, Bits 1-0 = color 0
                    let palette_index = ((palette_register >> (pixel_value * 2)) & 0b11) as usize;
                    let color = COLORS[palette_index];
                    
                    // FIX: Respect object priority
                    // If priority bit is set, only render if background is color 0 (white)
                    if object.attributes.priority {
                        // Get the background color at this position
                        let bg_color = cpu.ppu.screen[ly_usize][screen_x as usize];
                        // Only render if background is white (color 0 = 0xFFFFFF)
                        if bg_color != COLORS[0] {
                            continue; // Skip rendering, background has priority
                        }
                    }
                    
                    cpu.ppu.screen[ly_usize][screen_x as usize] = color;
                } 
            } 
        }
    }
}
