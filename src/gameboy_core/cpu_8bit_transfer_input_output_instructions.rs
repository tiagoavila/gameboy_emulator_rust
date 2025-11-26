use crate::gameboy_core::{
    constants::START_ADDRESS_FOR_LOAD_INSTRUCTIONS,
    cpu::Cpu,
};

pub trait Cpu8BitTransferInputOutputInstructions {
    fn ld_r8_imm8(&mut self, opcode: u8);
    fn ld_r8_r8(&mut self, opcode: u8);
    fn ld_r8_hl(&mut self, opcode: u8);
    fn ld_hl_r8(&mut self, opcode: u8);
    fn ld_hl_imm8(&mut self);
    fn ld_a_bc(&mut self);
    fn ld_a_de(&mut self);
    fn ld_a_c(&mut self);
    fn ld_c_a(&mut self);
    fn ld_a_imm8(&mut self);
    fn ld_imm8_a(&mut self);
    fn ld_a_imm16(&mut self);
    fn ld_imm16_a(&mut self);
    fn ld_a_hli(&mut self);
    fn ld_a_hld(&mut self);
    fn ld_bc_a(&mut self);
    fn ld_de_a(&mut self);
    fn ld_hli_a(&mut self);
    fn ld_hld_a(&mut self);
}

impl Cpu8BitTransferInputOutputInstructions for Cpu {
    /// Load the 8-bit immediate value into the specified 8-bit register.
    fn ld_r8_imm8(&mut self, opcode: u8) {
        let destination = Cpu::get_destination_register(opcode);
        let imm8 = self.get_imm8();
        self.registers.set_8bit_register_value(destination, imm8);
        self.registers.increment_pc();
    }

    fn ld_r8_r8(&mut self, opcode: u8) {
        let destination = Cpu::get_destination_register(opcode);
        let source = Cpu::get_source_register(opcode);

        if destination == source {
            return; // No operation needed if both registers are the same
        }

        let value = self.registers.get_8bit_register_value(source);
        self.registers.set_8bit_register_value(destination, value);
    }

    /// Load the contents of register HL into 8-bit register.
    fn ld_r8_hl(&mut self, opcode: u8) {
        let destination = Cpu::get_destination_register(opcode);
        let value = self.get_memory_value_at_hl();
        self.registers.set_8bit_register_value(destination, value);
    }

    /// Stores the contents of register r in memory specified by register pair HL.
    fn ld_hl_r8(&mut self, opcode: u8) {
        let source = Cpu::get_source_register(opcode);
        let value = self.registers.get_8bit_register_value(source);
        let hl = self.registers.get_hl();
        self.memory_bus.write_byte(hl, value);
    }

    /// Loads 8-bit immediate data n into memory specified by register pair HL.
    fn ld_hl_imm8(&mut self) {
        let imm8 = self.get_imm8();
        let hl = self.registers.get_hl();
        self.memory_bus.write_byte(hl, imm8);
        self.registers.increment_pc();
    }

    /// Loads the contents specified by the contents of register pair BC into register A.
    fn ld_a_bc(&mut self) {
        let bc = self.registers.get_bc();
        let value = self.memory_bus.read_byte(bc);
        self.registers.a = value;
    }

    /// Loads the contents specified by the contents of register pair DE into register A.
    fn ld_a_de(&mut self) {
        let de = self.registers.get_de();
        let value = self.memory_bus.read_byte(de);
        self.registers.a = value;
    }

    /// Loads into register A the contents of the internal RAM, port register, or mode register at the address in
    /// the range FF00h-FFFFh specified by register C.
    fn ld_a_c(&mut self) {
        let c_register_value = self.registers.c as u16;
        let ram_address = START_ADDRESS_FOR_LOAD_INSTRUCTIONS + c_register_value;
        let value = self.memory_bus.read_byte(ram_address);
        self.registers.a = value;
    }

    /// Loads the contents of register A in the internal RAM, port register, or mode register at the address in the
    /// range FF00h-FFFFh specified by register C.
    fn ld_c_a(&mut self) {
        let c_register_value = self.registers.c as u16;
        let ram_address = START_ADDRESS_FOR_LOAD_INSTRUCTIONS + c_register_value;
        self.memory_bus.write_byte(ram_address, self.registers.a);
    }

    /// Loads into register A the contents of the internal RAM, port register, or mode register at the address in the range FF00h-FFFFh
    /// specified by the 8-bit immediate operand n.
    /// Note, however, that a 16-bit address should be specified for the mnemonic portion of n, because only the lower-order 8 bits are
    /// automatically reflected in the machine language.
    fn ld_a_imm8(&mut self) {
        let imm8 = self.get_imm8() as u16;
        let address_to_read_from = START_ADDRESS_FOR_LOAD_INSTRUCTIONS + imm8;
        let value = self.memory_bus.read_byte(address_to_read_from);
        self.registers.a = value;
        self.registers.increment_pc();
    }

    /// Loads the contents of register A to the internal RAM, port register, or mode register at the address in the range FF00h-FFFFh
    /// specified by the 8-bit immediate operand n.
    /// Note, however, that a 16-bit address should be specified for the mnemonic portion of n, because only the
    /// lower-order 8 bits are automatically reflected in the machine language.
    fn ld_imm8_a(&mut self) {
        let imm8 = self.get_imm8() as u16;
        let address_to_write = START_ADDRESS_FOR_LOAD_INSTRUCTIONS + imm8;
        self.memory_bus
            .write_byte(address_to_write, self.registers.a);
        self.registers.increment_pc();
    }

    /// Loads into register A the contents of the internal RAM or register specified by 16-bit immediate operand nn.
    fn ld_a_imm16(&mut self) {
        let imm16 = self.get_imm16();
        let value = self.memory_bus.read_byte(imm16);
        self.registers.a = value;
        self.registers.increment_pc_twice();
    }

    /// Loads the contents of register A to the internal RAM or register specified by 16-bit immediate operand nn.
    fn ld_imm16_a(&mut self) {
        let imm16 = self.get_imm16();
        self.memory_bus.write_byte(imm16, self.registers.a);
        self.registers.increment_pc_twice();
    }

    /// Loads in register A the contents of memory specified by the contents of register pair HL and simultaneously increments the contents of HL.
    fn ld_a_hli(&mut self) {
        let value = self.get_memory_value_at_hl();
        self.registers.a = value;
        self.registers.increment_hl();
    }

    /// Loads in register A the contents of memory specified by the contents of register pair HL and simultaneously decrements the contents of HL.
    /// Example: When HL = 8A5Ch and (8A5Ch) = 3Ch,
    /// LD A, (HLD) ; A ← 3Ch, HL ← 8A5Bh
    fn ld_a_hld(&mut self) {
        let value = self.get_memory_value_at_hl();
        self.registers.a = value;
        self.registers.decrement_hl();
    }

    /// Stores the contents of register A in the memory specified by register pair BC.
    /// Example: When BC = 205Fh and A = 3Fh,
    /// LD (BC) , A ; (205Fh) ← 3Fh
    fn ld_bc_a(&mut self) {
        let bc = self.registers.get_bc();
        self.memory_bus.write_byte(bc, self.registers.a);
    }

    /// Stores the contents of register A in the memory specified by register pair DE.
    /// Example: When DE = 205Ch and A = 00h,
    /// LD (DE) , A ; (205Ch) ← 00h
    fn ld_de_a(&mut self) {
        let de = self.registers.get_de();
        self.memory_bus.write_byte(de, self.registers.a);
    }

    /// Stores the contents of register A in the memory specified by register pair HL and simultaneously increments the contents of HL.
    /// Example: When HL = FFFFh and A = 56h,
    /// LD (HLI), A ; (0xFFFF) ← 56h, HL = 0000h
    fn ld_hli_a(&mut self) {
        let hl = self.registers.get_hl();
        self.memory_bus.write_byte(hl, self.registers.a);
        self.registers.increment_hl();
    }

    /// Stores the contents of register A in the memory specified by register pair HL and simultaneously decrements the contents of HL.
    /// Example: HL = 4000h and A = 5h,
    /// LD (HLD), A ; (4000h) ← 5h, HL = 3FFFh
    fn ld_hld_a(&mut self) {
        let hl = self.registers.get_hl();
        self.memory_bus.write_byte(hl, self.registers.a);
        self.registers.decrement_hl();
    }
}
