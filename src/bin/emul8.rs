extern crate emul8;
extern crate clap;
use emul8::internals::*;
use emul8::internals::opcode::Opcode;
use clap::{Arg, App};
use std::io::{stdin};

fn main() {
    let args = App::new("CHIP-8 Emulator")
        .version("0.1")
        .author("Marc Streckfuß <marc.streckfuss@gmail.com>")
        .about("Emulator for the CHIP-8 Binary Format")
        .arg(Arg::with_name("infile")
            .index(1)
            .value_name("FILE")
            .help("Determines the file to execute")
            .default_value("delay.ch8"))
        .arg(Arg::with_name("loadingpoint")
            .long("loadingpoint")
            .help("Where the Emulator should load the file into (the default value is fine for programs, but if you have a full memory dump, you need to use 0x0)")
            .default_value("0x200"))
        .arg(Arg::with_name("entrypoint")
            .long("entrypoint")
            .help("The entry point of the binary file (from which address to start) in hex.")
            .default_value("0x200"))
        .arg(Arg::with_name("verbosity")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .arg(Arg::with_name("debug")
            .short("D")
            .long("debug")
            .help("Run the emulator as Debugger, where you have a command line to evaluate registers and control control-flow."))
        /*.arg(Arg::with_name("dont-ignore-errors")
            .long("dont-ignore-errors")
            .help("Error on the first Invalid Opcode. If not set, just ignore invalid opcodes and continue disassembling (when data is interleaved with code)."))
        .arg(Arg::with_name("dont-stop-on-zerobytes")
            .short("z")
            .long("dont-stop-on-zerobytes")
            .help("If this flag is present, the disassembling process will run until 0x1000 and not stop at the first 0x0 0x0 sequence."))
        .arg(Arg::with_name("canonical")
            .short("c")
            .long("canonical")
            .help("Use Canonical Output, that is: Can pipe the output directly into an .asm file"))*/
        .get_matches();

    let verbosity = std::cmp::min(args.occurrences_of("verbosity"), 2);
    //let canonical = args.is_present("canonical");

    let ep = u16::from_str_radix(args.value_of("entrypoint").unwrap().trim_start_matches("0x"), 16).expect("Unable to parse the entrypoint value");
    let lp = u16::from_str_radix(args.value_of("loadingpoint").unwrap().trim_start_matches("0x"), 16).expect("Unable to parse the loadingpoint value");
    //let ignore_errors = !args.is_present("dont-ignore-errors");
    //let stop_zero = !args.is_present("dont-stop-on-zerobytes");

    let mut processor = processor::Processor {
        memory: memory::Memory::default(),
        keyboard: keyboard::Keyboard::default(),
        display: display::Display::default()
    };

    processor.memory.load_from_file(args.value_of("infile").unwrap(), lp)
        .unwrap_or_else(|_| panic!("Failed to load the input file {}", args.value_of("infile").unwrap()));
    processor.memory.registers.pc = ep;

    if verbosity > 0 {
        println!("Emulating file {} starting at {:#X}, loaded at {:#X}", args.value_of("infile").unwrap(), ep, lp);
    }

    // @TODO: breakpoints!!!
    if args.is_present("debug") {
        let mut paused = true; // Start the debugger paused.

        loop {
            if processor.memory.registers.pc > 4094 {
                panic!("Exceeded Memory at pc={:#X}", processor.memory.registers.pc);
            }
            
            if paused {
                let opcode = processor.fetch_opcode();
                let op: Box<dyn Opcode> = processor.decode_opcode(opcode);
                if verbosity > 0 {
                    println!("<Memory Address>\t<Opcodes>\t<Assembler>");
                }
                println!("{:#X}\t\t\t{:#X} {:#X}\t{}", processor.memory.registers.pc, opcode.0, opcode.1, op);

                loop {
                    let mut cmd_line = String::new();
                    stdin().read_line(&mut cmd_line).expect("Error when reading input");

                    if cmd_line.starts_with("p ") {
                        let addr_s = cmd_line.trim_start_matches("p ").trim();

                        let addr = if addr_s.starts_with("0x") {
                            u16::from_str_radix(addr_s.trim_start_matches("0x"), 16).unwrap()
                        } else {
                            u16::from_str_radix(addr_s, 10).unwrap()
                        };
                        
                        let opc = processor.memory.read_two_u8(addr);
                        let opd = processor.decode_opcode(opc);

                        if verbosity > 0 {
                            println!("<Memory Address>\t<Opcodes>\t<Assembler>");
                        }
                        println!("{:#X}\t\t\t{:#X} {:#X}\t{}", addr, opc.0, opc.1, opd);
                    } else {
                        match cmd_line.trim() {
                            "s" => {
                                op.execute(&mut processor);
                                if !op.modified_pc() {
                                    processor.memory.registers.pc += 2;
                                }
                                break;
                            },
                            "q" => return,
                            "r" => println!("{}", processor.memory.registers),
                            "st" => println!("{:x?}", processor.memory.stack),
                            "d" => println!("{}", processor.display),
                            "i" => {
                                if verbosity > 0 {
                                    println!("<Memory Address>\t<Opcodes>\t<Assembler>");
                                }
                                println!("{:#X}\t\t\t{:#X} {:#X}\t{}", processor.memory.registers.pc, opcode.0, opcode.1, op);
                            },
                            "c" => {
                                println!("Continueing execution. Will only stop at a breakpoint again!");
                                paused = false;
                                break;
                            }
                            _ => println!("Syntax Error."),
                        }
                    }
                }
            } else {
                processor.tick();
            }
        }
    } else {
        unimplemented!()
    }
    
    /*while processor.memory.registers.pc <= 4094 {
        
        //if opcode.0 != 0 || opcode.1 != 0 || verbosity == 2 
        if op.to_string() != "INVALID" {
            if !canonical {
                println!("{:#X}\t\t\t{:#X} {:#X}\t{}", processor.memory.registers.pc, opcode.0, opcode.1, op);
            } else {
                println!("{} ; {:#X}", op, processor.memory.registers.pc);
            }
        } else {
            if opcode.0 == 0 && opcode.1 == 0 {
                if stop_zero {
                    panic!("Stopping disassembly as the first 0x0000 data has been reached");
                } else if verbosity == 2 {
                    if !canonical {
                        println!("{:#X}\t\t\t{:#X} {:#X}\t{}", processor.memory.registers.pc, opcode.0, opcode.1, op);
                    } else {
                        println!("{} ; {:#X}", op, processor.memory.registers.pc);
                    }
                }
            }
            else if !ignore_errors {
                panic!("Got an Invalid Opcode, probably reached the end of the file or a data sector.")
            } else if verbosity == 2 {
                if !canonical {
                    println!("{:#X}\t\t\t{:#X} {:#X}\t{}", processor.memory.registers.pc, opcode.0, opcode.1, op);
                } else {
                    println!("{} ; {:#X}", op, processor.memory.registers.pc);
                }
            }
        }

        processor.memory.registers.pc += 2;
    }

    /*loop {
        processor.tick();
        //processor.disassemble_tick();
    }
    dbg!(processor.memory);*/*/
}
