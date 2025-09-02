use std::io::{Result as IoResult, Write};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    net::TcpStream,
};

use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    // io의 Write는 바이트 데이터를 쓰는 것을 추상화한 트레이트, HTTP응답은 바이트 스트림을 쓰는 작업
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
