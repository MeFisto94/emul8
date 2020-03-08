use crate::internals::display::Display;
use crate::internals::keyboard::Keyboard;
use crate::internals::memory::Memory;
use crate::internals::opcode::*;

pub struct Processor {
    pub memory: Memory,
    pub keyboard: Keyboard,
    pub display: Display,
}

impl Processor {
    pub fn tick(&mut self) {
        let opcode = self.fetch_opcode();
        let op: Box<dyn Opcode> = self.decode_opcode(opcode);
        if !op.modified_pc() {
            self.memory.registers.pc += 2;
        }
        op.execute(self);
    }

    pub fn disassemble_tick(&mut self) {
        let opcode = self.fetch_opcode();
        self.memory.registers.pc += 2;
        let op: Box<dyn Opcode> = self.decode_opcode(opcode);
        if opcode.0 != 0 || opcode.1 != 0 {
            dbg!(op);
        } else {
            panic!("Finished probably");
        }
    }

    pub fn fetch_opcode(&mut self) -> (u8, u8) {
        //println!("Fetching from: {:#X}", self.memory.registers.pc);
        self.memory.read_two_u8(self.memory.registers.pc)
    }

    pub fn decode_opcode(&mut self, opcode: (u8, u8)) -> Box<dyn Opcode> {
        //println!("=> Decoding {:#X} {:#X}", opcode.0, opcode.1);

        // Simple Opcodes which don't require bit shifting.
        match opcode {
            (0x0, 0xE0) => return Box::new(CLS {}),
            (0x0, 0xEE) => return Box::new(RET {}),
            _ => (),
        };

        // selector, the 4 highest bits
        let sel = (opcode.0 & 0xF0) >> 4;
        // value, the 2nd most highest 4 bits
        let val = opcode.0 & 0x0F;

        //println!("===> Decoded HIGH: selector={:#X}, value={:#X}", sel, val);

        match sel {
            1 => Box::new(JMP::new(val, opcode.1)),
            2 => Box::new(CALL::new(val, opcode.1)),
            3 => Box::new(SEVxByte {
                reg: val,
                byte: opcode.1,
            }),
            4 => Box::new(SNEVxByte {
                reg: val,
                byte: opcode.1,
            }),
            5 => Box::new(SEVxVy {
                reg_a: val,
                reg_b: ((opcode.1 & 0xF0) >> 4),
            }),
            6 => Box::new(LDVxByte {
                reg: val,
                byte: opcode.1,
            }),
            7 => Box::new(ADDVxByte {
                reg: val,
                byte: opcode.1,
            }),
            8 => match opcode.1 & 0x0F {
                0 => Box::new(LDVxVy {
                    reg_a: val,
                    reg_b: ((opcode.1 & 0xF0) >> 4),
                }),
                1 => Box::new(ORVxVy {
                    reg_a: val,
                    reg_b: ((opcode.1 & 0xF0) >> 4),
                }),
                2 => Box::new(ANDVxVy {
                    reg_a: val,
                    reg_b: ((opcode.1 & 0xF0) >> 4),
                }),
                3 => Box::new(XORVxVy {
                    reg_a: val,
                    reg_b: ((opcode.1 & 0xF0) >> 4),
                }),
                4 => Box::new(ADDVxVy {
                    reg_a: val,
                    reg_b: ((opcode.1 & 0xF0) >> 4),
                }),
                5 => Box::new(SUBVxVy {
                    reg_a: val,
                    reg_b: ((opcode.1 & 0xF0) >> 4),
                }),
                6 => Box::new(SHRVxVy {
                    reg_a: val,
                    reg_b: ((opcode.1 & 0xF0) >> 4),
                }),
                7 => Box::new(SUBNVxVy {
                    reg_a: val,
                    reg_b: ((opcode.1 & 0xF0) >> 4),
                }),
                0xE => Box::new(SHLVxVy {
                    reg_a: val,
                    reg_b: ((opcode.1 & 0xF0) >> 4),
                }),
                _ => Box::new(InvalidOpcode { opcode }),
            },
            9 => Box::new(SNEVxVy {
                reg_a: val,
                reg_b: ((opcode.1 & 0xF0) >> 4),
            }),
            0xA => Box::new(LDIAddr::new(val, opcode.1)),
            0xB => Box::new(JPV0Offset::new(val, opcode.1)),
            0xC => Box::new(RNDVxByte {
                reg: val,
                byte: opcode.1,
            }),
            0xD => Box::new(DRW {
                reg_x: val,
                reg_y: ((opcode.1 & 0xF0) >> 4),
                size: (opcode.1 & 0xF),
            }),
            0xE => match opcode.1 {
                0x9E => Box::new(SKPKBRDVx { reg: val }),
                0xA1 => Box::new(SKNPBRDVx { reg: val }),
                _ => Box::new(InvalidOpcode { opcode }),
            },

            0xF => match opcode.1 {
                7 => Box::new(LDVxDT { reg: val }),
                0xA => Box::new(LDVxK { reg: val }),
                0x15 => Box::new(LDDTVx { reg: val }),
                0x18 => Box::new(LDSTVx { reg: val }),
                0x1E => Box::new(ADDIVx { reg: val }),
                0x29 => Box::new(LDFVx { reg: val }),
                0x33 => Box::new(LDBVx { reg: val }),
                0x55 => Box::new(LDIVx { reg: val }),
                0x65 => Box::new(LDVxI { reg: val }),
                _ => Box::new(InvalidOpcode { opcode }),
            },
            _ => Box::new(InvalidOpcode { opcode }),
        }
    }
}
