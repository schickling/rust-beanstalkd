use std::io::{BufferedStream, TcpStream};
use std::str::FromStr;

use error::{BeanstalkdError, BeanstalkdResult};
use response::{Response, Status};

pub struct Request<'a> {
    stream: &'a mut BufferedStream<TcpStream>,
}

impl<'a> Request<'a> {
    pub fn new<'b> (stream: &'b mut BufferedStream<TcpStream>) -> Request {
        Request { stream: stream }
    }

    pub fn send (&mut self, message: &[u8], read_body: bool) -> BeanstalkdResult<Response> {
        self.stream.write(message);
        self.stream.flush();

        let line = match self.stream.read_line() {
            Ok(r) => r,
            Err(_) => { return Err(BeanstalkdError::RequestError); },
        };

        let trimmed_line = line.as_slice().trim_right();
        let fields: Vec<&str> = trimmed_line.split(' ').collect();

        if fields.len() < 1 {
            return Err(BeanstalkdError::RequestError);
        }

        let status = match fields[0] {
            "RESERVED" => Status::RESERVED,
            "INSERTED" => Status::INSERTED,
            "USING" => Status::USING,
            _ => { return Err(BeanstalkdError::RequestError) },
        };

        let mut id = None;
        let mut body = None;

        if status != Status::USING {
            if fields.len() < 2 {
                return Err(BeanstalkdError::RequestError);
            }

            id = FromStr::from_str(fields[1]);

            if read_body {
                if fields.len() < 3 {
                    return Err(BeanstalkdError::RequestError);
                }

                let num_bytes: usize = FromStr::from_str(fields[fields.len() - 1]).unwrap();
                let utf8_payload = self.stream.read_exact(num_bytes + 2).unwrap();
                let payload = String::from_utf8(utf8_payload).unwrap().as_slice().trim_right().to_string();
                body = Some(payload);
            }
        }

        Ok(Response::new(status, id, body))
    }
}
