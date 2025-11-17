use crate::gameboy_core::cpu_utils;
use minifb::{Key, Window, WindowOptions};

pub mod gameboy_core;

const WIDTH: usize = 320;
const HEIGHT: usize = 288;

fn main() {
    // let rom_file = "tests/cpu_instrs.gb";
    let rom_file = "tests/nintendo_logo.gb";
    let rom_binary = cpu_utils::read_rom(format!("files/roms/{}", rom_file).as_str()).unwrap();

    let mut cpu = gameboy_core::cpu::Cpu::start(rom_binary, true);

    let mut window = Window::new(
        "Gameboy Emulator - Press SPACE to tick CPU",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // Run the event loop
    run_cpu_with_keyboard(&mut cpu, &mut window, &mut buffer);
}

/// Runs the CPU with a minifb window. Press SPACE to execute a CPU tick.
fn run_cpu_with_keyboard(
    cpu: &mut gameboy_core::cpu::Cpu,
    window: &mut Window,
    buffer: &mut Vec<u32>,
) {
    let mut space_pressed: bool = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Check if SPACE is pressed
        let space_down = window.is_key_down(Key::Space);
        if space_down && !space_pressed {
            cpu.tick();
        }
        space_pressed = space_down;

        // if window.is_key_down(Key::Space) {
        //     cpu.tick();
        //     println!("CPU tick executed!");
        //     std::thread::sleep(std::time::Duration::from_millis(100));
        // }

        // Update the window with the buffer
        window.update_with_buffer(buffer, WIDTH, HEIGHT).unwrap();
    }
}
