use crate::gameboy_core::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct Ppu {
    pub screen: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT] // 144 rows of 160 pixels
}

impl Ppu {
    pub(crate) fn new() -> Self {
        Self { screen: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT] }
    }
}