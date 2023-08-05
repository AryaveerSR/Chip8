//! Main Implementation for CHIP-8 Emulator
//! @AryaveerSR <me.aryaveer@gmail.com>
//!
//! References:
//! http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
//! https://tobiasvl.github.io/blog/write-a-chip-8-emulator

pub mod helpers;
pub mod structs;

use crate::structs::{BehaviorConfig, Instruction, VariableRegisters};
use rand::Rng;
use std::{num::Wrapping, time::Instant};

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

#[derive(Debug)]
pub struct Chip {
    /// 4KB Memory (4096 Bytes)
    memory: Vec<u8>,
    /// 2D Array of bools each representing a monochromatic pixel.
    /// 1 => White;
    /// 0 => Black
    // TODO: Use `bitvec` crate ??
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
    /// If the program is halted and waiting for a keypress
    /// The u8 is the register to put the keycode into.
    is_waiting_for_press: Option<u8>,
    /// Time elapsed since last timer update
    last_update: Instant,
    /// Behavior Configurations for conflicting implementations (Check `structs.rs`)
    behavior: BehaviorConfig,
}

impl Chip {
    /// The memory location at which programs should be loaded.
    /// Memory 0x000 to 0x1FF is (almost) empty as it used to contain the
    /// interpreter itself. It's only use is fonts.
    const PROGRAM_START: u16 = 0x200;

    pub fn display(&self) -> [[bool; 64]; 32] {
        self.display
    }

    /// Process current instruction
    /// Returns true if display is updated
    ///
    /// Takes a Vec<u8> of all the (supported) keys currently being pressed
    pub fn process_instruction(&mut self, keys: Vec<u8>) -> bool {
        // Update timers
        if self.last_update.elapsed().as_millis() > 17 {
            if self.delay_timer != 0 {
                self.delay_timer -= 1;
            }
            if self.sound_timer != 0 {
                self.sound_timer -= 1;
            }
            self.last_update = Instant::now();
        }

        // Check if it's supposed to wait for a keypress
        if self.is_waiting_for_press.is_some() {
            if keys.is_empty() {
                return false;
            } else {
                // Put the keycode into the register if there's a key pressed
                self.var_reg
                    .set(self.is_waiting_for_press.unwrap(), keys[0]);
            }
        }

        // Get two consecutive bytes from PC & PC+1 and combine to form one u16(2 bytes)
        // and then construct them into the Instruction struct
        let first_byte = *self
            .memory
            .get(self.pc as usize)
            .expect("Program Counter is invalid");

        let second_byte = *self
            .memory
            .get(self.pc as usize + 1)
            .expect("Program Counter is invalid") as u16;

        let instr = Instruction::from_u16(((first_byte as u16) << 8) | second_byte as u16);

        // Increment to next instruction
        self.pc += 2;

        match instr.get_nib(0) {
            0x0 => match instr.0 {
                // 00E0
                // Clear the display
                0x00E0 => {
                    self.display = [[false; 64]; 32];
                    return true;
                }

                // 00EE
                // Return from subroutine
                // Pops the return address from stack and sets the PC
                0x00EE => self.pc = self.stack.pop().unwrap(),

                // 0000
                // Blank
                0x0000 => {
                    return false;
                }
                _ => panic!("{:?}", instr,),
            },
            // 1nnn
            // Jump to address
            0x1 => self.pc = instr.get_addr(),
            // 2nnn
            // Call subroutine and push current PC to stack
            0x2 => {
                self.stack.push(self.pc);
                self.pc = instr.get_addr();
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
                ((instr.get_lbyte() as u16 + self.var_reg.get(instr.get_nib(1)) as u16) & 0x00FF)
                    as u8,
            ),
            0x8 => {
                let x_addr = instr.get_nib(1);
                let x_val = self.var_reg.get(x_addr);
                let y_val = self.var_reg.get(instr.get_nib(2));

                match instr.get_nib(3) {
                    // 8xy0
                    // Set V(x) = V(y)
                    0x0 => self.var_reg.set(x_addr, y_val),
                    // 8xy1
                    // Set V(x) = V(x) |(Bitwise OR) V(y)
                    0x1 => {
                        self.var_reg.set(x_addr, x_val | y_val);
                        if self.behavior.vf_reset {
                            self.var_reg.vf = 0;
                        }
                    }
                    // 8xy2
                    // Set V(x) = V(x) &(Bitwise AND) V(y)
                    0x2 => {
                        self.var_reg.set(x_addr, x_val & y_val);
                        if self.behavior.vf_reset {
                            self.var_reg.vf = 0;
                        }
                    }
                    // 8xy3
                    // Set V(x) = V(x) ^(Bitwise XOR) V(y)
                    0x3 => {
                        self.var_reg.set(x_addr, x_val ^ y_val);
                        if self.behavior.vf_reset {
                            self.var_reg.vf = 0;
                        }
                    }
                    // 8xy4
                    // Set V(x) = V(x) + V(y) & set V(F) to carry
                    0x4 => {
                        self.var_reg
                            .set(x_addr, ((x_val as u16 + y_val as u16) & 0x00FF) as u8);

                        // Set V(F) to 1 if its greater than 255
                        if ((x_val as u16 + y_val as u16) & 0xFF00) != 0 {
                            self.var_reg.vf = 1;
                        } else {
                            self.var_reg.vf = 0;
                        }
                    }
                    // 8xy5
                    // Set V(x) = V(x) - V(y) & set V(F) as NOT borrow
                    0x5 => {
                        self.var_reg
                            .set(x_addr, (Wrapping(x_val) - Wrapping(y_val)).0);

                        // Set V(F) to 1 if V(x) > V(y)
                        if x_val > y_val {
                            self.var_reg.vf = 1;
                        } else {
                            self.var_reg.vf = 0;
                        }
                    }
                    // 8xy6
                    // Bitshift to right with V(F) = removed bit
                    0x6 => {
                        // Conflicting Implementations
                        // https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#8xy6-and-8xye-shift

                        self.var_reg.set(x_addr, y_val >> 1);
                        self.var_reg.vf = ((x_val & 0x1) != 0) as u8;
                    }
                    // 8xy7
                    // Set V(x) = V(y) - V(x) & set V(F) as NOT borrow
                    0x7 => {
                        self.var_reg
                            .set(x_addr, (Wrapping(y_val) - Wrapping(x_val)).0);

                        // Set V(F) to 1 if V(y) > V(x)
                        if y_val > x_val {
                            self.var_reg.vf = 1;
                        } else {
                            self.var_reg.vf = 0;
                        }
                    }
                    // 8xyE
                    // Bitshift to left with V(F) = removed bit
                    0xE => {
                        // Conflicting Implementations
                        // https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#8xy6-and-8xye-shift

                        self.var_reg.set(x_addr, y_val << 1);
                        self.var_reg.vf = ((y_val & 0x80) != 0) as u8;
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
            0xD => {
                // Starting X-Coordinate
                let initial_x = self.var_reg.get(instr.get_nib(1)) % 64;
                // Current X-Coordinate
                let mut x_coord = initial_x as usize;
                // Current Y-Coordinate
                let mut y_coord = self.var_reg.get(instr.get_nib(2)) as usize % 32;
                // Total height
                let len = instr.get_nib(3);

                let addr = self.i_reg;
                self.var_reg.vf = 0;

                // For every line..
                for i in 0..len {
                    let sprite_data = self.memory[(addr + i as u16) as usize];

                    // For every bit..
                    for j in 0..(8 as u8) {
                        if sprite_data & (128 >> j) != 0 {
                            self.display[y_coord][x_coord] = !self.display[y_coord][x_coord];
                        }

                        x_coord += 1;
                        if x_coord >= 64 {
                            break;
                        }
                    }

                    x_coord = initial_x as usize;
                    y_coord += 1;
                    if y_coord >= 32 {
                        break;
                    }
                }

                return true;
            }
            // Keyboard Interactions
            0xE => {
                let x_val = self.var_reg.get(instr.get_nib(1));
                match instr.get_nib(3) {
                    // Ex9E
                    // Skip instruction if V(x) key is pressed
                    0xE => {
                        if keys.contains(&x_val) {
                            self.pc += 2;
                        }
                    }
                    // ExA1
                    // Skip instruction if V(x) is not pressed
                    0x1 => {
                        if !keys.contains(&x_val) {
                            self.pc += 2;
                        }
                    }
                    _ => panic!("{:?}", instr),
                };
            }
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
                        self.is_waiting_for_press = Some(x_addr);
                        return false;
                    }
                    // Fx18
                    // Set Sound Timer = V(x)
                    0x8 => self.sound_timer = x_val,
                    // Fx1E
                    // Set I = I + V(x)
                    0xE => self.i_reg += x_val as u16,
                    // Fx29
                    // Set I to location of sprite for V(x)
                    0x9 => self.i_reg = (5 * x_val) as u16,

                    // Fx33
                    // Set I, I+1, I+2 to V(x)'s hundreds, tens, and ones digits
                    0x3 => {
                        self.memory[self.i_reg as usize] = (x_val / 100) % 10;
                        self.memory[(self.i_reg + 1) as usize] = (x_val / 10) % 10;
                        self.memory[(self.i_reg + 2) as usize] = x_val % 10;
                    }
                    0x5 => {
                        match instr.get_nib(2) {
                            // Fx15
                            // Set Delay Timer = V(x)
                            0x1 => self.delay_timer = x_val,
                            // Fx55
                            // Store register values in memory
                            0x5 => {
                                for i in 0..(x_addr + 1) {
                                    self.memory[(self.i_reg + i as u16) as usize] =
                                        self.var_reg.get(i);
                                }
                                if self.behavior.increment_i_on_save_load {
                                    self.i_reg += x_addr as u16 + 1;
                                }
                            }
                            // Fx65
                            // Read register values from memory
                            0x6 => {
                                for i in 0..(x_addr + 1) {
                                    self.var_reg
                                        .set(i, self.memory[(self.i_reg + i as u16) as usize]);
                                }
                            }
                            _ => panic!(),
                        }
                    }

                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
        return false;
    }

    pub fn new(program: Vec<u8>) -> Self {
        let mut memory: Vec<u8> = vec![];

        // Loads stuff into memory (painful to the eyes)
        memory.extend_from_slice(&FONT_0);
        memory.extend_from_slice(&FONT_1);
        memory.extend_from_slice(&FONT_2);
        memory.extend_from_slice(&FONT_3);
        memory.extend_from_slice(&FONT_4);
        memory.extend_from_slice(&FONT_5);
        memory.extend_from_slice(&FONT_6);
        memory.extend_from_slice(&FONT_7);
        memory.extend_from_slice(&FONT_8);
        memory.extend_from_slice(&FONT_9);
        memory.extend_from_slice(&FONT_A);
        memory.extend_from_slice(&FONT_B);
        memory.extend_from_slice(&FONT_C);
        memory.extend_from_slice(&FONT_D);
        memory.extend_from_slice(&FONT_E);
        memory.extend_from_slice(&FONT_F);

        memory.resize((Self::PROGRAM_START) as usize, 0);
        memory.extend(program);
        memory.resize(4096, 0);

        Chip {
            memory,
            display: [[false; 64]; 32],
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
            pc: Self::PROGRAM_START,
            i_reg: 0,
            var_reg: VariableRegisters::new(),
            is_waiting_for_press: None,
            last_update: Instant::now(),
            behavior: BehaviorConfig::default(), // TODO: Take user input
        }
    }
}
