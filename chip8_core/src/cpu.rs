use std::usize;

use rand::random;

use crate::keypad::Keypad;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const START_ADDR: u16 = 0x200;
const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Emu {
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; NUM_REGS],
    i_reg: u16,
    sp: u16,
    stack: [u16; STACK_SIZE],
    pub keypad: Keypad,
    dt: u8,
    st: u8,
}

impl Emu {
    pub fn new() -> Self {
        let mut new_emu = Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keypad: Keypad::new(),
            dt: 0,
            st: 0,
        };
        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        new_emu
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        // self.keys = [false; NUM_KEYS]; //TODO: check if it's necessary
        self.dt = 0;
        self.st = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn tick(&mut self) {
        // Fetch
        let op = self.fetch();
        // Decode
        self.execute(op);
        // Execute
    }

    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                // BEEP
            }
            self.st -= 1;
        }
    }

    pub fn get_display(&self) -> &[bool] {
        &self.screen
    }


    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDR as usize;
        let end = (START_ADDR as usize) + data.len();
        self.ram[start..end].copy_from_slice(data);
    }

    fn fetch(&mut self) -> u16 {
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;
        let op = (higher_byte << 8) | lower_byte;
        self.pc += 2;
        op
    }

    fn execute(&mut self, op: u16) {
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x000F;

        let x = digit2 as usize;
        let y = digit3 as usize;
        let vx = self.v_reg[x];
        let nnn = op & 0xFFF;
        let kk = (op & 0x00FF) as u8;

        match (digit1, digit2, digit3, digit4) {
            // CLS
            (0, 0, 0xE, 0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }
            // RET
            (0, 0, 0xE, 0xE) => {
                let ret_addr = self.pop();
                self.pc = ret_addr;
            }
            // JMP NNN
            (0x1, _, _, _) => {
                self.pc = nnn;
            }
            // CALL NNN
            (0x2, _, _, _) => {
                self.push(self.pc);
                self.pc = nnn;
            }
            // SKIP VX == KK
            (0x3, _, _, _) => {
                if self.v_reg[x] == kk {
                    self.pc += 2;
                }
            }
            // SKIP VX != KK
            (0x4, _, _, _) => {
                if self.v_reg[x] != kk {
                    self.pc += 2;
                }
            }
            // SKIP VX == VY
            (0x5, _, _, _) => {
                if self.v_reg[x] == self.v_reg[y] {
                    self.pc += 2;
                }
            }
            // VX = KK
            (0x6, _, _, _) => {
                self.v_reg[x] = kk;
            }
            // VX += KK
            (0x7, _, _, _) => {
                self.v_reg[x] = self.v_reg[x].wrapping_add(kk);
            }
            // VX = VY
            (0x8, _, _, 0x0) => {
                self.v_reg[x] = self.v_reg[y];
            }
            // VX |= VY
            (0x8, _, _, 0x1) => {
                self.v_reg[x] |= self.v_reg[y];
            }
            // VX &= VY
            (0x8, _, _, 0x2) => {
                self.v_reg[x] &= self.v_reg[y];
            }
            // VX ^= VY
            (0x8, _, _, 0x3) => {
                self.v_reg[x] ^= self.v_reg[y];
            }
            // VX += VY
            (8, _, _, 0x4) => {
                let (new_vx, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                let new_vf = if carry { 1 } else { 0 };

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            }
            // VX -= VY
            (0x8, _, _, 0x5) => {
                let (new_vx, borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                let new_vf = if borrow { 0 } else { 1 };

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            }
            // VX >>= 1
            (0x8, _, _, 0x6) => {
                let lsb = self.v_reg[x] & 1;

                self.v_reg[x] >>= 1;
                self.v_reg[0xF] = lsb;
            }
            // VX = VY - VX
            (0x8, _, _, 0x7) => {
                let (new_vx, borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                let new_vf = if borrow { 0 } else { 1 };

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            }
            // VX <<= 1
            (0x8, _, _, 0xE) => {
                let msb = (self.v_reg[x] >> 7) & 1;
                self.v_reg[x] <<= 1;
                self.v_reg[0xF] = msb;
            }
            // SKIP VX != VY
            (0x9, _, _, _) => {
                if self.v_reg[x] != self.v_reg[y] {
                    self.pc += 2;
                }
            }
            // I = NNN
            (0xA, _, _, _) => {
                self.i_reg = nnn;
            }
            // JMP V0 + NNN
            (0xB, _, _, _) => {
                self.pc = (self.v_reg[0] as u16) + nnn;
            }
            // VX = rand() & KK
            (0xC, _, _, _) => {
                let rng: u8 = random();
                self.v_reg[x] = rng & kk;
            }
            // DRAW
            (0xD, _, _, _) => {
                // Get the (x, y) coords for our sprite
                let x_coord = self.v_reg[digit2 as usize] as u16;
                let y_coord = self.v_reg[digit3 as usize] as u16;
                // The last digit determines how many rows high our sprite is
                let num_rows = digit4;

                // Keep track if any pixels were flipped
                let mut flipped = false;
                // Iterate over each row of our sprite
                for y_line in 0..num_rows {
                    // Determine which memory address our row's data is stored
                    let addr = self.i_reg + y_line;
                    let pixels = self.ram[addr as usize];
                    // Iterate over each column in our row
                    for x_line in 0..8 {
                        // Use a mask to fetch current pixel's bit. Only flip if a 1
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            // Sprites should wrap around screen, so apply modulo
                            let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coord + y_line) as usize % SCREEN_HEIGHT;

                            // Get our pixel's index for our 1D screen array
                            let idx = x + SCREEN_WIDTH * y;
                            // Check if we're about to flip the pixel and set
                            flipped |= self.screen[idx];
                            self.screen[idx] ^= true;
                        }
                    }
                }

                // Populate VF register
                if flipped {
                    self.v_reg[0xF] = 1;
                } else {
                    self.v_reg[0xF] = 0;
                }
            }
            // SKIP KEY PRESS
            (0xE, _, 0x9, 0xE) => {
                if self.keypad.is_pressed(vx as usize) {
                    self.pc += 2;
                }
            }
            // SKIP KEY RELEASE
            (0xE, _, 0xA, 0x1) => {
                if !self.keypad.is_pressed(vx as usize) {
                    self.pc += 2;
                }
            }
            // VX = DT
            (0xF, _, 0x0, 0x7) => {
                self.v_reg[x] = self.dt;
            }
            // WAIT KEY
            (0xF, _, 0x0, 0xA) => {
                let mut pressed = false;
                for (i, key) in self.keypad.keys.iter().enumerate() {
                    if *key {
                        self.v_reg[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }

                if !pressed {
                    // Redo opcode
                    self.pc -= 2;
                }
            }
            // DT = VX
            (0xF, _, 0x1, 0x5) => {
                self.dt = self.v_reg[x];
            }
            // ST = VX
            (0xF, _, 0x1, 0x8) => {
                self.st = self.v_reg[x];
            }
            // I += VX
            (0xF, _, 0x1, 0xE) => {
                self.i_reg = self.i_reg.wrapping_add(vx as u16);
            }
            // I = FONT
            (0xF, _, 0x2, 0x9) => {
                self.i_reg = vx as u16 * 5;
            }
            // BCD
            (0xF, _, 0x3, 0x3) => {
                self.ram[self.i_reg as usize] = vx / 100;
                self.ram[(self.i_reg + 1) as usize] = (vx / 10) % 10;
                self.ram[(self.i_reg + 2) as usize] = (vx % 100) % 10;
            }
            // STORE V0 - VX
            (0xF, _, 0x5, 0x5) => {
                let i = self.i_reg as usize;
                for idx in 0..=x {
                    self.ram[i + idx] = self.v_reg[idx];
                }
            }
            // LOAD V0 - VX
            (0xF, _, 0x6, 0x5) => {
                let i = self.i_reg as usize;
                for idx in 0..=x {
                    self.v_reg[idx] = self.ram[i + idx];
                }
            }
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", op),
        }
    }
}

impl Default for Emu {
    fn default() -> Self {
        Self::new()
    }
}