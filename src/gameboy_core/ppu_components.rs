#[derive(Copy,Clone)]
pub enum TilePixelValue {
    Zero,
    One,
    Two,
    Three,
}

#[derive(Copy,Clone)]
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