use std::{io::Write, net::TcpStream};

pub trait TcpStreamed {
    fn send_to_stream(&self, stream: &mut TcpStream);
}

pub struct ApiResponse {
    is_ok: bool,
    data: Option<String>,
}

impl ApiResponse {
    pub fn ok() -> Self {
        ApiResponse {
            is_ok: true,
            data: None
        }
    }

    pub fn error() -> Self {
        ApiResponse {
            is_ok: false,
            data: None
        }
    }

    pub fn ok_from(data: &str) -> Self {
        ApiResponse {
            is_ok: true,
            data: Some(data.to_owned()),
        }
    }

    pub fn error_from(data: &str) -> Self {
        ApiResponse {
            is_ok: false,
            data: Some(data.to_owned()),
        }
    }
}

impl TcpStreamed for ApiResponse {
    fn send_to_stream(&self, stream: &mut TcpStream) {
        let mut response = {
            if self.is_ok {
                "OK\n"
            } else {
                "ERR\n"
            }
        }
        .to_owned();
        if self.data.is_some() {
            response.push_str(&self.data.clone().unwrap());
            response.push('\n');
        }
        response.push('\n');
        stream.write_all(response.as_bytes()).ok();
    }
}
