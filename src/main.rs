//! Entry point for CHIP-8 Emulator
//! @AryaveerSR <me.aryaveer@gmail.com>

use chip8::{
    helpers,
    structs::{Beeper, BehaviorConfig},
    Chip,
};
use gumdrop::Options;
use minifb::{Key, Scale, Window, WindowOptions};
use std::fs;

#[derive(Options)]
struct ArgOpts {
    #[options(free, help = "Path to the Chip 8 binary")]
    free: Vec<String>,

    #[options(help = "Print Help Message")]
    help: bool,

    #[options(help = "Window update rate limit")]
    rate: Option<u64>,

    #[options(help = "Whether to reset V(F) after 8xy1, 8xy2, and 8xy3 instructions")]
    vf_reset: Option<bool>,

    #[options(help = "Whether to increment I on save and load instructions")]
    increment_i: Option<bool>,
}

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() {
    let args = ArgOpts::parse_args_default_or_exit();
    let mut beep = Beeper::new();

    let update_rate = match args.rate {
        Some(r) => r,
        None => 75, // Default value. (feels slightly too fast)
    };

    let mut buffer: Vec<u32>;
    let mut opts = WindowOptions::default();
    opts.scale = Scale::X16;

    let mut window =
        Window::new("Chip 8 Emulator", WIDTH, HEIGHT, opts).expect("Should create widnow");
    window.limit_update_rate(Some(std::time::Duration::from_micros(update_rate)));

    let mut behavior = BehaviorConfig::default();
    if args.increment_i.is_some() {
        behavior.increment_i_on_save_load = args.increment_i.unwrap();
    }
    if args.vf_reset.is_some() {
        behavior.vf_reset = args.vf_reset.unwrap();
    }

    let mut chip = Chip::new(fs::read(&args.free[0]).unwrap(), behavior);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let (display_update, is_playing_sound) =
            chip.process_instruction(helpers::keys_to_u8(window.get_keys()));
        beep.update(is_playing_sound);
        if display_update {
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
