//! Main Implementation for CHIP-8 Emulator
//! @AryaveerSR <me.aryaveer@gmail.com>
//!
//! References:
//! http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
//! https://tobiasvl.github.io/blog/write-a-chip-8-emulator

use std::ops::Index;

use rand::Rng;

const FONT_0: [u8; 5] = [0xF0, 0x90, 0x90, 0x90, 0xF0];
const FONT_1: [u8; 5] = [0x20, 0x60, 0x20, 0x20, 0x70];
const FONT_2: [u8; 5] = [0xF0, 0x10, 0xF0, 0x80, 0xF0];
const FONT_3: [u8; 5] = [0xF0, 0x10, 0xF0, 0x10, 0xF0];
const FONT_4: [u8; 5] = [0x90, 0x90, 0xF0, 0x10, 0x10];
const FONT_5: [u8; 5] = [0xF0, 0x80, 0xF0, 0x10, 0xF0];
const FONT_6: [u8; 5] = [0xF0, 0x80, 0xF0, 0x90, 0xF0];
const FONT_7: [u8; 5] = [0xF0, 0x10, 0x20, 0x40, 0x40];
const FONT_8: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0xF0];
const FONT_9: [u8; 5] = [0xF0, 0x90, 0xF0, 0x10, 0xF0];
const FONT_A: [u8; 5] = [0xF0, 0x90, 0xF0, 0x90, 0x90];
const FONT_B: [u8; 5] = [0xE0, 0x90, 0xE0, 0x90, 0xE0];
const FONT_C: [u8; 5] = [0xF0, 0x80, 0x80, 0x80, 0xF0];
const FONT_D: [u8; 5] = [0xE0, 0x90, 0x90, 0x90, 0xE0];
const FONT_E: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0xF0];
const FONT_F: [u8; 5] = [0xF0, 0x80, 0xF0, 0x80, 0x80];

const INSTRUCTIONS_PER_SECOND: u16 = 700;

struct Chip {
    /// 4KB Memory (4096 Bytes)
    memory: [u8; 4096],
    /// 2D Array of bools.
    /// True => On(Black);
    /// False => Off(Black)
    display: [[bool; 64]; 32],
    /// Last in, First out stack.
    stack: Vec<u16>,
    /// Decremented by 60 every second (60Hz)
    delay_timer: u8,
    /// Decremented by 60 every second (60Hz)
    /// Play a buzzer if not zero
    sound_timer: u8,
    /// Program Counter.
    /// Stores the location of current instruction in memory
    pc: u16,
    /// Index Register
    /// Used to point at locations in memory
    i_reg: u16,
    /// General-purpose variable registers used by programs
    var_reg: VariableRegisters,
}

impl Chip {
    /// The memory location at which programs should be loaded.
    /// Memory 0x000 to 0x1FF is (almost) empty as it used to contain the
    /// interpreter itself. It's only use is fonts.
    const PROGRAM_START: u16 = 0x200;

    fn process_instruction(&mut self) {
        // Get two consecutive bytes from PC & PC+1 and combine to form one u16(2 bytes)
        // and then construct them into the Instruction struct
        let instr = Instruction::from_u16(
            ((*self
                .memory
                .get(self.pc as usize)
                .expect("Program Counter is invalid") as u16)
                << 8)
                + (*self
                    .memory
                    .get(self.pc as usize + 1)
                    .expect("Program Counter is invalid") as u16)
                << 8,
        );

        // Increment to next instruction
        self.pc += 2;

        //TODO
        match instr.get_nib(0) {
            0x0 => match instr.get_nib(3) {
                // 00E0
                // Clear the display
                0x0 => {
                    // TODO: Clear display
                }
                // 00EE
                // Return from subroutine
                // Pops the return address from stack and sets the PC
                0xE => self.pc = self.stack.pop().unwrap(),
                _ => panic!(),
            },
            // 1nnn
            // Jump to address
            0x1 => self.pc = instr.get_addr(),
            // 2nnn
            // Call subroutine and push current PC to stack
            0x2 => {
                self.stack.push(self.pc);
                self.pc = instr.get_addr()
            }
            // 3xnn
            // Skip instruction if V(x) == nn
            0x3 => {
                if self.var_reg.get(instr.get_nib(1)) == instr.get_lbyte() {
                    self.pc += 2;
                }
            }
            // 4xnn
            // Skip instruction if V(x) != nn
            0x4 => {
                if self.var_reg.get(instr.get_nib(1)) != instr.get_lbyte() {
                    self.pc += 2;
                }
            }
            // 5xy0
            // Skip instruction if V(x) == V(y)
            0x5 => {
                if self.var_reg.get(instr.get_nib(1)) == self.var_reg.get(instr.get_nib(2)) {
                    self.pc += 2;
                }
            }
            // 6xnn
            // Set V(x) = nn
            0x6 => self.var_reg.set(instr.get_nib(1), instr.get_lbyte()),
            // 7xnn
            // Set V(x) += nn
            0x7 => self.var_reg.set(
                instr.get_nib(1),
                instr.get_lbyte() + self.var_reg.get(instr.get_nib(1)),
            ),
            0x8 => {
                let x_addr = instr.get_nib(1);
                let x_val = instr.get_nib(1);
                let y_val = instr.get_nib(2);

                match instr.get_nib(3) {
                    // 8xy0
                    // Set V(x) = V(y)
                    0x0 => self.var_reg.set(x_addr, y_val),
                    // 8xy1
                    // Set V(x) = V(x) |(Bitwise OR) V(y)
                    0x1 => self.var_reg.set(x_addr, x_val | y_val),
                    // 8xy2
                    // Set V(x) = V(x) &(Bitwise AND) V(y)
                    0x2 => self.var_reg.set(x_addr, x_val & y_val),
                    // 8xy3
                    // Set V(x) = V(x) ^(Bitwise XOR) V(y)
                    0x3 => self.var_reg.set(x_addr, x_val ^ y_val),
                    // 8xy4
                    // Set V(x) = V(x) + V(y) & set V(F) to carry
                    0x4 => {
                        self.var_reg
                            .set(x_addr, ((x_val as u16 + y_val as u16) & 0x0F) as u8);

                        // Set V(F) to 1 if its greater than 255
                        if ((x_val as u16 + y_val as u16) & 0xF0) != 0 {
                            self.var_reg.vf = 1;
                        } else {
                            self.var_reg.vf = 0;
                        }
                    }
                    // 8xy5
                    // Set V(x) = V(x) - V(y) & set V(F) as NOT borrow
                    0x5 => {
                        self.var_reg
                            .set(x_addr, (x_val as i8 - y_val as i8).abs() as u8);

                        // Set V(F) to 1 if V(x) > V(y)
                        self.var_reg.vf = (x_val > y_val) as u8;
                    }
                    // 8xy6
                    // Bitshift to right with V(F) = removed bit
                    0x6 => {
                        // Conflicting Implementations
                        // https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#8xy6-and-8xye-shift
                        //
                        // self.var_reg.vf = y_val & 0x1;
                        // self.var_reg.set(x_addr, y_val >> 1)
                        self.var_reg.vf = x_val & 0x1;
                        self.var_reg.set(x_addr, x_val >> 1);
                    }
                    // 8xy7
                    // Set V(x) = V(y) - V(x) & set V(F) as NOT borrow
                    0x7 => {
                        self.var_reg
                            .set(x_addr, (x_val as i8 - y_val as i8).abs() as u8);

                        // Set V(F) to 1 if V(y) > V(x)
                        self.var_reg.vf = (y_val > x_val) as u8;
                    }
                    // 8xyE
                    // Bitshift to left with V(F) = removed bit
                    0xE => {
                        // Conflicting Implementations
                        // https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#8xy6-and-8xye-shift
                        //
                        // self.var_reg.vf = y_val & 0x80;
                        // self.var_reg.set(x_addr, y_val << 1);
                        self.var_reg.vf = x_val & 0x80;
                        self.var_reg.set(x_addr, x_val << 1);
                    }
                    _ => panic!(),
                };
            }
            // 9xy0
            // Skip next instruction if V(x) != V(y)
            0x9 => {
                if self.var_reg.get(instr.get_nib(1)) != self.var_reg.get(instr.get_nib(2)) {
                    self.pc = self.pc + 2;
                }
            }
            // Annn
            // Set I to nnn
            0xA => self.i_reg = instr.get_addr(),
            // Bnnn
            // Set PC to nnn + V(0)
            0xB => self.pc = instr.get_addr() + (self.var_reg.v0 as u16),
            // Cxnn
            // Generate a random number from 0 to 255 and AND it with nn
            0xC => {
                let num = rand::thread_rng().gen_range(0..256) as u8;
                self.var_reg.set(instr.get_nib(1), instr.get_lbyte() & num);
            }
            // Dxyn
            // Draw sprite function
            //
            // https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#dxyn-display
            // TODO: Doc
            0xD => {
                let mut x_coord = self.var_reg.get(instr.get_nib(1)) & 0x3F;
                let mut y_coord = self.var_reg.get(instr.get_nib(2)) & 0x1F;
                let len = instr.get_nib(3);

                let addr = self.i_reg;
                self.var_reg.vf = 0;

                for i in 0..len {
                    let sprite_data = self.memory.index((addr + i as u16) as usize);

                    for j in 0..(8 as u8) {
                        if sprite_data & (1 << j) == 0 {
                            self.display[y_coord as usize][x_coord as usize] =
                                !self.display[y_coord as usize][x_coord as usize];
                        }

                        x_coord += 1;
                        if x_coord == 0x40 {
                            break;
                        }
                    }

                    y_coord += 1;
                    if y_coord == 0x20 {
                        break;
                    }
                }
            }
            // Keyboard Interactions
            0xE => match instr.get_nib(3) {
                // Ex9E
                // Skip instruction if V(x) key is pressed
                0xE => {
                    //TODO: Check keyboard if V(x) is pressed
                    if false {
                        self.pc += 2;
                    }
                }
                // ExA1
                // Skip instruction if V(x) is not pressed
                0x1 => {
                    //TODO: Check keyboard if V(x) is not pressed
                    if true {
                        self.pc += 2;
                    }
                }
                _ => panic!(),
            },
            // Delays and sound timers
            0xF => {
                let x_addr = instr.get_nib(1);
                let x_val = self.var_reg.get(x_addr);

                match instr.get_nib(3) {
                    // Fx07
                    // Set V(x) = Delay Timer
                    0x7 => self.var_reg.set(x_addr, self.delay_timer),
                    // Fx0A
                    // Wait for key press and store value in V(x)
                    0xA => {
                        //TODO: Wait for key press
                        while !true {
                            //TODO: Set V(x) to key
                            self.var_reg.set(x_addr, 0x00);
                        }
                    }
                    // Fx15
                    // Set Delay Timer = V(x)
                    0x5 => self.delay_timer = x_val,

                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    fn new() -> Self {
        Chip {
            memory: [0; 4096], //TODO: Load program
            display: [[false; 64]; 32],
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
            pc: Self::PROGRAM_START,
            i_reg: 0,
            var_reg: VariableRegisters::new(),
        }
    }
}

// Contains helpful methods for parsing instructions
struct Instruction(u16);

impl Instruction {
    // Group of 4 bits. Index from most to least significant
    fn get_nib(&self, index: u8) -> u8 {
        ((self.0 & (0x000f << 4 * (4 - index as i8))) >> (4 * (4 - index as i8))) as u8
    }

    // Lowest 8 bits (lower byte)
    fn get_lbyte(&self) -> u8 {
        (self.0 & 0x00ff) as u8
    }

    // Lowest 12 bits
    fn get_addr(&self) -> u16 {
        (self.0 & 0x0fff) as u16
    }

    fn from_u16(n: u16) -> Self {
        Instruction(n)
    }
}

/// Structure for general-purpose registers.
/// Simplies accessing them from instructions.
struct VariableRegisters {
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    va: u8,
    vb: u8,
    vc: u8,
    vd: u8,
    ve: u8,
    /// It is also used as flag register for instructions.
    /// Many instructions set it to 1 or 0 based on some rule (eg: carry flag)
    vf: u8,
}

impl VariableRegisters {
    fn get(&self, reg: u8) -> u8 {
        match reg {
            0 => self.v0,
            1 => self.v1,
            2 => self.v2,
            3 => self.v3,
            4 => self.v4,
            5 => self.v5,
            6 => self.v6,
            7 => self.v7,
            8 => self.v8,
            9 => self.v9,
            10 => self.va,
            11 => self.vb,
            12 => self.vc,
            13 => self.vd,
            14 => self.ve,
            15 => self.vf,
            _ => panic!(),
        }
    }

    fn set(&mut self, reg: u8, val: u8) {
        match reg {
            0 => self.v0 = val,
            1 => self.v1 = val,
            2 => self.v2 = val,
            3 => self.v3 = val,
            4 => self.v4 = val,
            5 => self.v5 = val,
            6 => self.v6 = val,
            7 => self.v7 = val,
            8 => self.v8 = val,
            9 => self.v9 = val,
            10 => self.va = val,
            11 => self.vb = val,
            12 => self.vc = val,
            13 => self.vd = val,
            14 => self.ve = val,
            15 => self.vf = val,
            _ => panic!(),
        };
    }

    fn new() -> Self {
        VariableRegisters {
            v0: 0,
            v1: 0,
            v2: 0,
            v3: 0,
            v4: 0,
            v5: 0,
            v6: 0,
            v7: 0,
            v8: 0,
            v9: 0,
            va: 0,
            vb: 0,
            vc: 0,
            vd: 0,
            ve: 0,
            vf: 0,
        }
    }
}
