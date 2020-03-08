#[macro_use]
extern crate downcast_rs;

pub mod internals {
    pub mod display;
    pub mod keyboard;
    pub mod memory;
    pub mod opcode;
    pub mod processor;
}
