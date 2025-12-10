use crate::gameboy_core::cpu_components::MemoryBus;

pub struct Timer {
    /// TODO: remove the div, tima, tma, tac fields since they are already in memory bus

    /// Divider Register (DIV) - increments at a rate of 16384 Hz.
    /// Therefore, it increments every 256 CPU cycles, because the CPU runs at 4.194304 MHz.
    /// The math is 4,194,304 Hz / 16,384 Hz = 256 cycles.
    pub div: u8,
    /// Timer Counter (TIMA) - increments at a rate determined by the TAC register.
    pub tima: u8,
    /// Timer Modulo (TMA) - when TIMA overflows (from 0xFF to 0x00), it is reloaded with the value in TMA.
    pub tma: u8,
    /// Timer Control (TAC) - controls the timer's operation, including its speed and whether it is enabled.
    pub tac: u8,
    /// Number of cycles executed since last increment for the DIV register
    pub cycles_executed_div: u16,
    /// Number of cycles executed since last increment for the TIMA register
    pub cycles_executed_tima: u16,
}

pub enum InterruptRequested {
    Yes,
    No,
}

impl Timer {
    pub(crate) fn new() -> Self {
        Self {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            cycles_executed_div: 0,
            cycles_executed_tima: 0,
        }
    }

    /// Update the DIV and TIMA registers based on the number of cycles executed since the last instruction.
    /// Returns whether an interrupt was requested due to TIMA overflow.
    pub fn update(&mut self, cycles_before: u64, cycles_after: u64, memory: &mut MemoryBus,) -> InterruptRequested {
        let cycles_of_last_instruction: u8 = (cycles_after - cycles_before) as u8;
        self.update_div(cycles_of_last_instruction, memory);
        self.update_tima(cycles_of_last_instruction, memory)
    }

    /// Update the DIV register based on the number of cycles executed since the last instruction.
    /// If total cycles exceed 256, increment DIV and reset the cycle counter.
    fn update_div(&mut self, cycles_of_last_instruction: u8, memory: &mut MemoryBus) {
        let total_cycles = self.cycles_executed_div + cycles_of_last_instruction as u16;


        if total_cycles >= 256 {
            self.div = self.div.wrapping_add(1);
            self.cycles_executed_div = total_cycles - 256;
            memory.write_byte(0xFF04, self.div); // Update DIV register in memory
        } else {
            self.cycles_executed_div = total_cycles;
        }
    }

    /// Increment the TIMA register every n cycles (where n is determined by bits 1-0 of the TAC register).
    /// If TIMA overflows (since it's an u8 register it means going from 0xFF to 0x00), it is reset to the value specified in TMA register
    /// and an interrupt is requested.
    fn update_tima(&mut self, cycles_of_last_instruction: u8, memory: &mut MemoryBus,) -> InterruptRequested {
        self.tac = memory.get_tac_register();
        let timer_enabled = (self.tac & 0b00000100) != 0;
        if !timer_enabled {
            return InterruptRequested::No; 
        }

        let input_clock_select = self.tac & 0b00000011;
        let tima_increment_threshold = Self::get_tima_increment_threshould(input_clock_select);
        
        self.tma = memory.get_tma_register();
        self.tima = memory.get_tima_register();

        let total_cycles = self.cycles_executed_tima + cycles_of_last_instruction as u16;
        
        if total_cycles >= tima_increment_threshold {
            self.cycles_executed_tima = total_cycles - tima_increment_threshold;

            let (_, tima_overflowed) = self.tima.overflowing_add(1);
            
            if tima_overflowed {
                self.tima = self.tma;
                memory.write_byte(0xFF05, self.tima); // Update TIMA register in memory
                return InterruptRequested::Yes;
            } else {
                self.tima = self.tima.wrapping_add(1);
                memory.write_byte(0xFF05, self.tima); // Update TIMA register in memory
            }
        } else {
            self.cycles_executed_tima = total_cycles;
        }
        
        InterruptRequested::No
    }
    
    /// Get the threshold of cycles for TIMA increment based on the TAC input clock select bits.
    /// The thresholds are returned in terms of Master Clock cycles (4.194304 MHz).
    /// - 00: 4096 Hz -> 1024 cycles
    /// - 01: 262144 Hz -> 16 cycles
    /// - 10: 65536 Hz -> 64 cycles
    /// - 11: 16384 Hz -> 256 cycles
    fn get_tima_increment_threshould(input_clock_select: u8) -> u16 {
        match input_clock_select {
            0b00 => 1024,   // 4096 Hz - 4,194,304 Hz / 4,096 Hz = 1024 cycles
            0b01 => 16,     // 262144 Hz - 4,194,304 Hz / 262,144 Hz = 16 cycles
            0b10 => 64,    // 65536 Hz - 4,194,304 Hz / 65,536 Hz = 64 cycles
            0b11 => 256,    // 16384 Hz - 4,194,304 Hz / 16,384 Hz = 256 cycles
            _ => 256,      // Default case (should not occur)
        }
    }
}
