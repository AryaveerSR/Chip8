use chip8::Chip;
use minifb::{Key, Scale, Window, WindowOptions};
use std::{env, fs};

// 10:1 scaling
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() {
    let args: Vec<_> = env::args().collect();

    let program = file_as_vec(&args[1]);
    let mut chip = Chip::new(program);

    let mut buffer: Vec<u32> = vec![0x0000; WIDTH * HEIGHT];
    let mut opts = WindowOptions::default();
    opts.scale = Scale::X16;

    let mut window =
        Window::new("Chip 8 Emulator", WIDTH, HEIGHT, opts).expect("Should create widnow");
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if chip.process_instruction() {
            buffer = chip
                .display()
                .iter()
                .flatten()
                .map(|x| if *x { 0x0FFF } else { 0x0000 })
                .collect();
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn file_as_vec(path: &str) -> Vec<u8> {
    // TODO: More user friendly errors
    fs::read(path).unwrap()
}
