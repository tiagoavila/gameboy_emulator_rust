use core::fmt;

use crate::gameboy_core::{constants::*, cpu_components};

#[derive(Copy,Clone,fmt::Debug)]
pub enum TilePixelValue {
    Zero,
    One,
    Two,
    Three,
}

#[derive(Copy,Clone,fmt::Debug)]
pub struct Tile {
    pub pixels: [[TilePixelValue; 8]; 8],
}

impl Tile {
    pub fn new() -> Self {
        Self {
            pixels: [[TilePixelValue::Zero; 8]; 8],
        }
    }
}

pub struct LcdcRegister {
    /// This bit controls whether the LCD is on and the PPU is active. 
    /// Setting it to 0 turns both off, which grants immediate and full access to VRAM, OAM, etc.
    pub lcd_ppu_enabled: bool,

    /// This bit controls which background map the Window uses for rendering.
    /// When it’s clear (0), the $9800 tilemap is used, otherwise it’s the $9C00 one.
    pub window_tile_map_area: bool,
    
    /// This bit controls whether the window shall be displayed or not. This bit is overridden on DMG by bit 0 if that bit is clear.
    pub window_enable: bool,
    
    /// This bit controls which addressing mode the BG and Window use to pick tiles. 
    /// Objects (sprites) aren’t affected by this, and will always use the $8000 addressing mode.
    pub bg_window_tiles: bool,

    /// This bit works similarly to window_tile_map_area: if the bit is clear (0),
    ///  the BG uses tilemap from $9800 to $9BFF, otherwise tilemap $9C00 to $9FFF.
    pub bg_tile_map_area: bool,
    
    /// This bit controls the size of all objects (1 tile or 2 stacked vertically). 
    /// Be cautious when changing object size mid-frame. Changing from 8×8 to 8×16 pixels mid-frame within 8 scanlines 
    /// of the bottom of an object causes the object’s second tile to be visible for the rest of those 8 lines.
    /// If the size is changed during mode 2 or 3, remnants of objects in range could “leak” into the other tile and cause artifacts.
    pub obj_size: bool,

    /// This bit toggles whether objects are displayed or not. 
    /// This can be toggled mid-frame, for example to avoid objects being displayed on top of a status bar or text box.
    pub obj_enable: bool,

    /// BG and Window display:
    /// When Bit 0 is cleared, both background and window become blank (white), and the Window Display Bit is ignored in that case.
    /// Only objects may still be displayed (if enabled in Bit 1).
    pub bg_window_enable: bool
}

impl LcdcRegister {
    /// Reads the LCDC register from the memory bus and returns an instance of LcdcRegister with the corresponding flags set.
    pub fn get_lcdc_register(memory_bus: &cpu_components::MemoryBus) -> Self {
        let lcdc_value = memory_bus.get_lcdc_register();
        
        Self {
            lcd_ppu_enabled: (lcdc_value & 0b1000_0000) != 0,
            window_tile_map_area: (lcdc_value & 0b0100_0000) != 0,
            window_enable: (lcdc_value & 0b0010_0000) != 0,
            bg_window_tiles: (lcdc_value & 0b0001_0000) != 0,
            bg_tile_map_area: (lcdc_value & 0b0000_1000) != 0,
            obj_size: (lcdc_value & 0b0000_0100) != 0,
            obj_enable: (lcdc_value & 0b0000_0010) != 0,
            bg_window_enable: (lcdc_value & 0b0000_0001) != 0,
        }
    }
    /// Returns the memory address range the BG and Window use to pick up tiles.
    /// When bg_window_tiles is true, returns the address range from 0x8000 to 0x8FFF.
    /// When false, returns the address range from 0x8800 to 0x97FF.
    pub fn get_bg_window_tiles_area_address_range(&self) -> (u16, u16) {
        if self.bg_window_tiles {
            return (BG_WINDOW_DATA_AREA_0_START, BG_WINDOW_DATA_AREA_0_END);
        }
        
        (BG_WINDOW_DATA_AREA_1_START, BG_WINDOW_DATA_AREA_1_END)
    }

    /// Returns the memory address range the Window Tile Map is located at. So it returns where the map for Window is.
    /// When window_tile_map_area is true, returns the address range from 0x9C00 to 0x9FFF.
    /// When false, returns the address range from 0x9800 to 0x9BFF.
    pub fn get_window_tile_map_area_address_range(&self) -> (u16, u16) {
        if self.window_tile_map_area {
            return (TILE_MAP_AREA_1_START, TILE_MAP_AREA_1_END);
        }
        
        (TILE_MAP_AREA_0_START, TILE_MAP_AREA_0_END)
    }
    
    /// Returns the memory address range the BG Tile Map is located at. So it returns where the map for BG is.
    /// When bg_tile_map_area is true, returns the address range from 0x9C00 to 0x9FFF.
    /// When false, returns the address range from 0x9800 to 0x9BFF.
    pub fn get_bg_tiles_map_area_address_range(&self) -> (u16, u16) {
        if self.bg_tile_map_area {
            return (TILE_MAP_AREA_1_START, TILE_MAP_AREA_1_END);
        }
        
        (TILE_MAP_AREA_0_START, TILE_MAP_AREA_0_END)
    }
}