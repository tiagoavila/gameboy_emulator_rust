use crate::gameboy_core::{
    constants::{
        JOYPAD_INTERRUPT_HANDLER_ADDRESS, LCD_STAT_INTERRUPT_HANDLER_ADDRESS,
        SERIAL_INTERRUPT_HANDLER_ADDRESS, TIMER_INTERRUPT_HANDLER_ADDRESS,
        VBLANK_INTERRUT_HANDLER_ADDRESS,
    },
    cpu::Cpu,
    registers_contants::{IE, IF},
};

pub enum InterruptType {
    VBlank,
    LCD,
    Timer,
    Serial,
    Joypad,
}

pub struct InterruptsHandler;

/// Used to represent both IE and IF registers, since they have the same bit layout.
pub struct InterruptStatus {
    vblank: bool,
    lcd: bool,
    timer: bool,
    serial: bool,
    joypad: bool,
}

impl InterruptsHandler {
    /// Checks if any interrupts are requested by checking the IME, IF and IE registers.
    /// If an interrupt is requested, it handles it by calling the appropriate interrupt handler.
    pub fn handle(cpu: &mut Cpu) {
        if !cpu.ime {
            if cpu.is_halt_mode {
                Self::check_pending_interrupts_to_exit_halt_mode(cpu);
            }

            return;
        }

        let if_register = cpu.memory_bus.read_byte(IF);
        let ie_register = cpu.memory_bus.read_byte(IE);

        let if_register_flags = Self::get_register_flag_values(if_register);
        let ie_register_flags = Self::get_register_flag_values(ie_register);

        // The order of the if statements is important, as it defines the priority of the interrupts.
        // Priority order: VBlank > LCD > Timer > Serial > Joypad

        if if_register_flags.vblank && ie_register_flags.vblank {
            Self::do_before_handling_interrupt(cpu, InterruptType::VBlank);
            Self::do_handle_interrupt(cpu, InterruptType::VBlank);
        }

        if if_register_flags.lcd && ie_register_flags.lcd {
            Self::do_before_handling_interrupt(cpu, InterruptType::LCD);
            Self::do_handle_interrupt(cpu, InterruptType::LCD);
        }

        if if_register_flags.timer && ie_register_flags.timer {
            Self::do_before_handling_interrupt(cpu, InterruptType::Timer);
            Self::do_handle_interrupt(cpu, InterruptType::Timer);
        }

        if if_register_flags.serial && ie_register_flags.serial {
            Self::do_before_handling_interrupt(cpu, InterruptType::Serial);
            Self::do_handle_interrupt(cpu, InterruptType::Serial);
        }

        if if_register_flags.joypad && ie_register_flags.joypad {
            Self::do_before_handling_interrupt(cpu, InterruptType::Joypad);
            Self::do_handle_interrupt(cpu, InterruptType::Joypad);
        }

        // Setting IME to its previous value is handled by the interrupt handler itself, using the RETI instruction or by calling EI instruction.
        // Return from an interrupt routine can be performed by either RETI or RET instruction.
        // The RETI instruction enables interrupts after doing a return operation.
        // If a RET is used as the final instruction in an interrupt routine, interrupts will remain disabled
        // unless a EI was used in the interrupt routine or is used at a later time.
        // The interrupt will be acknowledged during opcode fetch period of each instruction.
    }

    /// When the IF and IE flags of a specific interrupt are both set, the following steps are performed before handling the interrupt:
    /// 1. The IME flag is reset to disable further interrupts.
    /// 2. The corresponding bit in the IF register is reset.
    /// 3. The program counter (PC) is pushed onto the stack.
    fn do_before_handling_interrupt(cpu: &mut Cpu, interrupt_type: InterruptType) {
        cpu.ime = false; // Disable further interrupts

        // Reset the corresponding bit in the IF register
        let mut if_register = cpu.memory_bus.read_byte(IF);
        match interrupt_type {
            InterruptType::VBlank => if_register &= 0b11111110,
            InterruptType::LCD => if_register &= 0b11111101,
            InterruptType::Timer => if_register &= 0b11111011,
            InterruptType::Serial => if_register &= 0b11110111,
            InterruptType::Joypad => if_register &= 0b11101111,
        }
        cpu.memory_bus.write_byte(IF, if_register);

        // Push the current PC onto the stack
        cpu.push_value_to_sp(cpu.registers.pc);
    }

    /// Sets the PC to the interrupt handler address based on the interrupt type and increments clock cycles.
    fn do_handle_interrupt(cpu: &mut Cpu, interrupt_type: InterruptType) {
        cpu.registers.pc = match interrupt_type {
            InterruptType::VBlank => VBLANK_INTERRUT_HANDLER_ADDRESS,
            InterruptType::LCD => LCD_STAT_INTERRUPT_HANDLER_ADDRESS,
            InterruptType::Timer => TIMER_INTERRUPT_HANDLER_ADDRESS,
            InterruptType::Serial => SERIAL_INTERRUPT_HANDLER_ADDRESS,
            InterruptType::Joypad => JOYPAD_INTERRUPT_HANDLER_ADDRESS,
        };

        cpu.increment_20_clock_cycles();
        cpu.is_halt_mode = false;
    }

    /// Returns a struct indicating which interrupts are enabled.
    /// This is used by both IE and IF registers because they have the same bit layout.
    /// * Bit 0 - VBlank Interrupt
    /// * Bit 1 - LCD Interrupt
    /// * Bit 2 - Timer Interrupt
    /// * Bit 3 - Serial Interrupt
    /// * Bit 4 - Joypad Interrupt
    fn get_register_flag_values(if_register: u8) -> InterruptStatus {
        return InterruptStatus {
            vblank: 0b00000001 & if_register != 0,
            lcd: 0b00000010 & if_register != 0,
            timer: 0b00000100 & if_register != 0,
            serial: 0b00001000 & if_register != 0,
            joypad: 0b00010000 & if_register != 0,
        };
    }
    
    /// If any interrupts are pending (IE and IF have matching bits set), exit HALT mode even if IME is disabled.
    fn check_pending_interrupts_to_exit_halt_mode(cpu: &mut Cpu) {
        let if_register = cpu.memory_bus.read_byte(IF);
        let ie_register = cpu.memory_bus.read_byte(IE);

        if if_register != 0 && ie_register != 0 {
            cpu.is_halt_mode = false;
        }
    }
}