use crate::gameboy_core::{
    constants::{
        BGP, INITIAL_PC, LCDC, LY, MEMORY_SIZE, SCX, SCY
    }, interrupts::InterruptType, ppu_components::LcdcRegister, registers_contants
};

pub struct CpuRegisters {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub flags: FlagsRegister,
}

/// Represents the CPU Flags Register, they are stored here as individual boolean fields but in actual hardware 
/// they are packed into a single byte where each bit represents a different flag.
/// Bit 7: Z (Zero flag)
/// Bit 6: N (Subtraction flag)
/// Bit 5: H (Half Carry flag)
/// Bit 4: C (Carry flag)
/// Bits 3-0: Always 0
/// Therefore: `Z N H C 0 0 0 0`
pub struct FlagsRegister {
    /// Zero Flag: True if the last operation resulted in zero
    pub z: bool, // Zero Flag

    /// Half Carry Flag: True if there was a carry from bit 3 to bit 4 in the last operation
    pub h: bool, // Half Carry Flag

    /// Subtract Flag: True if the last operation was a subtraction, false if it was an addition
    pub n: bool, // Subtract Flag

    /// Carry Flag: True if there was a carry from bit 7 to bit 8 in the last operation
    pub c: bool, // Carry Flag
}

pub struct MemoryBus {
    memory: [u8; MEMORY_SIZE],
}

impl CpuRegisters {
    pub fn new() -> Self {
        Self {
            a: 0x01,
            b: 0,
            c: 0x13,
            d: 0,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            sp: 0xFFFE,
            pc: INITIAL_PC,
            flags: FlagsRegister::new(),
        }
    }

    pub fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }

    pub fn increment_pc_twice(&mut self) {
        self.pc = self.pc.wrapping_add(2);
    }

    pub fn increment_sp(&mut self) {
        self.sp = self.sp.wrapping_add(1);
    }

    pub fn set_8bit_register_value(&mut self, register: u8, value: u8) {
        match register {
            0b000 => self.b = value,
            0b111 => self.a = value,
            0b001 => self.c = value,
            0b010 => self.d = value,
            0b011 => self.e = value,
            0b100 => self.h = value,
            0b101 => self.l = value,
            _ => (),
        }
    }

    /// Register r, r'
    /// A 111
    /// B 000
    /// C 001
    /// D 010
    /// E 011
    /// H 100
    /// L 101
    pub fn get_8bit_register_value(&self, register: u8) -> u8 {
        match register {
            0b000 => self.b,
            0b111 => self.a,
            0b001 => self.c,
            0b010 => self.d,
            0b011 => self.e,
            0b100 => self.h,
            0b101 => self.l,
            _ => 0,
        }
    }

    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | self.flags.get_flags_as_u8() as u16
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn increment_hl(&mut self) {
        let hl = self.get_hl();
        let (new_hl, _overflowed) = hl.overflowing_add(1);
        self.set_hl(new_hl);
    }

    pub fn decrement_hl(&mut self) {
        let hl = self.get_hl();
        let (new_hl, _overflowed) = hl.overflowing_sub(1);
        self.set_hl(new_hl);
    }
    
    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.flags.set_flags_from_u8((value & 0b011111111) as u8);
    }

    pub fn set_bc(&mut self, value: u16) {
        let b = (value >> 8) as u8;
        let c = (value & 0b011111111) as u8;
        self.b = b;
        self.c = c;
    }

    pub fn set_de(&mut self, value: u16) {
        let d = (value >> 8) as u8;
        let e = (value & 0b011111111) as u8;
        self.d = d;
        self.e = e;
    }

    pub fn set_hl(&mut self, value: u16) {
        let h = (value >> 8) as u8;
        let l = (value & 0b011111111) as u8;
        self.h = h;
        self.l = l;
    }
}

impl FlagsRegister {
    pub fn new() -> Self {
        Self {
            z: true,
            n: false,
            h: true,
            c: true,
        }
    }

    pub fn get_zero_flag(&self) -> bool {
        self.z
    }

    /// This bit is set if a carry occurred from the last math operation
    pub fn set_c_flag(&mut self, carry: bool) {
        self.c = carry
    }

    /// This bit is set if a carry occurred from the lower nibble (a.k.a the lower four bits) in the last math operation.
    /// We can set this by masking out the upper nibble of both the A register and the value we're adding and testing
    /// if this value is greater than 0xF (0b00001111).
    pub fn calculate_h_flag_on_add(value1: u8, value2: u8) -> bool {
        let value1_lower_nibble = value1 & 0b00001111;
        let value2_lower_nibble = value2 & 0b00001111;

        value1_lower_nibble + value2_lower_nibble > 0xF
    }

    /// Half-carry flag (H): Set if carry from bit 11.
    /// For u16 numbers, we check if there's a carry from bit 11 to bit 12.
    pub fn calculate_h_flag_on_add_u16_numbers(value1: u16, value2: u16) -> bool {
        let value1_bit_11 = value1 & 0x0FFF;
        let value2_bit_11 = value2 & 0x0FFF;

        value1_bit_11 + value2_bit_11 > 0x0FFF
    }

    /// Half-carry flag (H): Set if no borrow from bit 4.
    /// In subtraction, half-carry is set when the lower nibble of value1 is less than the lower nibble of value2
    pub fn calculate_h_flag_on_sub(value1: u8, value2: u8) -> bool {
        (value1 & 0x0F) < (value2 & 0x0F)
    }
    
    /// Sets the z_flag to the provided boolean value
    pub fn set_z_flag(&mut self, z_flag: bool) {
        self.z = z_flag;
    }

    /// This bit is set if and only if the result of an operation is zero
    pub fn set_z_flag_from_u8(&mut self, result: u8) {
        self.z = result == 0;
    }

    /// The same as set_z_flag but for u16 values
    pub fn set_z_flag_from_u16(&mut self, result: u16) {
        self.z = result == 0;
    }
    
    pub(crate) fn set_n_flag(&mut self, value: bool) {
        self.n = value;
    }

    /// Returns the c_flag as u8 to be used in ADC instructions
    pub fn get_c_flag_u8(&self) -> u8 {
        if self.c { 1 } else { 0 }
    }

    pub fn set_h_flag(&mut self, h_flag: bool) {
        self.h = h_flag;
    }

    /// Returns the flags register as a u8 value
    /// Bit 7: Z (Zero flag)
    /// Bit 6: N (Subtraction flag)
    /// Bit 5: H (Half Carry flag)
    /// Bit 4: C (Carry flag)
    /// Bits 3-0: Always 0
    pub fn get_flags_as_u8(&self) -> u8 {
        let mut value = 0u8;
        
        if self.z {
            value |= 0b10000000; // Set bit 7
        }
        if self.n {
            value |= 0b01000000; // Set bit 6
        }
        if self.h {
            value |= 0b00100000; // Set bit 5
        }
        if self.c {
            value |= 0b00010000; // Set bit 4
        }
        
        value
    }
    
    pub(crate) fn set_flags_from_u8(&mut self, value: u8) {
        self.z = (value & 0b10000000) != 0;
        self.n = (value & 0b01000000) != 0;
        self.h = (value & 0b00100000) != 0;
        self.c = (value & 0b00010000) != 0;
    }
}

impl MemoryBus {
    pub fn new() -> Self {
        Self {
            memory: [0; MEMORY_SIZE],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        if address == LY {
            // LY register always returns the current scanline (for simplicity, we return 0 here)
            return 0x90;
        }

        self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn copy_from_binary(&mut self, rom_binary: Vec<u8>) {
        let start_ram_address = 0 as usize;
        self.memory[start_ram_address..(start_ram_address + rom_binary.len())]
            .copy_from_slice(&rom_binary);
    }

    /// Gets a reference to the VRAM (Video RAM) region
    pub fn get_vram(&self) -> &[u8] {
        &self.memory[0x8000..=0x9FFF]
    }

    /// Gets a reference to the VRAM tile data region which covers addressess $8000-$97FF
    pub fn get_vram_tile_data(&self) -> &[u8] {
        &self.memory[0x8000..=0x97FF]
    }

    /// Gets a mutable reference to the VRAM region
    pub fn get_vram_mut(&mut self) -> &mut [u8] {
        &mut self.memory[0x8000..=0x9FFF]
    }

    /// Get LCDC register value
    pub fn get_lcdc_register(&self) -> u8 {
        self.read_byte(LCDC)
    }

    /// Set LCDC register value
    pub fn set_lcdc_register(&mut self, value: u8) {
        self.write_byte(LCDC, value);
    }

    /// Returns the background tile map area from 9800-9BFF or 9C00-9FFF based on the bg_tile_map_area flag in the LCDC register.
    pub fn get_bg_tile_map(&self, lcdc_register: &LcdcRegister) -> &[u8] {
        let (start, end) = lcdc_register.get_bg_tiles_map_area_address_range();
        &self.memory[start as usize..=end as usize]
    }

    /// Get SCY register value
    pub fn get_scy_register(&self) -> u8 {
        self.read_byte(SCY)
    }

    /// Get SCX register value
    pub fn get_scx_register(&self) -> u8 {
        self.read_byte(SCX)
    }

    /// Set SCY register value
    pub fn set_scy_register(&mut self, value: u8) {
        self.write_byte(SCY, value);
    }

    /// Set SCX register value
    pub fn set_scx_register(&mut self, value: u8) {
        self.write_byte(SCX, value);
    }

    pub(crate) fn set_bgp_register(&mut self, value: u8) {
        self.write_byte(BGP, value);
    }

    /// Divider Register (DIV) - increments at a rate of 16384 Hz.
    /// Therefore, it increments every 256 CPU cycles, because the CPU runs at 4.194304 MHz.
    /// The math is 4,194,304 Hz / 16,384 Hz = 256 cycles.
    pub(crate) fn get_div_register(&self) -> u8 {
        self.read_byte(registers_contants::DIV)
    }
    
    /// Get the TMA register value, that is located at address 0xFF06
    /// Timer Modulo (TMA) - when TIMA overflows (from 0xFF to 0x00), it is reloaded with the value in TMA.
    pub(crate) fn get_tma_register(&self) -> u8 {
        self.read_byte(registers_contants::TMA)
    }
    
    /// Get the TAC register value, that is located at address 0xFF07
    /// Timer Control (TAC) - controls the timer's operation, including its speed and whether it is enabled.
    pub(crate) fn get_tac_register(&self) -> u8 {
        self.read_byte(registers_contants::TAC)
    }
    
    /// Get the TIMA register value, that is located at address 0xFF05
    /// Timer Counter (TIMA) - increments at a rate determined by the TAC register.
    pub(crate) fn get_tima_register(&self) -> u8 {
        self.read_byte(registers_contants::TIMA)
    }
    
    pub(crate) fn set_div_register(&mut self, value: u8) {
        self.write_byte(registers_contants::DIV, value);
    }

    pub(crate) fn set_tima_register(&mut self, value: u8) {
        self.write_byte(registers_contants::TIMA, value);
    }
    
    /// Sets or clears the timer interrupt flag in the IF register.
    /// The IF register controls which interrupts are being requested.
    pub(crate) fn update_timer_flag_in_if_register(&mut self, interrupt_type: InterruptType, value: bool) {
        let mut if_register = self.read_byte(registers_contants::IF);
        if value {
            match interrupt_type {
                InterruptType::VBlank => if_register |= 0b00000001, // Set bit 0 to request V-Blank interrupt
                InterruptType::LCD => if_register |= 0b00000010, // Set bit 1 to request LCD STAT interrupt
                InterruptType::Timer => if_register |= 0b00000100, // Set bit 2 to request Timer interrupt
                InterruptType::Serial => if_register |= 0b00001000, // Set bit 3 to request Serial interrupt
                InterruptType::Joypad => if_register |= 0b00010000, // Set bit 4 to request Joypad interrupt 
            }
        } else {
            match interrupt_type {
                InterruptType::VBlank => if_register &= 0b11111110, // Clear bit 0 to clear V-Blank interrupt
                InterruptType::LCD => if_register &= 0b11111101, // Clear bit 1 to clear LCD STAT interrupt
                InterruptType::Timer => if_register &= 0b11111011, // Clear bit 2 to clear Timer interrupt
                InterruptType::Serial => if_register &= 0b11110111, // Clear bit 3 to clear Serial interrupt
                InterruptType::Joypad => if_register &= 0b11101111, // Clear bit 4 to clear Joypad interrupt
            }
        }

        self.write_byte(registers_contants::IF, if_register);
    }
}
