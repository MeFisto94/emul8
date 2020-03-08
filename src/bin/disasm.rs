extern crate emul8;
extern crate clap;
use emul8::internals::*;
use emul8::internals::opcode::Opcode;
use clap::{Arg, App};

fn main() {
    let args = App::new("CHIP-8 Disassembler")
        .version("0.1")
        .author("Marc Streckfuß <marc.streckfuss@gmail.com>")
        .about("Disassembler for the CHIP-8 Binary Format")
        .arg(Arg::with_name("infile")
            /*.short("i")
            .long("input")*/
            .index(1)
            .value_name("FILE")
            .help("Determines the file to disassemble")
            .default_value("delay.ch8")
        )
        // Actually, loadingpoint should only be relevant for a real emulator. For disasm loading point can be entrypoint, as we wont disasm the stuff before that.
        .arg(Arg::with_name("loadingpoint")
            .long("loadingpoint")
            .help("Where the Emulator should load the file into (the default value is fine for programs, but if you have a full memory dump, you need to use 0x0)")
            .default_value("0x200")
        )
        .arg(Arg::with_name("entrypoint")
            .long("entrypoint")
            .help("The entry point of the binary file (from which address to start) in hex.")
            .default_value("0x200")
        )
        .arg(Arg::with_name("verbosity")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity")
        )
        .arg(Arg::with_name("mode")
            .short("m")
            .long("mode")
            .help("Disassembly Mode: a 'linear' disassembler steps through starting with entrypoint and decoding every opcode. a 'recursive' disassembler recursively follows the op code flow (following jumps).")
            .possible_values(&["linear"]) //, "recursive"])
            .default_value("linear")
        )
        .arg(Arg::with_name("dont-ignore-errors")
            .long("dont-ignore-errors")
            .help("Error on the first Invalid Opcode. If not set, just ignore invalid opcodes and continue disassembling (when data is interleaved with code).")
        )
        .arg(Arg::with_name("dont-stop-on-zerobytes")
            .short("z")
            .long("dont-stop-on-zerobytes")
            .help("If this flag is present, the disassembling process will run until 0x1000 and not stop at the first 0x0 0x0 sequence.")
        )
        .arg(Arg::with_name("canonical")
            .short("c")
            .long("canonical")
            .help("Use Canonical Output, that is: Can pipe the output directly into an .asm file")
        ).get_matches();

    let verbosity = std::cmp::min(args.occurrences_of("verbosity"), 2);
    let canonical = args.is_present("canonical");

    let ep = u16::from_str_radix(args.value_of("entrypoint").unwrap().trim_start_matches("0x"), 16).expect("Unable to parse the entrypoint value");
    let lp = u16::from_str_radix(args.value_of("loadingpoint").unwrap().trim_start_matches("0x"), 16).expect("Unable to parse the loadingpoint value");
    let ignore_errors = !args.is_present("dont-ignore-errors");
    let stop_zero = !args.is_present("dont-stop-on-zerobytes");

    let mut processor = processor::Processor {
        display: display::Display::default(),
        memory: memory::Memory::default(),
        keyboard: keyboard::Keyboard::default()
    };

    processor.memory.load_from_file(args.value_of("infile").unwrap(), lp)
        .unwrap_or_else(|_| panic!("Failed to load the input file {}", args.value_of("infile").unwrap()));
    processor.memory.registers.pc = ep;

    if !canonical {
        println!("Disassembling file {} starting at {:#X}", args.value_of("infile").unwrap(), ep);
        println!("<Memory Address>\t<Opcodes>\t<Assembler>");
    }
    
    while processor.memory.registers.pc <= 4094 {
        let opcode = processor.fetch_opcode();
        let op: Box<dyn Opcode> = processor.decode_opcode(opcode);
        //if opcode.0 != 0 || opcode.1 != 0 || verbosity == 2
        if op.to_string() != "INVALID" {
            if !canonical {
                println!("{:#X}\t\t\t{:#X} {:#X}\t{}", processor.memory.registers.pc, opcode.0, opcode.1, op);
            } else {
                println!("{} ; {:#X}", op, processor.memory.registers.pc);
            }
        } else if opcode.0 == 0 && opcode.1 == 0 {
                if stop_zero {
                    panic!("Stopping disassembly as the first 0x00 0x00 data has been reached");
                } else if verbosity == 2 {
                    if !canonical {
                        println!("{:#X}\t\t\t{:#X} {:#X}\t{}", processor.memory.registers.pc, opcode.0, opcode.1, op);
                    } else {
                        println!("{} ; {:#X}", op, processor.memory.registers.pc);
                    }
                }
        } else if !ignore_errors {
            panic!("Got an Invalid Opcode, probably reached the end of the file or a data sector.")
        } else if verbosity == 2 {
            if !canonical {
                println!("{:#X}\t\t\t{:#X} {:#X}\t{}", processor.memory.registers.pc, opcode.0, opcode.1, op);
            } else {
                println!("{} ; {:#X}", op, processor.memory.registers.pc);
            }
        }

        processor.memory.registers.pc += 2;
    }

    /*loop {
        processor.tick();
        //processor.disassemble_tick();
    }
    dbg!(processor.memory);*/
}
