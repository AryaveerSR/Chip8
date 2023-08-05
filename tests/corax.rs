use chip8::{structs::BehaviorConfig, Chip};
use std::fs;

#[test]
fn test_corax_output() {
    let mut instr: u32 = 0;
    let mut chip = Chip::new(
        fs::read("roms/tests/corax.ch8").unwrap(),
        BehaviorConfig::default(),
    );

    for _ in 0..INSTRUCTION_COUNT {
        chip.process_instruction(vec![]);
        instr += 1;
    }

    assert_eq!(chip.display(), CORAX_DISPLAY);
}

const INSTRUCTION_COUNT: u32 = 300;

// Weird but good-enough way to test output.
const CORAX_DISPLAY: [[bool; 64]; 32] = [
    [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ],
    [
        false, false, true, true, true, false, true, false, true, false, false, false, false,
        false, false, false, false, false, true, true, true, false, true, false, true, false,
        false, false, false, false, false, false, false, false, true, true, true, false, true,
        false, true, false, false, false, false, false, false, false, false, false, true, true,
        true, false, true, true, true, false, false, false, false, false, false, false,
    ],
    [
        false, false, false, true, true, false, false, true, false, false, false, true, false,
        true, false, false, false, false, false, false, true, false, false, true, false, false,
        false, true, false, true, false, false, false, false, true, true, true, false, true, true,
        true, false, false, true, false, true, false, false, false, false, true, false, false,
        false, true, true, false, false, false, true, false, true, false, false,
    ],
    [
        false, false, false, false, true, false, true, false, true, false, false, true, true,
        false, false, false, false, false, true, true, false, false, true, false, true, false,
        false, true, true, false, false, false, false, false, true, false, true, false, false,
        false, true, false, false, true, true, false, false, false, false, false, true, true,
        false, false, false, false, true, false, false, true, true, false, false, false,
    ],
    [
        false, false, true, true, true, false, true, false, true, false, false, true, false, false,
        false, false, false, false, true, true, true, false, true, false, true, false, false, true,
        false, false, false, false, false, false, true, true, true, false, false, false, true,
        false, false, true, false, false, false, false, false, false, true, false, false, false,
        true, true, false, false, false, true, false, false, false, false,
    ],
    [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ],
    [
        false, false, true, false, true, false, true, false, true, false, false, false, false,
        false, false, false, false, false, true, true, true, false, true, true, true, false, false,
        false, false, false, false, false, false, false, true, true, true, false, true, true, true,
        false, false, false, false, false, false, false, false, false, true, true, true, false,
        true, true, true, false, false, false, false, false, false, false,
    ],
    [
        false, false, true, true, true, false, false, true, false, false, false, true, false, true,
        false, false, false, false, true, false, true, false, true, true, false, false, false,
        true, false, true, false, false, false, false, true, true, true, false, true, true, false,
        false, false, true, false, true, false, false, false, false, true, false, false, false,
        false, true, true, false, false, true, false, true, false, false,
    ],
    [
        false, false, false, false, true, false, true, false, true, false, false, true, true,
        false, false, false, false, false, true, false, true, false, true, false, false, false,
        false, true, true, false, false, false, false, false, true, false, true, false, false,
        false, true, false, false, true, true, false, false, false, false, false, true, true,
        false, false, false, false, true, false, false, true, true, false, false, false,
    ],
    [
        false, false, false, false, true, false, true, false, true, false, false, true, false,
        false, false, false, false, false, true, true, true, false, true, true, true, false, false,
        true, false, false, false, false, false, false, true, true, true, false, true, true, false,
        false, false, true, false, false, false, false, false, false, true, false, false, false,
        true, true, true, false, false, true, false, false, false, false,
    ],
    [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ],
    [
        false, false, true, true, true, false, true, false, true, false, false, false, false,
        false, false, false, false, false, true, true, true, false, true, true, true, false, false,
        false, false, false, false, false, false, false, true, true, true, false, true, true, true,
        false, false, false, false, false, false, false, false, false, true, true, true, false,
        true, true, true, false, false, false, false, false, false, false,
    ],
    [
        false, false, true, true, false, false, false, true, false, false, false, true, false,
        true, false, false, false, false, true, true, true, false, true, false, true, false, false,
        true, false, true, false, false, false, false, true, true, true, false, false, false, true,
        false, false, true, false, true, false, false, false, false, true, false, false, false,
        true, true, false, false, false, true, false, true, false, false,
    ],
    [
        false, false, false, false, true, false, true, false, true, false, false, true, true,
        false, false, false, false, false, true, false, true, false, true, false, true, false,
        false, true, true, false, false, false, false, false, true, false, true, false, false,
        true, false, false, false, true, true, false, false, false, false, false, true, true,
        false, false, true, false, false, false, false, true, true, false, false, false,
    ],
    [
        false, false, true, true, false, false, true, false, true, false, false, true, false,
        false, false, false, false, false, true, true, true, false, true, true, true, false, false,
        true, false, false, false, false, false, false, true, true, true, false, false, true,
        false, false, false, true, false, false, false, false, false, false, true, false, false,
        false, true, true, true, false, false, true, false, false, false, false,
    ],
    [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ],
    [
        false, false, true, true, true, false, true, false, true, false, false, false, false,
        false, false, false, false, false, true, true, true, false, true, true, false, false,
        false, false, false, false, false, false, false, false, true, true, true, false, false,
        true, true, false, false, false, false, false, false, false, false, false, false, false,
        false, false, true, false, true, false, false, false, false, false, false, false,
    ],
    [
        false, false, false, false, true, false, false, true, false, false, false, true, false,
        true, false, false, false, false, true, true, true, false, false, true, false, false,
        false, true, false, true, false, false, false, false, true, true, true, false, true, false,
        false, false, false, true, false, true, false, false, false, false, true, false, true,
        false, false, true, false, false, false, true, false, true, false, false,
    ],
    [
        false, false, false, true, false, false, true, false, true, false, false, true, true,
        false, false, false, false, false, true, false, true, false, false, true, false, false,
        false, true, true, false, false, false, false, false, true, false, true, false, true, true,
        true, false, false, true, true, false, false, false, false, false, true, false, true,
        false, true, false, true, false, false, true, true, false, false, false,
    ],
    [
        false, false, false, true, false, false, true, false, true, false, false, true, false,
        false, false, false, false, false, true, true, true, false, true, true, true, false, false,
        true, false, false, false, false, false, false, true, true, true, false, true, true, true,
        false, false, true, false, false, false, false, false, false, false, true, false, false,
        true, false, true, false, false, true, false, false, false, false,
    ],
    [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ],
    [
        false, false, true, true, true, false, true, false, true, false, false, false, false,
        false, false, false, false, false, true, true, true, false, true, true, true, false, false,
        false, false, false, false, false, false, false, true, true, true, false, true, true, true,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false,
    ],
    [
        false, false, true, true, true, false, false, true, false, false, false, true, false, true,
        false, false, false, false, true, true, true, false, false, false, true, false, false,
        true, false, true, false, false, false, false, true, true, true, false, true, true, false,
        false, false, true, false, true, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false,
    ],
    [
        false, false, false, false, true, false, true, false, true, false, false, true, true,
        false, false, false, false, false, true, false, true, false, true, true, false, false,
        false, true, true, false, false, false, false, false, true, false, true, false, true,
        false, false, false, false, true, true, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ],
    [
        false, false, true, true, false, false, true, false, true, false, false, true, false,
        false, false, false, false, false, true, true, true, false, true, true, true, false, false,
        true, false, false, false, false, false, false, true, true, true, false, true, true, true,
        false, false, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false,
    ],
    [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ],
    [
        false, false, true, true, false, false, true, false, true, false, false, false, false,
        false, false, false, false, false, true, true, true, false, true, true, true, false, false,
        false, false, false, false, false, false, false, true, true, true, false, false, true,
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, true, false, true, false, false, false, true, true, true, false,
    ],
    [
        false, false, false, true, false, false, false, true, false, false, false, true, false,
        true, false, false, false, false, true, true, true, false, false, true, true, false, false,
        true, false, true, false, false, false, false, true, false, false, false, true, false,
        false, false, false, true, false, true, false, false, false, false, true, false, true,
        false, true, true, true, false, false, false, true, false, true, false,
    ],
    [
        false, false, false, true, false, false, true, false, true, false, false, true, true,
        false, false, false, false, false, true, false, true, false, false, false, true, false,
        false, true, true, false, false, false, false, false, true, true, false, false, true, true,
        true, false, false, true, true, false, false, false, false, false, true, false, true,
        false, false, false, true, false, false, false, true, false, true, false,
    ],
    [
        false, false, true, true, true, false, true, false, true, false, false, true, false, false,
        false, false, false, false, true, true, true, false, true, true, true, false, false, true,
        false, false, false, false, false, false, true, false, false, false, true, true, true,
        false, false, true, false, false, false, false, false, false, false, true, false, false,
        false, false, true, false, true, false, true, true, true, false,
    ],
    [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ],
    [
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ],
];
