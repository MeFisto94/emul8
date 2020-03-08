use std::io::stdin;

pub struct Keyboard {
    pub keys: [bool; 16]
}

impl Default for Keyboard {
    fn default() -> Self {
        Keyboard {keys: [false; 16]}
    }
}

impl Keyboard {
    pub fn do_read(&mut self) {

    }

    pub fn blocking_read(&mut self) -> u8 {
        loop {
            println!("Keyboard Input> ");
            //stdout().flush();
            let mut buffer = String::new();
            stdin().read_line(&mut buffer).expect("Error when reading keyboard input!");
            let buffer = buffer.trim().to_uppercase();
            match buffer.as_str() {
                "0" => return 0,
                "1" => return 1,
                "2" => return 2,
                "3" => return 3,
                "4" => return 4,
                "5" => return 5,
                "6" => return 6,
                "7" => return 7,
                "8" => return 8,
                "9" => return 9,
                "A" => return 0xA,
                "B" => return 0xB,
                "C" => return 0xC,
                "D" => return 0xD,
                "E" => return 0xE,
                "F" => return 0xF,
                _ => println!("Error: Invalid Input, try again!")
            }
        }
    }
}