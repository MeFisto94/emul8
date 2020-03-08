#[macro_use]
extern crate downcast_rs;

pub mod internals {
    pub mod memory;
    pub mod processor;
    pub mod opcode;
    pub mod keyboard;
    pub mod display;
}
