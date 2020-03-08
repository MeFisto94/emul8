#[macro_use]
extern crate downcast_rs;

pub mod internals {
    pub mod memory;
    pub mod processor;
    pub mod opcode;
    pub mod keyboard;
    pub mod display;
}

/*use crate::internals::memory::Memory;
use crate::internals::processor::Processor;
use crate::internals::keyboard::Keyboard;

fn main() {
    let mut processor = Processor {
        memory: Memory::new(),
        keyboard: Keyboard::new()
    };

    processor.memory.load_from_file("delay.ch8");
    processor.memory.registers.pc = 0x200;


    /*for i in 0..20 {
        processor.tick();
    }*/

    /*processor.tick();
    processor.tick();
    processor.tick();*/

    loop {
        processor.tick();
        //processor.disassemble_tick();
    }
    dbg!(processor.memory);
}*/
