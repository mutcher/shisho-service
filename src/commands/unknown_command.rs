use crate::tcp_api::{ApiResponse, TcpStreamed};

use super::ShishoCommand;

pub struct UknownCommand;

impl ShishoCommand for UknownCommand {
    fn perform(args: &[String], stream: &mut std::net::TcpStream) {
        let command = args[0].to_string();
        println!("Received unknown command: {}", command);
        ApiResponse::error_from("Unknown command").send_to_stream(stream);
    }
}