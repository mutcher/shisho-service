use curl::easy::Easy;

use crate::tcp_api::{ApiResponse, TcpStreamed};
use std::{
    error,
    fs::File,
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use super::common::ShishoCommand;

pub struct LoadBookCommand;

impl LoadBookCommand {
    fn load_file(url: &str, output_path: &str) -> Result<(), Box<dyn error::Error>> {
        let mut curl_client = Easy::new();
        let buffer = Arc::new(Mutex::new(Vec::<u8>::new()));
        curl_client.url(url)?;

        let local_buffer = buffer.clone();
        curl_client.write_function(move |data| {
            local_buffer.lock().unwrap().extend_from_slice(data);
            Ok(data.len())
        })?;
        curl_client.perform()?;

        let mut file = File::create(output_path)?;
        file.write_all(buffer.clone().lock().unwrap().as_slice())?;

        Ok(())
    }
}

impl ShishoCommand for LoadBookCommand {
    fn perform(args: &[String], stream: &mut TcpStream) {
        if args.len() != 1 {
            ApiResponse::error_from("Incorrect number of arguments").send_to_stream(stream);
            return;
        }

        let url = args[0].as_str();
        let output_path = "file.pdf";
        let result = LoadBookCommand::load_file(url, output_path);
        match result {
            Ok(_) => ApiResponse::ok().send_to_stream(stream),
            Err(err) => ApiResponse::error_from(&err.to_string()).send_to_stream(stream),
        }
    }
}
