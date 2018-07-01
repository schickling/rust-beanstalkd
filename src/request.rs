extern crate bufstream;

use std::net::TcpStream;
use self::bufstream::BufStream;
use std::io::{Write, BufRead, Read};
use std::str::FromStr;
use std::str::from_utf8;

use error::{BeanstalkdError, BeanstalkdResult};
use response::{Response, Status};

macro_rules! try {
    ($e:expr) => (match $e { Ok(e) => e, Err(_) => return Err(BeanstalkdError::RequestError) })
}

macro_rules! try_option {
    ($e:expr) => (match $e { Some(e) => e, None => return Err(BeanstalkdError::RequestError) })
}

pub struct Request<'a> {
    stream: &'a mut BufStream<TcpStream>,
}

impl<'a> Request<'a> {
    pub fn new<'b>(stream: &'b mut BufStream<TcpStream>) -> Request {
        Request { stream: stream }
    }

    pub fn send(&mut self, message: &[u8]) -> BeanstalkdResult<Response> {
        let _ = self.stream.write(message);
        let _ = self.stream.flush();

        let mut line = String::new();
        try!(self.stream.read_line(&mut line));
        let line_segments: Vec<&str> = line.trim().split(' ').collect();
        let status_str = try_option!(line_segments.first());
        
        let status = match *status_str {
            "OK" => Status::OK,
            "RESERVED" => Status::RESERVED,
            "INSERTED" => Status::INSERTED,
            "USING" => Status::USING,
            "DELETED" => Status::DELETED,
            "WATCHING" => Status::WATCHING,
            "NOT_IGNORED" => Status::NOT_IGNORED,
            "TIMED_OUT" => Status::TIMED_OUT,
            "FOUND" => Status::FOUND,
            "NOT_FOUND" => Status::NOT_FOUND,
            "BURIED" => Status::BURIED,
            _ => return Err(BeanstalkdError::RequestError),
        };
        let mut data = line.clone();

        if status == Status::OK || status == Status::RESERVED || status == Status::FOUND {
            let segment_offset = match status {
                Status::OK => 1,
                Status::RESERVED => 2,
                Status::FOUND => 2,
                _ => return Err(BeanstalkdError::RequestError),
            };
            let bytes_count_str = try_option!(line_segments.get(segment_offset));
            let bytes_count: usize = try!(FromStr::from_str(*bytes_count_str));
            let mut tmp_vec: Vec<u8> = vec![0; bytes_count + 2]; // +2 needed for trailing line break
            let payload_utf8 = &mut tmp_vec[..];
            try!(self.stream.read(payload_utf8));
            let payload_str = try!(from_utf8(&payload_utf8));
            data = data + &payload_str;
        }
        println!("data {}", data);
        Ok(Response {
            status: status,
            data: data,
        })
    }
}
