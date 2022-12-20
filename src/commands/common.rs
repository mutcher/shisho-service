use std::{fmt, net::TcpStream};

pub trait ShishoCommand {
    fn perform(args: &[String], stream: &mut TcpStream);
}

#[derive(Debug)]
#[derive(Default)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub year: Option<u32>,
    pub url: Option<String>,
    pub journal: Option<String>,
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.title == other.title
            && self.authors == other.authors
            && self.year == other.year
            && self.url == other.url
            && self.journal == other.journal;
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "title: {}, authors: {}, year: {}, url: {}",
            self.title.clone(),
            self.authors.join(","),
            self.year.clone().unwrap_or(0),
            self.url.clone().unwrap_or("".to_owned())
        )
    }
}
