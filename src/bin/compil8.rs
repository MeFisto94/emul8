extern crate emul8;
extern crate clap;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use clap::{Arg, App};
use std::fs::{File, OpenOptions};

#[derive(Parser)]
#[grammar = "grammar/asm.pest"]
pub struct ASMParser;

fn main() {
    let args = App::new("CHIP-8 Compiler")
        .version("0.1")
        .author("Marc Streckfuß <marc.streckfuss@gmail.com>")
        .about("Simple C Compiler for the CHIP-8 Binary Format")
        .arg(Arg::with_name("infile")
            .index(1)
            .value_name("FILE")
            .help("Determines the file to compile")
            .default_value("delay.c"))
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
        outfilename = format!("{}{}", args.value_of("infile").unwrap().trim_end_matches(".c"), ".obj");
    }
 
    /* Messy code, is there a more simple solution? */
    let outfile = match File::open(&outfilename) {
        Err(_e) => File::create(&outfilename).unwrap(),
        Ok(_f) => if args.is_present("overwrite")  { OpenOptions::new().write(true).open(&outfilename).unwrap() } else { panic!("Won't overwrite the output file!")},
    };

    unimplemented!();

    // into_inner to not have file as Rule but all the expressions
    let contents = std::fs::read_to_string(args.value_of("infile").unwrap()).expect("Cannot read input file");
    let parse_file = ASMParser::parse(Rule::file, &contents).expect("Parser Error").next().unwrap().into_inner();
    //dbg!(parseFile);

    // see asm.rs for reference
}
