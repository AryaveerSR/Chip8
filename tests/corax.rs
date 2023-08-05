use chip8::{helpers, structs::BehaviorConfig, Chip};

#[test]
fn test_corax_output() {
    let mut instr: u32 = 0;
    let mut chip = Chip::new(
        helpers::file_as_vec("roms/tests/corax.ch8"),
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
