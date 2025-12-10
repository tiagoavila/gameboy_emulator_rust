pub trait CpuMiscellaneousInstructions {
    fn stop(&mut self);
    fn halt(&mut self);
    fn daa(&mut self);
    fn cpl(&mut self);
    fn scf(&mut self);
    fn ei(&mut self);
    fn nop(&mut self);
    fn di(&mut self);
    fn ccf(&mut self);
}

impl CpuMiscellaneousInstructions for crate::gameboy_core::cpu::Cpu {
    /// No Operation - Do nothing for one CPU cycle.
    fn nop(&mut self) {
        self.increment_4_clock_cycles();
        return;
    }

    /// This instruction disables interrupts but not immediately. Interrupts are disabled after instruction after DI is executed.
    fn di(&mut self) {
        self.di_instruction_pending = true;
        self.increment_4_clock_cycles();
    }

    fn ei(&mut self) {
        self.ei_instruction_pending = true;
        self.increment_4_clock_cycles();
    }

    /// Flips the carry flag CY. H and N flags are reset.
    fn ccf(&mut self) {
        self.registers.flags.c = !self.registers.flags.c;
        self.registers.flags.h = false;
        self.registers.flags.n = false;
        self.increment_4_clock_cycles();
    }

    /// Adjusts register A to form a correct BCD representation after a binary addition or subtraction.
    /// The adjustment is based on the values of the N, H, and C flags.
    /// If the previous operation was an addition (N flag is reset):
    /// - If the H flag is set or the lower nibble of A is greater than 9, add 0x06 to A.
    /// - If the C flag is set or A is greater than 0x99, add 0x60 to A and set the C flag. 
    /// If the previous operation was a subtraction (N flag is set):
    /// - If the H flag is set, subtract 0x06 from A.
    /// - If the C flag is set, subtract 0x60 from A.
    /// After the adjustment, the Z flag is set if A is zero, and the H flag is cleared.
    fn daa(&mut self) {
        let mut adjustment = 0u8;
        
        if !self.registers.flags.n {
            // After addition (ADD, ADC, INC)
            if self.registers.flags.h || (self.registers.a & 0x0F) > 0x09 {
                adjustment += 0x06;
            }
            if self.registers.flags.c || self.registers.a > 0x99 {
                adjustment += 0x60;
                self.registers.flags.set_c_flag(true);
            }
            self.registers.a = self.registers.a.wrapping_add(adjustment);
        } else {
            // After subtraction (SUB, SBC, DEC)
            if self.registers.flags.h {
                adjustment = (adjustment as i16 - 0x06) as u8;
            }
            if self.registers.flags.c {
                adjustment = (adjustment as i16 - 0x60) as u8;
                self.registers.flags.set_c_flag(true);
            }
            self.registers.a = self.registers.a.wrapping_add(adjustment);
        }
        
        self.registers.flags.set_z_flag_from_u8(self.registers.a);
        self.registers.flags.set_h_flag(false); // H flag always cleared after DAA
        self.increment_4_clock_cycles();
    }

    /// Inverts all bits in register A. N and H flags are set.
    fn cpl(&mut self) {
        self.registers.a = !self.registers.a;
        self.registers.flags.n = true;
        self.registers.flags.h = true;
        self.increment_4_clock_cycles();
    }
    
    /// Sets the carry flag CY. H and N flags are reset.
    fn scf(&mut self) {
        self.registers.flags.set_c_flag(true);
        self.registers.flags.h = false;
        self.registers.flags.n = false;
        self.increment_4_clock_cycles();
    }

    fn halt(&mut self) {
        self.increment_4_clock_cycles();
    }
    
    fn stop(&mut self) {
        self.increment_4_clock_cycles();
    }
}
