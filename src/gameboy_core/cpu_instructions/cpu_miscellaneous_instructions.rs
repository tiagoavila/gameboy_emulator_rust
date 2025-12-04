pub trait CpuMiscellaneousInstructions {
    fn daa_sub(&mut self, bits_4_7: u8, bits_0_3: u8, h_flag: bool, c_flag: bool);
    fn daa_add(&mut self, bits_4_7: u8, bits_0_3: u8, h_flag: bool, c_flag: bool);
    fn daa(&mut self);
    fn cpl(&mut self);
    fn ei(&mut self);
    fn nop(&mut self);
    fn di(&mut self);
    fn ccf(&mut self);
}

impl CpuMiscellaneousInstructions for crate::gameboy_core::cpu::Cpu {
    /// No Operation - Do nothing for one CPU cycle.
    fn nop(&mut self) {
        return;
    }

    /// This instruction disables interrupts but not immediately. Interrupts are disabled after instruction after DI is executed.
    fn di(&mut self) {
        self.di_instruction_pending = true;
    }

    fn ei(&mut self) {
        self.ei_instruction_pending = true;
    }

    /// Flips the carry flag CY. H and N flags are reset.
    fn ccf(&mut self) {
        self.registers.flags.c = !self.registers.flags.c;
        self.registers.flags.h = false;
        self.registers.flags.n = false;
    }

    // CORRECT IMPLEMENTATION
    fn daa(&mut self) {
        let mut adjustment = 0;
        let mut set_carry = false;

        if !self.registers.flags.n {
            // After addition
            if self.registers.flags.h || (self.registers.a & 0x0F) > 9 {
                adjustment += 0x06;
            }
            if self.registers.flags.c || self.registers.a > 0x99 {
                adjustment += 0x60;
                set_carry = true;
            }
            self.registers.a = self.registers.a.wrapping_add(adjustment);
        } else {
            // After subtraction
            if self.registers.flags.h {
                adjustment += 0x06;
            }
            if self.registers.flags.c {
                adjustment += 0x60;
            }
            self.registers.a = self.registers.a.wrapping_sub(adjustment);
        }

        self.registers.flags.set_z_flag_from_u8(self.registers.a);
        self.registers.flags.set_h_flag(false); // Always cleared
        self.registers.flags.set_c_flag(set_carry);
    }

    // fn daa(&mut self) {
    //     let bits_4_7 = (self.registers.a & 0xF0) >> 4;
    //     let bits_0_3 = self.registers.a & 0x0F;
    //     let h_flag = self.registers.flags.h;
    //     let c_flag = self.registers.flags.c;

    //     if !self.registers.flags.n {
    //         self.daa_add(bits_4_7, bits_0_3, h_flag, c_flag);
    //     } else {
    //         self.daa_sub(bits_4_7, bits_0_3, h_flag, c_flag);
    //     }
    // }

    fn daa_add(&mut self, bits_4_7: u8, bits_0_3: u8, h_flag: bool, mut c_flag: bool) {
        let mut adjustment: u8 = 0;

        if !self.registers.flags.c {
            if h_flag && bits_0_3 <= 0x04 {
                if bits_4_7 <= 0x09 {
                    adjustment += 0x06;
                } else {
                    adjustment += 0x66;
                    c_flag = true;
                }
            }

            if !h_flag {
                if bits_4_7 <= 0x08 && bits_0_3 >= 0xA {
                    adjustment += 0x06;
                } else {
                    if bits_0_3 <= 0x09 {
                        adjustment += 0x60;
                        c_flag = true;
                    } else {
                        adjustment += 0x66;
                        c_flag = true;
                    }
                }
            }
        } else {
            if h_flag && bits_4_7 <= 0x03 && bits_0_3 <= 0x03 {
                adjustment += 0x66;
                c_flag = true;
            } else {
                if bits_4_7 <= 0x02 {
                    if bits_0_3 <= 0x09 {
                        adjustment += 0x60;
                        c_flag = true;
                    } else {
                        adjustment += 0x66;
                        c_flag = true;
                    }
                }
            }
        }

        self.registers.a = self.registers.a.wrapping_add(adjustment);
        self.registers.flags.set_c_flag(c_flag);
        self.registers.flags.set_h_flag(false);
        self.registers.flags.set_z_flag_from_u8(self.registers.a);
    }

    fn daa_sub(&mut self, bits_4_7: u8, bits_0_3: u8, h_flag: bool, mut c_flag: bool) {
        let mut adjustment: u8 = 0;

        if c_flag {
            if h_flag && bits_4_7 >= 0x06 && bits_0_3 >= 0x06 {
                adjustment = 0x9A;
                c_flag = true;
            } else if bits_4_7 >= 0x07 && bits_0_3 <= 0x09 {
                adjustment = 0xA0;
                c_flag = true;
            }
        } else if h_flag && bits_4_7 <= 0x08 && bits_0_3 >= 0x06 {
            adjustment = 0xFA;
        }

        self.registers.a = self.registers.a.wrapping_sub(adjustment);
        self.registers.flags.set_c_flag(c_flag);
        self.registers.flags.set_h_flag(false);
        self.registers.flags.set_z_flag_from_u8(self.registers.a);
    }

    fn cpl(&mut self) {
        self.registers.a = !self.registers.a;
        self.registers.flags.n = true;
        self.registers.flags.h = true;
    }
}
