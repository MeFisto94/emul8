extern crate emul8;
extern crate clap;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use emul8::internals::opcode::*;
use emul8::internals::opcode::Opcode;
use emul8::internals::processor::*;
use clap::{Arg, App};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};

#[derive(Parser)]
#[grammar = "grammar/asm.pest"]
pub struct ASMParser;

pub struct LabelDefinition {
    pub name: String,
    pub addr: u16
}

#[derive(Debug)]
pub struct JmpLabel {
    pub name: String
}

#[derive(Debug)]
pub struct CallLabel {
    pub name: String
}

impl Opcode for JmpLabel {
    fn execute(&self, _processor: &mut Processor) {
        panic!("This Opcode is not meant to be executed and should be replaced by the assembler!");
    }
    fn modified_pc(&self) -> bool {
        return false; // The majority does not tamper with the PC
    }
    fn assemble(&self) -> (u8, u8) {
        panic!("This Opcode is not meant to be assembled and should be replaced by the assembler!");
    }
}

impl std::fmt::Display for JmpLabel {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unreachable!()
    }
}

impl Opcode for  CallLabel {
    fn execute(&self, _processor: &mut Processor) {
        panic!("This Opcode is not meant to be executed and should be replaced by the assembler!");
    }
    fn assemble(&self) -> (u8, u8) {
        panic!("This Opcode is not meant to be assembled and should be replaced by the assembler!");
    }
}

impl std::fmt::Display for CallLabel {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unreachable!()
    }
}

// @TODO: Replace min() with assertions
fn parse_register(pair: pest::iterators::Pair<Rule>) -> u8 {
    std::cmp::min(0xF, u8::from_str_radix(pair.as_span().as_str().trim_start_matches('V'), 16).unwrap())
}

fn parse_constant(pair: pest::iterators::Pair<Rule>) -> u8 {
    let addr_s = pair.as_span().as_str();
    let addr: u16;
    if addr_s.starts_with("0x") {
        addr = u16::from_str_radix(addr_s.trim_start_matches("0x"), 16).unwrap();
    } else {
        addr = u16::from_str_radix(addr_s, 10).unwrap();
    }
    
    if addr > 0xFF {
        panic!("Syntax Error: Constant {} too large!", addr_s);
    }

    addr as u8
}

fn parse_address(pair: pest::iterators::Pair<Rule>) -> u16 {
    let addr_s = pair.as_span().as_str();
    let addr: u16;
    if addr_s.starts_with("0x") {
        addr = u16::from_str_radix(addr_s.trim_start_matches("0x"), 16).unwrap();
    } else {
        addr = u16::from_str_radix(addr_s, 10).unwrap();
    }
    
    if addr > 0x1000 {
        panic!("Syntax Error: Address out of boundaries: {}", addr_s);
    }

    addr
}

fn main() {
    let args = App::new("CHIP-8 Assembler")
        .version("0.1")
        .author("Marc Streckfuß <marc.streckfuss@gmail.com>")
        .about("Assembler for the CHIP-8 Binary Format")
        .arg(Arg::with_name("infile")
            .index(1)
            .value_name("FILE")
            .help("Determines the file to assemble")
            .default_value("delay.as8"))
        .arg(Arg::with_name("offset")
            .long("offset")
            .help("Where the entry point is (the offset from the file start where to put the data)")
            .default_value("0x200"))
        .arg(Arg::with_name("outfile")
            .long("outfile")
            .short("o")
            .help("How the resulting binary should be called (if empty: guess from input filename)")
            .value_name("FILE"))
        .arg(Arg::with_name("verbosity")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .arg(Arg::with_name("overwrite")
            .long("overwrite")
            .help("If $filename is already taken, overwrite the file. Without this flag the attempt will fail"))
        .get_matches();

    let verbosity = std::cmp::min(args.occurrences_of("verbosity"), 2);

    let offset = u16::from_str_radix(args.value_of("offset").unwrap().trim_start_matches("0x"), 16).expect("Unable to parse the offset value");
    let outfilename: String;

    if args.is_present("outfile") {
        outfilename = args.value_of("outfile").unwrap().to_string();
    } else {
        outfilename = format!("{}{}", args.value_of("infile").unwrap().trim_end_matches(".as8"), ".obj");
    }

    if verbosity > 0 {
        println!("Assembling {} as {}, starting at offset {:#X}", args.value_of("infile").unwrap(), &outfilename, offset);
    }
 
    /* Messy code, is there a more simple solution? */
    let outfile = match File::open(&outfilename) {
        Err(_e) => File::create(&outfilename).unwrap(),
        Ok(_f) => if args.is_present("overwrite")  { OpenOptions::new().write(true).open(&outfilename).unwrap() } else { panic!("Won't overwrite the output file!")},
    };

    // into_inner to not have file as Rule but all the expressions
    let contents = std::fs::read_to_string(args.value_of("infile").unwrap()).expect("Cannot read input file");
    let parse_file = ASMParser::parse(Rule::file, &contents).expect("Parser Error").next().unwrap().into_inner();
    //dbg!(parseFile);

    let mut opcodes = Vec::new();
    let mut label_definitions = std::collections::HashMap::new();

    for pair in parse_file {
        //Rule::COMMENT => println!("Comment: {}", pair.as_span().as_str()),
        if pair.as_rule() == Rule::opcode {
            let opcode: Option<Box<dyn Opcode>> = match pair.as_span().as_str() {
                "CLS" => Some(Box::new(CLS{})),
                "RET" => Some(Box::new(RET{})),
                _ => {
                    let opcode_str = pair.as_span().as_str();
                    let mut opcode_node = &mut pair.into_inner();
                    let operator = opcode_node.next().unwrap();
                    match operator.as_rule() {
                        Rule::ld_operator => {
                            let operand1 = opcode_node.next().unwrap();
                            let operand2 = opcode_node.next().unwrap();
                            Some(match operand1.as_rule() {
                                Rule::register => {
                                    match operand2.as_rule() {
                                        Rule::special_register => {
                                            match operand2.as_span().as_str() {
                                                "K" => Box::new(LDVxK{reg: parse_register(operand1)}),
                                                "DT" => Box::new(LDVxDT{reg: parse_register(operand1)}),
                                                "I" => Box::new(LDVxI{reg: parse_register(operand1)}),
                                                _ => unreachable!("Invalid special register")
                                            }
                                        },
                                        Rule::address => {
                                            Box::new(LDVxByte{reg: parse_register(operand1), byte: parse_constant(operand2)})
                                        }
                                        _ => unreachable!()
                                    }
                                },
                                Rule::special_register => {
                                    let register = parse_register(operand2);
                                    match operand1.as_span().as_str() {
                                        "B" => Box::new(LDBVx{reg: register}),
                                        "F" => Box::new(LDFVx{reg: register}),
                                        "I" => Box::new(LDIVx{reg: register}),
                                        "DT" => Box::new(LDDTVx{reg: register}),
                                        _ => unreachable!("Invalid special register")
                                    }
                                }
                                _ => unreachable!()
                            })
                        },
                        Rule::call_operator => {
                            let operand = opcode_node.next().unwrap();
                            Some(match operand.as_rule() {
                                Rule::address => Box::new(CALL{address: parse_address(operand)}),
                                Rule::identifier => Box::new(CallLabel{name: operand.as_span().as_str().to_string()}),
                                _ => unreachable!("Unknown CALL Operand {:?}", operand.as_rule())
                            })
                        },
                        Rule::jmp_operator => {
                            let operand = opcode_node.next().unwrap();
                            Some(match operand.as_rule() {
                                Rule::address => Box::new(JMP{address: parse_address(operand)}),
                                Rule::identifier => Box::new(JmpLabel{name: operand.as_span().as_str().to_string()}),
                                _ => unreachable!("Unknown JMP Operand {:?}", operand.as_rule())
                            })
                        },
                        Rule::conditionals => {
                            let register = parse_register(opcode_node.next().unwrap());
                            let op2 = opcode_node.next().unwrap();

                            Some(match operator.as_span().as_str() {
                                "SE" => {
                                    match op2.as_rule() {
                                        Rule::register => Box::new(SEVxVy{reg_a: register, reg_b: parse_register(op2)}),
                                        Rule::address => Box::new(SEVxByte{reg: register, byte: parse_constant(op2)}),
                                        _ => unreachable!()
                                    }
                                },
                                "SNE" => {
                                    match op2.as_rule() {
                                        Rule::register => Box::new(SNEVxVy{reg_a: register, reg_b: parse_register(op2)}),
                                        Rule::address => Box::new(SNEVxByte{reg: register, byte: parse_constant(op2)}),
                                        _ => unreachable!()
                                    }
                                },
                                _ => unreachable!()
                            })
                        },
                        Rule::math_operator => {
                            let op1 = opcode_node.next().unwrap();
                            let op2 = opcode_node.next().unwrap();

                            Some(match operator.as_span().as_str() {
                                "ADD" => {
                                    match op1.as_rule() {
                                        Rule::special_register => {
                                            assert_eq!(op1.as_span().as_str(), "I");
                                            Box::new(ADDIVx{reg: parse_register(op2)})
                                        },
                                        Rule::register => {
                                            match op2.as_rule() {
                                                Rule::register => {
                                                    Box::new(ADDVxVy{reg_a: parse_register(op1), reg_b: parse_register(op2)})
                                                },
                                                Rule::address => {
                                                    Box::new(ADDVxByte{reg: parse_register(op1), byte: parse_constant(op2)})
                                                },
                                                _ => unreachable!()
                                            }
                                        },
                                        _ => unreachable!()
                                    }
                                },
                                "SUB" => Box::new(SUBVxVy{reg_a: parse_register(op1), reg_b: parse_register(op2)}),
                                "SUBN" => unimplemented!(),
                                _ => unreachable!()
                            })
                        },
                        Rule::ldi_operator => {
                            let op = opcode_node.next().unwrap();
                            
                            Some(match op.as_rule() {
                                Rule::register => Box::new(LDIVx{reg: parse_register(op)}),
                                Rule::address => Box::new(LDIAddr{address: parse_address(op)}),
                                _ => unreachable!()
                            })
                        },
                        Rule::drw_operator => Some(Box::new(DRW{
                            reg_x: parse_register(opcode_node.next().unwrap()),
                            reg_y: parse_register(opcode_node.next().unwrap()),
                            size: std::cmp::min(0xF, parse_constant(opcode_node.next().unwrap()))}
                        )),
                        // Byte arithmetic is only allowed here because we know the string is no Unicode.
                        Rule::label_definition => {
                            let s = &opcode_str[..opcode_str.len() - 1];
                            label_definitions.insert(s.to_string(), Box::new(LabelDefinition{name: s.to_string(), addr: (opcodes.len() * 2) as u16}));
                            None
                        },
                        _ => {
                            panic!("Unknown OPCODE {}", opcode_str)
                        }
                    }
                }
            };
            
            match opcode {
                Some(op) => opcodes.push(op),
                _ => ()
            }
        }
    }

    if verbosity > 1 {
        /* In practice, if we were to be invoked from a C-Compiler, we would keep our intermediary results in opcodes as an object file.
            Ideally we would already pre-assemble all opcodes which are possible into a file and keep the label definitions and the label dependants
            in place. While we could already solve in-file labels, we don't do it so that object files are "relocateable", because otherwise we'd have
            to shift them with a static offset anyway. We probably won't do that to not have a seperate object file format and instead just
            paste all files together into a big asm file.
        */
        println!("Entering Linking Stage...")
    }

    for (_idx, op) in opcodes.iter_mut().enumerate() {
        if op.is::<JmpLabel>() {
            let lbl = op.downcast_ref::<JmpLabel>().unwrap();
            if label_definitions.contains_key(&lbl.name) {
                let jmp: Box<dyn Opcode> = Box::new(JMP{address: label_definitions.get(&lbl.name).unwrap().addr + offset});
                *op = jmp; // like std::mem::replace(op, jmp), but doesn't care about the old value
            } else {
                panic!("ERROR LNK001: Unresolved Label {}", &lbl.name);
            }
        } else if op.is::<CallLabel>() {
            let lbl = op.downcast_ref::<CallLabel>().unwrap();
            if label_definitions.contains_key(&lbl.name) {
                let call: Box<dyn Opcode> = Box::new(CALL{address: label_definitions.get(&lbl.name).unwrap().addr + offset});
                *op = call;
            } else {
                panic!("ERROR LNK001: Unresolved Label {}", &lbl.name);
            }
        }
    }

    let mut buf = BufWriter::new(outfile);
    opcodes.iter().map(|x| x.assemble()).for_each(move |x| {
        buf.write(&[x.0]).expect("Error when writing to the object file!");
        buf.write(&[x.1]).expect("Error when writing to the object file!");
    });
}
