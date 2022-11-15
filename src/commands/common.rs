use std::{net::TcpStream, fmt};

pub trait ShishoCommand {
    fn perform(args: &[String], stream: &mut TcpStream);
}

pub struct Book {
    pub name: String,
    pub authors: String,
    pub year: u32,
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "name: {}, authors: {}, year: {}",
            self.name, self.authors, self.year
        )
    }
}