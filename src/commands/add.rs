use crate::tcp_api::{ApiResponse, TcpStreamed};

use super::{common::Book, ShishoCommand};

pub struct AddCommand;

impl ShishoCommand for AddCommand {
    fn perform(args: &[String], stream: &mut std::net::TcpStream) {
        let input = args.join("");
        let parsing_result = Book::from_bibtex(input.as_str());
        let response = {
            match parsing_result {
                Ok(book) => ApiResponse::ok_from(book.to_string().as_str()),
                Err(error) => ApiResponse::error_from(error.as_str()),
            }
        };
        response.send_to_stream(stream);
    }
}
