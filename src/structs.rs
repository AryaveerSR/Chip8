/// Structure for general-purpose registers.
/// Simplies accessing them from instructions.
#[derive(Debug)]
pub struct VariableRegisters {
    pub v0: u8,
    pub v1: u8,
    pub v2: u8,
    pub v3: u8,
    pub v4: u8,
    pub v5: u8,
    pub v6: u8,
    pub v7: u8,
    pub v8: u8,
    pub v9: u8,
    pub va: u8,
    pub vb: u8,
    pub vc: u8,
    pub vd: u8,
    pub ve: u8,
    /// It is also used as flag register for instructions.
    /// Many instructions set it to 1 or 0 based on some rule (eg: carry flag)
    pub vf: u8,
}

impl VariableRegisters {
    pub fn get(&self, reg: u8) -> u8 {
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

    pub fn set(&mut self, reg: u8, val: u8) {
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

    pub fn new() -> Self {
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

// Contains helpful methods for parsing instructions
#[derive(Debug)]
pub struct Instruction(pub u16);

impl Instruction {
    // Group of 4 bits. Index from most to least significant
    pub fn get_nib(&self, index: u8) -> u8 {
        ((self.0 & (0x000f << 4 * (3 - index as i8))) >> (4 * (3 - index as i8))) as u8
    }

    // Lowest 8 bits (lower byte)
    pub fn get_lbyte(&self) -> u8 {
        (self.0 & 0x00ff) as u8
    }

    // Lowest 12 bits
    pub fn get_addr(&self) -> u16 {
        (self.0 & 0x0fff) as u16
    }

    pub fn from_u16(n: u16) -> Self {
        Instruction(n)
    }
}

/// Behavior Configurations for conflicting implementations
#[derive(Debug)]
pub struct BehaviorConfig {
    /// Does it reset V(F) register to 0 for 8xy1, 8xy2 and 8xy3 instructions
    pub vf_reset: bool,
    /// Does it increment I register on save and load operations
    pub increment_i_on_save_load: bool,
}

impl BehaviorConfig {
    pub fn default() -> Self {
        BehaviorConfig {
            vf_reset: true,
            increment_i_on_save_load: true,
        }
    }
}
