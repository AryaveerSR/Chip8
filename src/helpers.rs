//! Helper functions
//! @AryaveerSR <me.aryaveer@gmail.com>

use minifb::Key;

/// ## Key mappings
/// The key mappings are described here:
///
/// https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#keypad
pub fn keys_to_u8(keys: Vec<Key>) -> Vec<u8> {
    let mut key_vec: Vec<u8> = vec![];

    for key in keys {
        key_vec.push(match key {
            Key::Key1 => 0x01,
            Key::Key2 => 0x02,
            Key::Key3 => 0x03,
            Key::Key4 => 0x0C,
            Key::Q => 0x04,
            Key::W => 0x05,
            Key::E => 0x06,
            Key::R => 0x0D,
            Key::A => 0x07,
            Key::S => 0x08,
            Key::D => 0x09,
            Key::F => 0x0E,
            Key::Z => 0x0A,
            Key::X => 0x00,
            Key::C => 0x0B,
            Key::V => 0x0F,
            _ => {
                continue;
            }
        });
    }

    key_vec
}
