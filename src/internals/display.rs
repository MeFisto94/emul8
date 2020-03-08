pub struct Display {
    pub screen: [bool; 64*32]
}

impl Display {
    pub fn new() -> Display {
        Display {screen: [false; 64*32]}
    }
}

impl std::fmt::Display for Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..32 {
            for x in 0..64 {
                if self.screen[64 * y + x] {
                    write!(f, "*")?;
                } else {
                    write!(f, " ")?;
                }

                if x != 63 {
                    write!(f, " ")?;
                } else {
                    write!(f, "\n")?;
                }   
            }
        }

        Ok(())
    }
}
