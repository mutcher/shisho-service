use crate::tcp_api::{ApiResponse, TcpStreamed};

use super::{common::Book, ShishoCommand};

pub struct AddCommand;

impl Book {
    fn parse(data: &[String]) -> Result<Self, &str> {
        if data.len() != 3 {
            return Err("Incorrent number of parameters");
        }

        let (name, authors, year_str) = (&data[0], &data[1], &data[2]);

        let book_year = u32::from_str_radix(year_str.as_str(), 10)
            .map_err::<&str, _>(|_| "Cannot parse book year")?;

        return Ok(Book {
            name: name.to_string(),
            authors: authors.to_string(),
            year: book_year,
        });
    }
}

impl ShishoCommand for AddCommand {
    fn perform(args: &[String], stream: &mut std::net::TcpStream) {
        let parsing_result = Book::parse(args);
        let response = {
            match parsing_result {
                Ok(book) => ApiResponse::ok_from(book.to_string().as_str()),
                Err(error) => ApiResponse::error_from(error),
            }
        };
        response.send_to_stream(stream);
    }
}
