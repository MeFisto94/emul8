use crate::internals::processor::Processor;
use downcast_rs::Downcast;
use rand::Rng;
use std::fmt;

pub trait Opcode: fmt::Debug + fmt::Display + Downcast {
    fn execute(&self, processor: &mut Processor);
    fn modified_pc(&self) -> bool {
        false // The majority does not tamper with the PC
    }
    fn assemble(&self) -> (u8, u8) {
        (0x0, 0x0) // To easier migrate, return an invalid OPCode
    }
}
impl_downcast!(Opcode);

#[derive(Debug)]
pub struct InvalidOpcode {
    pub opcode: (u8, u8),
}
#[derive(Debug)]
pub struct CLS {}
#[derive(Debug)]
pub struct RET {}

#[derive(Debug)]
pub struct JMP {
    pub address: u16,
}
#[derive(Debug)]
pub struct CALL {
    pub address: u16,
}
#[derive(Debug)]
pub struct SEVxByte {
    pub reg: u8,
    pub byte: u8,
}
#[derive(Debug)]
pub struct SNEVxByte {
    pub reg: u8,
    pub byte: u8,
}
#[derive(Debug)]
pub struct SEVxVy {
    pub reg_a: u8,
    pub reg_b: u8,
}
#[derive(Debug)]
pub struct SNEVxVy {
    pub reg_a: u8,
    pub reg_b: u8,
}
#[derive(Debug)]
pub struct LDVxByte {
    pub reg: u8,
    pub byte: u8,
}
#[derive(Debug)]
pub struct ADDVxByte {
    pub reg: u8,
    pub byte: u8,
}

#[derive(Debug)]
pub struct LDVxVy {
    pub reg_a: u8,
    pub reg_b: u8,
}
#[derive(Debug)]
pub struct ORVxVy {
    pub reg_a: u8,
    pub reg_b: u8,
}
#[derive(Debug)]
pub struct ANDVxVy {
    pub reg_a: u8,
    pub reg_b: u8,
}
#[derive(Debug)]
pub struct XORVxVy {
    pub reg_a: u8,
    pub reg_b: u8,
}
#[derive(Debug)]
pub struct ADDVxVy {
    pub reg_a: u8,
    pub reg_b: u8,
}
#[derive(Debug)]
pub struct SUBVxVy {
    pub reg_a: u8,
    pub reg_b: u8,
}
#[derive(Debug)]
pub struct SHRVxVy {
    pub reg_a: u8,
    pub reg_b: u8,
}
#[derive(Debug)]
pub struct SUBNVxVy {
    pub reg_a: u8,
    pub reg_b: u8,
}
#[derive(Debug)]
pub struct SHLVxVy {
    pub reg_a: u8,
    pub reg_b: u8,
}

#[derive(Debug)]
pub struct LDVxDT {
    pub reg: u8,
}
#[derive(Debug)]
pub struct LDVxK {
    pub reg: u8,
}
#[derive(Debug)]
pub struct LDDTVx {
    pub reg: u8,
}
#[derive(Debug)]
pub struct LDSTVx {
    pub reg: u8,
}
#[derive(Debug)]
pub struct ADDIVx {
    pub reg: u8,
}
#[derive(Debug)]
pub struct LDFVx {
    pub reg: u8,
}
#[derive(Debug)]
pub struct LDBVx {
    pub reg: u8,
}
#[derive(Debug)]
pub struct LDIVx {
    pub reg: u8,
}
#[derive(Debug)]
pub struct LDVxI {
    pub reg: u8,
}

#[derive(Debug)]
pub struct SKPKBRDVx {
    pub reg: u8,
}
#[derive(Debug)]
pub struct SKNPBRDVx {
    pub reg: u8,
}

#[derive(Debug)]
pub struct DRW {
    pub reg_x: u8,
    pub reg_y: u8,
    pub size: u8,
}

#[derive(Debug)]
pub struct RNDVxByte {
    pub reg: u8,
    pub byte: u8,
}

#[derive(Debug)]
pub struct LDIAddr {
    pub address: u16,
}

#[derive(Debug)]
pub struct JPV0Offset {
    pub address: u16,
}

impl Opcode for CLS {
    fn execute(&self, processor: &mut Processor) {
        for i in 0..64 * 32 {
            processor.display.screen[i] = false;
        }
    }

    fn assemble(&self) -> (u8, u8) {
        (0x0, 0xE0)
    }
}

impl fmt::Display for CLS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CLS")
    }
}

impl Opcode for RET {
    fn execute(&self, processor: &mut Processor) {
        let pop = processor.memory.stack_pop();
        println!(
            "Returning from {:#X} to {:#X}",
            processor.memory.registers.pc, pop
        );
        processor.memory.registers.pc = pop;
    }

    fn assemble(&self) -> (u8, u8) {
        (0x0, 0xEE)
    }

    fn modified_pc(&self) -> bool {
        true
    }
}

impl fmt::Display for RET {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RET")
    }
}

impl Opcode for InvalidOpcode {
    fn execute(&self, _processor: &mut Processor) {
        panic!("Unknown Opcode: {:#X} {:#X}", self.opcode.0, self.opcode.1)
    }
}

impl fmt::Display for InvalidOpcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "INVALID")
    }
}

impl JMP {
    pub fn new(val: u8, low: u8) -> JMP {
        JMP {
            address: (((val as u16) << 8) | low as u16),
        }
    }
}

impl Opcode for JMP {
    fn execute(&self, processor: &mut Processor) {
        if self.address + 2 == processor.memory.registers.pc {
            panic!("Useless infinite loop detected, jmp jumps to itself");
        }
        //println!("Jumping to {:#X}", self.address);
        processor.memory.registers.pc = self.address;
    }

    fn modified_pc(&self) -> bool {
        true
    }

    fn assemble(&self) -> (u8, u8) {
        (
            (1 << 4 | (self.address & 0xF00) >> 8) as u8,
            (self.address & 0xFF) as u8,
        )
    }
}

impl fmt::Display for JMP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JMP {:#X}", self.address)
    }
}

impl CALL {
    pub fn new(val: u8, low: u8) -> CALL {
        CALL {
            address: (((val as u16) << 8) | low as u16),
        }
    }
}

impl fmt::Display for CALL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CALL {:#X}", self.address)
    }
}

impl Opcode for CALL {
    fn execute(&self, processor: &mut Processor) {
        //println!("Calling {:#X}", self.address);
        processor
            .memory
            .stack_push(processor.memory.registers.pc + 2);
        processor.memory.registers.pc = self.address;
    }

    fn assemble(&self) -> (u8, u8) {
        (
            (2 << 4 | (self.address & 0xF00) >> 8) as u8,
            (self.address & 0xFF) as u8,
        )
    }

    fn modified_pc(&self) -> bool {
        true
    }
}

impl LDIAddr {
    pub fn new(val: u8, low: u8) -> LDIAddr {
        LDIAddr {
            address: (((val as u16) << 8) | low as u16),
        }
    }
}

impl Opcode for LDIAddr {
    fn execute(&self, processor: &mut Processor) {
        processor.memory.registers.i = self.address;
    }

    fn assemble(&self) -> (u8, u8) {
        (
            (0xA << 4 | (self.address & 0xF00) >> 8) as u8,
            (self.address & 0xFF) as u8,
        )
    }
}

impl fmt::Display for LDIAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LDI {:#X}", self.address)
    }
}

impl JPV0Offset {
    pub fn new(val: u8, low: u8) -> JPV0Offset {
        JPV0Offset {
            address: (((val as u16) << 8) | low as u16),
        }
    }
}

impl Opcode for JPV0Offset {
    fn execute(&self, _processor: &mut Processor) {
        unimplemented!()
    }
}

impl fmt::Display for JPV0Offset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JPV0Offset {:#X}", self.address)
    }
}

impl Opcode for SEVxByte {
    fn execute(&self, processor: &mut Processor) {
        if processor.memory.registers.v[self.reg as usize] == self.byte {
            processor.memory.registers.pc += 2;
        }
    }

    fn assemble(&self) -> (u8, u8) {
        (3 << 4 | self.reg, self.byte)
    }
}

impl fmt::Display for SEVxByte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SE V{:X}, {:#X}", self.reg, self.byte)
    }
}

impl Opcode for SNEVxByte {
    fn execute(&self, processor: &mut Processor) {
        if processor.memory.registers.v[self.reg as usize] != self.byte {
            processor.memory.registers.pc += 2;
        }
    }

    fn assemble(&self) -> (u8, u8) {
        (4 << 4 | self.reg, self.byte)
    }
}

impl fmt::Display for SNEVxByte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SNE V{:X}, {:#X}", self.reg, self.byte)
    }
}

//@TODO: Implement and fix
impl Opcode for SEVxVy {
    fn execute(&self, _processor: &mut Processor) {
        unimplemented!();
    }

    fn assemble(&self) -> (u8, u8) {
        (5 << 4 | self.reg_a, self.reg_b << 4)
    }
}

impl fmt::Display for SEVxVy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SE V{:X}, V{:X}", self.reg_a, self.reg_b)
    }
}

impl Opcode for LDVxByte {
    fn execute(&self, processor: &mut Processor) {
        processor.memory.registers.v[self.reg as usize] = self.byte;
    }

    fn assemble(&self) -> (u8, u8) {
        (6 << 4 | self.reg, self.byte)
    }
}

impl fmt::Display for LDVxByte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD V{:X}, {:#X}", self.reg, self.byte)
    }
}

impl Opcode for ADDVxByte {
    fn execute(&self, processor: &mut Processor) {
        processor.memory.registers.v[self.reg as usize] += self.byte;
    }

    fn assemble(&self) -> (u8, u8) {
        (7 << 4 | self.reg, self.byte)
    }
}

impl fmt::Display for ADDVxByte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADD V{:X}, {:#X}", self.reg, self.byte)
    }
}

impl Opcode for LDVxVy {
    fn execute(&self, _processor: &mut Processor) {
        unimplemented!();
    }

    fn assemble(&self) -> (u8, u8) {
        (8 << 4 | self.reg_a, self.reg_b << 4)
    }
}

impl fmt::Display for LDVxVy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD V{:X}, V{:X}", self.reg_a, self.reg_b)
    }
}

impl Opcode for ORVxVy {
    fn execute(&self, _processor: &mut Processor) {
        unimplemented!();
    }

    fn assemble(&self) -> (u8, u8) {
        (8 << 4 | self.reg_a, self.reg_b << 4 | 1)
    }
}

impl fmt::Display for ORVxVy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OR V{:X}, V{:X}", self.reg_a, self.reg_b)
    }
}

impl Opcode for ANDVxVy {
    fn execute(&self, _processor: &mut Processor) {
        unimplemented!();
    }

    fn assemble(&self) -> (u8, u8) {
        (8 << 4 | self.reg_a, self.reg_b << 4 | 2)
    }
}

impl fmt::Display for ANDVxVy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AND V{:X}, V{:X}", self.reg_a, self.reg_b)
    }
}

impl Opcode for XORVxVy {
    fn execute(&self, _processor: &mut Processor) {
        unimplemented!();
    }

    fn assemble(&self) -> (u8, u8) {
        (8 << 4 | self.reg_a, self.reg_b << 4 | 3)
    }
}

impl fmt::Display for XORVxVy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "XOR V{:X}, V{:X}", self.reg_a, self.reg_b)
    }
}

impl Opcode for ADDVxVy {
    fn execute(&self, processor: &mut Processor) {
        let res: u16 = processor.memory.registers.v[self.reg_a as usize] as u16
            + processor.memory.registers.v[self.reg_b as usize] as u16;
        if res > 255 {
            processor.memory.registers.v[0xF as usize] = 1;
        } else {
            processor.memory.registers.v[0xF as usize] = 0;
        }

        processor.memory.registers.v[self.reg_a as usize] = (res & 0x00FF) as u8;
    }

    fn assemble(&self) -> (u8, u8) {
        (8 << 4 | self.reg_a, self.reg_b << 4 | 4)
    }
}

impl fmt::Display for ADDVxVy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADD V{:X}, V{:X}", self.reg_a, self.reg_b)
    }
}

impl Opcode for SUBVxVy {
    fn execute(&self, processor: &mut Processor) {
        let res: i16 = processor.memory.registers.v[self.reg_a as usize] as i16
            - processor.memory.registers.v[self.reg_b as usize] as i16;
        if res < 0 {
            processor.memory.registers.v[0xF as usize] = 1;
        } else {
            processor.memory.registers.v[0xF as usize] = 0;
        }

        processor.memory.registers.v[self.reg_a as usize] = (std::cmp::max(res, 0) & 0x00FF) as u8;
    }

    fn assemble(&self) -> (u8, u8) {
        (8 << 4 | self.reg_a, self.reg_b << 4 | 5)
    }
}

impl fmt::Display for SUBVxVy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SUB V{:X}, V{:X}", self.reg_a, self.reg_b)
    }
}

impl Opcode for SHRVxVy {
    fn execute(&self, _processor: &mut Processor) {
        unimplemented!();
    }

    fn assemble(&self) -> (u8, u8) {
        (8 << 4 | self.reg_a, self.reg_b << 4 | 6)
    }
}

impl fmt::Display for SHRVxVy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SHR V{:X}, V{:X}", self.reg_a, self.reg_b)
    }
}

impl Opcode for SUBNVxVy {
    fn execute(&self, _processor: &mut Processor) {
        unimplemented!();
    }

    fn assemble(&self) -> (u8, u8) {
        (8 << 4 | self.reg_a, self.reg_b << 4 | 7)
    }
}

impl fmt::Display for SUBNVxVy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SUBN V{:X}, V{:X}", self.reg_a, self.reg_b)
    }
}

impl Opcode for SHLVxVy {
    fn execute(&self, _processor: &mut Processor) {
        unimplemented!();
    }

    fn assemble(&self) -> (u8, u8) {
        (8 << 4 | self.reg_a, self.reg_b << 4 | 0xE)
    }
}

impl fmt::Display for SHLVxVy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SHL V{:X}, V{:X}", self.reg_a, self.reg_b)
    }
}

impl Opcode for SNEVxVy {
    fn execute(&self, _processor: &mut Processor) {
        unimplemented!();
    }
}

impl fmt::Display for SNEVxVy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SNE V{:X}, V{:X}", self.reg_a, self.reg_b)
    }
}

impl Opcode for LDVxDT {
    fn execute(&self, processor: &mut Processor) {
        //@FIXME @TODO: Currently we have no feeling for time, especially not when stepping through, so we just decrease the timer every time it is accessed
        processor.memory.registers.dt -= 1;
        processor.memory.registers.v[self.reg as usize] = processor.memory.registers.dt;
    }

    fn assemble(&self) -> (u8, u8) {
        (0xF << 4 | self.reg, 0x7)
    }
}

impl fmt::Display for LDVxDT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD V{:X}, DT", self.reg)
    }
}

impl Opcode for LDDTVx {
    fn execute(&self, processor: &mut Processor) {
        processor.memory.registers.dt = processor.memory.registers.v[self.reg as usize];
    }

    fn assemble(&self) -> (u8, u8) {
        (0xF << 4 | self.reg, 0x15)
    }
}

impl fmt::Display for LDDTVx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD DT, V{:X}", self.reg)
    }
}

impl Opcode for LDVxK {
    fn execute(&self, processor: &mut Processor) {
        processor.memory.registers.v[self.reg as usize] = processor.keyboard.blocking_read();
    }

    fn assemble(&self) -> (u8, u8) {
        (0xF << 4 | self.reg, 0xA)
    }
}

impl fmt::Display for LDVxK {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD V{:X}, K", self.reg)
    }
}

impl Opcode for LDSTVx {
    fn execute(&self, _processor: &mut Processor) {
        unimplemented!()
    }

    fn assemble(&self) -> (u8, u8) {
        (0xF << 4 | self.reg, 0x18)
    }
}

impl fmt::Display for LDSTVx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO LDSTVx")
    }
}

impl Opcode for ADDIVx {
    fn execute(&self, processor: &mut Processor) {
        processor.memory.registers.i += processor.memory.registers.v[self.reg as usize] as u16;
    }

    fn assemble(&self) -> (u8, u8) {
        (0xF << 4 | self.reg, 0x1E)
    }
}

impl fmt::Display for ADDIVx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO ADDIVx")
    }
}

impl Opcode for LDFVx {
    fn execute(&self, processor: &mut Processor) {
        let vx = processor.memory.registers.v[self.reg as usize];
        if vx > 0xF {
            panic!(
                "Invalid Opcode: Value in Register V{} exceeds 0xF.",
                self.reg
            );
        }

        processor.memory.registers.i = (6 * vx) as u16;
    }

    fn assemble(&self) -> (u8, u8) {
        (0xF << 4 | self.reg, 0x29)
    }
}

impl fmt::Display for LDFVx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD F, V{:X}", self.reg)
    }
}

impl Opcode for LDBVx {
    fn execute(&self, processor: &mut Processor) {
        //@TODO: Validate since this has nothing to do with real BCD here.
        // Store BCD representation of Vx in I, I+1 and I+2 (100s, 10s, 1s)
        let vx = processor.memory.registers.v[self.reg as usize];
        let hundreds: u8 = vx / 100;
        let tens: u8 = (vx - hundreds) / 10;
        let ones: u8 = vx - hundreds - tens;
        let i = processor.memory.registers.i;
        processor.memory.ram[i as usize] = hundreds;
        processor.memory.ram[(i + 1) as usize] = tens;
        processor.memory.ram[(i + 2) as usize] = ones;
    }

    fn assemble(&self) -> (u8, u8) {
        (0xF << 4 | self.reg, 0x33)
    }
}

impl fmt::Display for LDBVx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD B, V{:X}", self.reg)
    }
}

impl Opcode for LDIVx {
    fn execute(&self, _processor: &mut Processor) {
        unimplemented!()
    }

    fn assemble(&self) -> (u8, u8) {
        (0xF << 4 | self.reg, 0x55)
    }
}

impl fmt::Display for LDIVx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO")
    }
}

impl Opcode for LDVxI {
    fn execute(&self, processor: &mut Processor) {
        let i = processor.memory.registers.i;
        for x in 0..=self.reg {
            //dbg!((i + x as u16) as usize);
            //dbg!(processor.memory.ram[(i + x as u16) as usize]);
            processor.memory.registers.v[x as usize] =
                processor.memory.ram[(i + x as u16) as usize];
            //dbg!(processor.memory.registers.v[x as usize]);
            //dbg!(x);
        }
    }

    fn assemble(&self) -> (u8, u8) {
        (0xF << 4 | self.reg, 0x65)
    }
}

impl fmt::Display for LDVxI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LD V{:X}, I", self.reg)
    }
}

impl Opcode for SKPKBRDVx {
    fn execute(&self, _processor: &mut Processor) {}
}

impl fmt::Display for SKPKBRDVx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO")
    }
}

impl Opcode for SKNPBRDVx {
    fn execute(&self, _processor: &mut Processor) {}
}

impl fmt::Display for SKNPBRDVx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TODO")
    }
}

impl DRW {
    fn ld_sprite(&self, processor: &mut Processor) -> Vec<bool> {
        let i = processor.memory.registers.i as usize;
        let mut vec: Vec<bool> = Vec::new();

        for byte in &processor.memory.ram[i..i + self.size as usize] {
            for x in 0..8 {
                // Note: The inversion here is necessary, because the highest bit is the first one
                vec.push((byte & (1 << (7 - x))) != 0);
            }
        }

        vec
    }
}

impl Opcode for DRW {
    fn execute(&self, processor: &mut Processor) {
        processor.memory.registers.v[0xF] = 0;
        let vx = processor.memory.registers.v[self.reg_x as usize] as u16;
        let vy = processor.memory.registers.v[self.reg_y as usize] as u16;
        let vec_sprite = self.ld_sprite(processor);

        for y in 0..self.size {
            for x in 0..8 {
                if processor.display.screen[((vy + y as u16) * 64 + vx + x as u16) as usize]
                    != vec_sprite[(y * 8 + x) as usize]
                {
                    processor.memory.registers.v[0xF] = 1;
                }

                processor.display.screen[((vy + y as u16) * 64 + vx + x as u16) as usize] =
                    vec_sprite[(y * 8 + x) as usize];
            }
        }
    }

    fn assemble(&self) -> (u8, u8) {
        (0xD << 4 | self.reg_x, self.reg_y << 4 | self.size)
    }
}

impl fmt::Display for DRW {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DRW V{:X}, V{:X}, {:X}",
            self.reg_x, self.reg_y, self.size
        )
    }
}

impl Opcode for RNDVxByte {
    fn execute(&self, processor: &mut Processor) {
        // We need as u8 here, because otherwise rust says 256 > 1 Byte, even though the border is exclusive
        // that way a u16 or larger is generated (but still in the range of 0..255
        let rnd: u8 = rand::thread_rng().gen_range(0, 256) as u8; // [0, 255]
        dbg!(rnd);
        dbg!(self.byte);
        processor.memory.registers.v[self.reg as usize] = rnd & self.byte;
        dbg!(processor.memory.registers.v[self.reg as usize]);
    }
}

impl fmt::Display for RNDVxByte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RND V{:X}, {:X}", self.reg, self.byte)
    }
}
