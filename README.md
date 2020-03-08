# CHIP-8 Toolchain

**Disclaimer**: The motivation behind this project was for me to get acquainted with Rust while working on something relatively meaningful.
It is specifically **no** good example in terms of idiomatic rust, a feature complete tool or anything. Don't expect it to work at all.

## CHIP-8 Platform

The CHIP-8 is an artificial architecture of mostly educational purpose.  
It was written as an interpreter compatible with a few other old architectures.  
There are a plenty sources for programs (games, demos) as well as plenty other documentation for the architecture.

## The Tools

### emul8 - The CHIP-8 Interpreter

This is the main tool provided by this crate. It is used to emulate the CHIP-8 architecture and instructions.  
It is capable of running several applications that are available on the net, however it could be that you run into issues or unsupported Opcodes.  
If that is the case, get in touch with us and raise an issue in [GitHub](https://github.com/MeFisto94/emul8).

In addition to the simple emulation of instructions, emul8 also features a debugger mode where each instruction is disassembled and can be executed much like stepping through code in your favourite IDE.
See: `cargo run --bin emul8 -- --help`

### disasm - The CHIP-8 Disassembler

This tool allows you to disassemble binary files into their mnemonics in order to view the instructions they contain.  
This disassmbler is a _linear_ disassembler as opposed to _recursive_ diassemblers. That means it is very limited in it's abilities and specifically easy to confuse.  

The problem comes from the fact that this disassembler expects instructions to be aligned to every second byte. Most applications add their data after all the code instructions, but it is also legal to embed random data directly inbetween instructions.  
When random data is found, the disassembler tries to disassemble invalid opcodes but more important: it looses the alignment. There is no indication of where the next instruction happens.  

We try to overcome this problem by just skipping invalid opcodes. This however means when opcodes start to get aligned at odd memory addresses, the disassembler will not find any further instructions.

A clever disassembler (_recursive_) is based upon the linear approach but as soon as branchings are encountered, a new instance of the linear disassembler is spawned which starts disassembling until the next branch is encountered.
This ensures that every potential instruction is disassembled as long as it has a reachable address.
Since there are quite a few possibilities to influence the branching and since the emulator is following the branches anyway, I did not attempt to implement recursive disassembly, yet.
Instead of that, just execute the program step by step.

See `cargo run --bin disasm -- --help`

### asm - The CHIP-8 Assembler

If you want to start writing programs for CHIP-8, you can use this very simple assembler to generate binary files, which _should_ be compatible with other CHIP-8 emulators, but they are best consumed by this Toolchain.

Currently this Assembler is very limited, because only the most simple Opcodes have been implemented so far.
More advanced features like linking or including of other files are missing as well.

To understand the syntax, it's probably best to look into `grammar/asm.pest`, but it should correspond to the output of the disassembler.

See `cargo run --bin asm -- --help`

### Using this crate as a dependency

Just look at the binaries' source codes, they all use the same internal data structures and methods.
Specifically `emul8::internals::opcode` and `emul8::internals::processor`.

Copyright 2019 - 2020 Marc Streckfuß, License: MIT
