mod commands;
mod tcp_api;

use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

use commands::{
    AddCommand, DeleteBookCommand, GetAllBooksCommand, GetBookCommand, LoadBookCommand,
    ShishoCommand, UknownCommand,
};

fn process_command(command: &str, args: &[String], stream: &mut TcpStream) -> bool {
    match command {
        "get" => GetBookCommand::perform(args, stream),
        "getall" => GetAllBooksCommand::perform(args, stream),
        "add" => AddCommand::perform(args, stream),
        "delete" => DeleteBookCommand::perform(args, stream),
        "load" => LoadBookCommand::perform(args, stream),
        "exit" => {
            return true;
        }
        _ => {
            let mut command_with_args = vec![command.to_string()];
            command_with_args.extend_from_slice(args);
            UknownCommand::perform(command_with_args.as_slice(), stream);
        }
    }

    return false;
}

fn handle_connection(mut stream: TcpStream) {
    loop {
        let buf_reader = BufReader::new(&mut stream);
        let request_data: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap_or("".to_owned()))
            .take_while(|line| !line.is_empty())
            .collect();

        if request_data.len() < 1 {
            continue;
        };

        let result = process_command(
            request_data.get(0).unwrap_or(&"".to_owned()),
            &request_data[1..request_data.len()],
            &mut stream,
        );
        if result {
            return;
        }
    }
}

fn main() {
    println!(
        "Initializing service: version={}",
        env!("CARGO_PKG_VERSION")
    );
    let address = "127.0.0.1:6673";
    let listener = TcpListener::bind(address).expect("Cannot bind address");
    println!("Binding to {}", address);

    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => handle_connection(stream),
            Err(error) => println!("Error handling incomming connection: {}", error),
        }
    }
}
