//! Entry point for CHIP-8 Emulator
//! @AryaveerSR <me.aryaveer@gmail.com>

use chip8::{helpers, Chip};
use minifb::{Key, Scale, Window, WindowOptions};
use std::env;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut buffer: Vec<u32>;
    let mut opts = WindowOptions::default();
    opts.scale = Scale::X16;

    let mut window =
        Window::new("Chip 8 Emulator", WIDTH, HEIGHT, opts).expect("Should create widnow");
    window.limit_update_rate(Some(std::time::Duration::from_micros(75))); // This value is from trial and error :D

    let mut chip = Chip::new(helpers::file_as_vec(&args[1]));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if chip.process_instruction(helpers::keys_to_u8(window.get_keys())) {
            buffer = chip
                .display()
                .iter()
                .flatten()
                .map(|x| if *x { 0x00FFFFFF } else { 0x0000 })
                .collect();

            window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        } else {
            window.update();
        }
    }
}
