use std::io::{BufferedStream, TcpStream};

use error::{BeanstalkdError, BeanstalkdResult};
use commands;
use request::Request;
use response::Response;

pub struct Beanstalkd {
    stream: BufferedStream<TcpStream>,
}

impl Beanstalkd {
    pub fn localhost() -> BeanstalkdResult<Beanstalkd> {
        Beanstalkd::connect("localhost", 11300)
    }

    pub fn connect(host: &str, port: u16) -> BeanstalkdResult<Beanstalkd> {
        let tcp_stream = match TcpStream::connect((host, port)) {
            Ok(s) => s,
            Err(_) => { return Err(BeanstalkdError::ConnectionError) },
        };
        let instance = Beanstalkd { stream: BufferedStream::new(tcp_stream) };
        Ok(instance)
    }

    pub fn tube(&mut self, tube: &str) -> BeanstalkdResult<()> {
        match self.cmd(commands::tube(tube), false) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn put (&mut self, body: &str, priority: u32, delay: u32, ttr: u32) -> BeanstalkdResult<u64> {
        match self.cmd(commands::put(body, priority, delay, ttr), false) {
            Ok(r) => Ok(r.id.unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn reserve (&mut self) -> BeanstalkdResult<(u64, String)> {
        match self.cmd(commands::reserve(), true) {
            Ok(r) => Ok((r.id.unwrap(), r.body.unwrap())),
            Err(e) => Err(e),
        }
    }

    pub fn delete(&mut self, id: u64) -> BeanstalkdResult<()> {
        match self.cmd(commands::delete(id), false) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn cmd(&mut self, message: String, read_body: bool) -> BeanstalkdResult<Response> {
        let mut request = Request::new(&mut self.stream);

        request.send(message.as_bytes(), read_body)
    }
}
