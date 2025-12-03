use crate::gameboy_core::cpu_components::FlagsRegister;

pub trait CpuAddOperation {
    fn add_u8_as_signed(&self, value: u8) -> (u16, bool, bool);
}

impl CpuAddOperation for u16 {
    /// Adds a signed 8-bit value (u8) to the current u16 value, 
    /// parsing the u8 to a signed integer i16 which will handle negative offsets correctly.
    /// Carry and Half Carry flags are calculated based on the unsigned addition of the lower byte of the u16 value and the u8 value.
    /// Returns a tuple containing the resulting u16 value, Carry flag (C) and Half Carry flag (H).
    fn add_u8_as_signed(&self, value: u8) -> (u16, bool, bool) {
        // The Carry (C) and Half Carry (H) flags are calculated
        // based on the unsigned addition of the low byte of the u16 value (self) and the u8 value,
        // checking for carries out of bit 7 (C) and bit 3 (H).
        let lower_part_of_u16 = (*self & 0x00FF) as u8;
        let (_, c_flag) = lower_part_of_u16.overflowing_add(value);
        let h_flag = FlagsRegister::calculate_h_flag_on_add(lower_part_of_u16, value);

        // Convert the 8-bit unsigned immediate value (u8: 0xFF) into a signed 8-bit integer (i8: -1).
        let value_u8_signed = value as i8;

        // Sign-extend the offset to a 16-bit signed integer (i16: 0xFFFF).
        // We can´t convert the u8 directly to i16 because Rust will do zero extension instead of sign extension.
        // Sign extension means that the sign bit (most significant bit) is replicated to fill the higher bits which will represent
        // the same negative value in a larger bit-width.
        let offset_signed: i16 = value_u8_signed as i16;

        // Convert the resulting 16-bit signed offset to its unsigned representation (u16)
        // to allow safe wrapping addition with the u16 self value.
        let offset_u16 = offset_signed as u16;

        // Perform the 16-bit addition. The Game Boy SP wraps around 16 bits.
        let result = self.wrapping_add(offset_u16);

        (result, c_flag, h_flag)
    }
}
