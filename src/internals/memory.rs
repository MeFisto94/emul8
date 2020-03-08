use std::fmt;
use std::io;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Registers {
    // program counter (stores the current instruction) allowed values from 0 to 4096
    pub pc: u16,

    // stack pointer, stack is pretty limited on CHIP-8 [16 16bit values], thus only 16 levels of
    // recursion and they only store ret addresses (hence 16 bit). Initialized to -1, because this
    // always points to the top / last pushed element, which is -1 when nothing has been pushed yet
    pub sp: i8,

    // This register is generally used to store memory addresses, so only the lowest
    // (rightmost) 12 bits are usually used.
    pub i: u16,

    // The delay timer register
    pub dt: u8,

    // The sound timer
    pub st: u8,

    // general purpose registers, but VF (V[15]) is used as a special flag by some instructions
    pub v: [u8; 16]
}

impl Default for Registers {
    fn default() -> Self {
        Registers { pc: 0, sp: -1, i: 0, v: [0; 16], dt: 0, st: 0}
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Special Registers: pc={:#X}, sp={:#X}, i={:#X}, dt={:#X}, st={:#X}", self.pc, self.sp, self.i, self.dt, self.st)?;
        writeln!(f, "General Purpose Registers")?;
        writeln!(f, "V0 : {:#X},  V1: {:#X},  V2 : {:#X},  V3: {:#X}",     self.v[0],  self.v[1],  self.v[2],  self.v[3])?;
        writeln!(f, "V4 : {:#X},  V5: {:#X},  V6 : {:#X},  V7: {:#X}",     self.v[4],  self.v[5],  self.v[6],  self.v[7])?;
        writeln!(f, "V8 : {:#X},  V9: {:#X},  V10: {:#X}, V11: {:#X}",    self.v[8],  self.v[9],  self.v[10], self.v[11])?;
        writeln!(f, "V12: {:#X}, V13: {:#X},  V14: {:#X}, V15: {:#X}",    self.v[12], self.v[13], self.v[14], self.v[15])
    }
}

pub struct Memory {
    pub registers: Registers,
    pub stack: [u16; 16],
    pub ram: [u8; 4096]
}

impl Default for Memory {
    fn default() -> Self {
        let mut mem = Memory { stack: [0; 16], registers: Registers::default(), ram: [0; 4096]};

        let digit_0 = [0xF0, 0x90, 0x90, 0x90, 0xF0];
        let digit_1 = [0x20, 0x60, 0x20, 0x20, 0x70];
        let digit_2 = [0xF0, 0x10, 0xF0, 0x80, 0xF0];
        let digit_3 = [0xF0, 0x10, 0xF0, 0x10, 0xF0];
        let digit_4 = [0x90, 0x90, 0xF0, 0x10, 0x10];
        let digit_5 = [0xF0, 0x80, 0xF0, 0x10, 0xF0];
        let digit_6 = [0xF0, 0x80, 0xF0, 0x90, 0xF0];
        let digit_7 = [0xF0, 0x10, 0x20, 0x40, 0x40];
        let digit_8 = [0xF0, 0x90, 0xF0, 0x10, 0xF0];
        let digit_9 = [0xF0, 0x90, 0xF0, 0x10, 0xF0];
        let digit_a = [0xF0, 0x90, 0xF0, 0x90, 0x90];
        let digit_b = [0xE0, 0x90, 0xE0, 0x90, 0xF0];
        let digit_c = [0xF0, 0x80, 0x80, 0x80, 0xF0];
        let digit_d = [0xE0, 0x90, 0x90, 0x90, 0xE0];
        let digit_e = [0xF0, 0x80, 0xF0, 0x80, 0xF0];
        let digit_f = [0xF0, 0x80, 0xF0, 0x80, 0x80];

        for i in 0..5 {
            mem.ram[        i] = digit_0[i];
            mem.ram[    6 + i] = digit_1[i];
            mem.ram[2 * 6 + i] = digit_2[i];
            mem.ram[3 * 6 + i] = digit_3[i];
            mem.ram[4 * 6 + i] = digit_4[i];
            mem.ram[5 * 6 + i] = digit_5[i];
            mem.ram[6 * 6 + i] = digit_6[i];
            mem.ram[7 * 6 + i] = digit_7[i];
            mem.ram[8 * 6 + i] = digit_8[i];
            mem.ram[9 * 6 + i] = digit_9[i];
            mem.ram[0xA * 6 + i] = digit_a[i];
            mem.ram[0xB * 6 + i] = digit_b[i];
            mem.ram[0xC * 6 + i] = digit_c[i];
            mem.ram[0xD * 6 + i] = digit_d[i];
            mem.ram[0xE * 6 + i] = digit_e[i];
            mem.ram[0xF * 6 + i] = digit_f[i];
        }

        mem
    }
}

impl Memory {
    pub fn load_from_file(&mut self, name: &str, loading_point: u16) -> Result<(), io::Error> {
        if loading_point >= 4096 {
            panic!("Loading Point exceeds memory range [0, 4096]");
        }

        let mut buffer = Vec::new();
        File::open(name)?.read_to_end(&mut buffer)?;

        if buffer.len()  > 4096 - loading_point as usize {
            panic!("Image too large to fit into memory");
        }

        // Move buffer into ram at a given loading_point. There got to be a more rusty way for this,
        // but array.from_slice() does not support the offset of the loading_point

        for (i, buf) in buffer.iter().enumerate() {
            self.ram[loading_point as usize + i] = *buf;
        }

        Ok(())
    }

    pub fn read_u8(&mut self, addr: u16) -> u8 {
        if addr > 4096 {
            panic!("Segmentation Fault: Tried to read from address {}", addr);
        }

        self.ram[addr as usize]
    }

    pub fn read_two_u8(&mut self, addr: u16) -> (u8, u8) {
        (self.read_u8(addr), self.read_u8(addr + 1))
    }

    pub fn read_u16(&mut self, addr: u16) -> u16 {
        let (x, y) = self.read_two_u8(addr);
        //x as u16 >> 16 | y as u16
        x as u16 >> 8 | y as u16
    }

    pub fn stack_push(&mut self, val: u16) {
        self.registers.sp += 1;
        self.stack[self.registers.sp as usize] = val;
    }

    pub fn stack_pop(&mut self) -> u16 {
        let peek = self.stack_peek();
        self.registers.sp -= 1;
        peek
    }

    pub fn stack_peek(&mut self) -> u16 {
        self.stack[self.registers.sp as usize]
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Memory {{ registers: {:?}, stack: {:?} }}", self.registers, self.stack)
    }
}