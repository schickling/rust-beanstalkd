use std::io::{BufferedStream, TcpStream};
use std::str::FromStr;
use std::str;

use error::{BeanstalkdError, BeanstalkdResult};
use response::{Response, Status};

macro_rules! try {
    ($e:expr) => (match $e { Ok(e) => e, Err(_) => return Err(BeanstalkdError::RequestError) })
}

macro_rules! try_option {
    ($e:expr) => (match $e { Some(e) => e, None => return Err(BeanstalkdError::RequestError) })
}

pub struct Request<'a> {
    stream: &'a mut BufferedStream<TcpStream>,
}

impl<'a> Request<'a> {
    pub fn new<'b> (stream: &'b mut BufferedStream<TcpStream>) -> Request {
        Request { stream: stream }
    }

    pub fn send (&mut self, message: &[u8]) -> BeanstalkdResult<Response> {
        let _ = self.stream.write(message);
        let _ = self.stream.flush();

        let line = try!(self.stream.read_line());
        let line_segments: Vec<&str> = line.trim().split(' ').collect();
        let status_str = try_option!(line_segments.first());
        let status = match *status_str {
            "OK" => Status::OK,
            "RESERVED" => Status::RESERVED,
            "INSERTED" => Status::INSERTED,
            "USING" => Status::USING,
            _ => { return Err(BeanstalkdError::RequestError) },
        };
        let mut data = line.clone();

        if status == Status::OK || status == Status::RESERVED {
            let segment_offset = match status {
                Status::OK => 1,
                Status::RESERVED => 2,
                _ => { return Err(BeanstalkdError::RequestError) },
            };
            let bytes_count_str = try_option!(line_segments.get(segment_offset));
            let bytes_count: usize = try_option!(FromStr::from_str(*bytes_count_str));
            let payload_utf8 = try!(self.stream.read_exact(bytes_count + 2)); // +2 needed for trailing line break
            let payload_str = try!(str::from_utf8(payload_utf8.as_slice()));
            data = data + payload_str;
        }

        Ok(Response { status: status, data: data })
    }
}
