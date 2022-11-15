use crate::tcp_api::{ApiResponse, TcpStreamed};
use std::{
    net::TcpStream,
    process::{Command, Stdio},
};

use super::common::ShishoCommand;

pub struct LoadBookCommand;

impl ShishoCommand for LoadBookCommand {
    fn perform(args: &[String], stream: &mut TcpStream) {
        if args.len() != 1 {
            ApiResponse::error_from("Incorrect number of arguments").send_to_stream(stream);
            return;
        }

        let url = args[0].to_string();

        let status = Command::new("curl")
            .args(["--output", "file.pdf", "--url", url.as_ref()])
            .stdout(Stdio::null())
            .status();

        let response = {
            if status.is_ok() && status.as_ref().unwrap().success() {
                ApiResponse::ok()
            } else {
                ApiResponse::error_from(
                    status
                        .as_ref()
                        .ok()
                        .unwrap()
                        .code()
                        .unwrap()
                        .to_string()
                        .as_str(),
                )
            }
        };
        response.send_to_stream(stream);
    }
}
