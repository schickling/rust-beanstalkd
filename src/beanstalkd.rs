extern crate bufstream;

use std::collections::HashMap;
use std::net::TcpStream;
use self::bufstream::BufStream;

use commands;
use error::{BeanstalkdError, BeanstalkdResult};
use parse;
use request::Request;
use response::{Response, Status};

macro_rules! try {
    ($e:expr) => (match $e { Ok(e) => e, Err(_) => return Err(BeanstalkdError::ConnectionError) })
}

pub struct Beanstalkd {
    stream: BufStream<TcpStream>,
}

impl Beanstalkd {
    /// Connect to a running beanstalkd server
    ///
    /// Example: `let mut beanstalkd = Beanstalkd::connect('localhost', 11300).unwrap();`
    pub fn connect(host: &str, port: u16) -> BeanstalkdResult<Beanstalkd> {
        let tcp_stream = try!(TcpStream::connect(&(host, port)));

        Ok(Beanstalkd { stream: BufStream::new(tcp_stream) })
    }

    /// Short hand method to connect to `localhost:11300`
    pub fn localhost() -> BeanstalkdResult<Beanstalkd> {
        Beanstalkd::connect("localhost", 11300)
    }

    /// Change the tube where put new messages (Standard tube is called `default`)
    pub fn tube(&mut self, tube: &str) -> BeanstalkdResult<()> {
        self.cmd(commands::tube(tube)).map(|_| ())
    }

    /// Inserts a job into the client's currently used tube
    pub fn put(&mut self,
               body: &str,
               priority: u32,
               delay: u32,
               ttr: u32)
               -> BeanstalkdResult<u64> {
        self.cmd(commands::put(body, priority, delay, ttr)).map(parse::id)
    }

    pub fn put_u8(&mut self,
               body: &Vec<u8>,
               priority: u32,
               delay: u32,
               ttr: u32)
               -> BeanstalkdResult<u64> {
        self.cmd(commands::put_u8(body, priority, delay, ttr)).map(parse::id)
    }

    /// Get the next message out of the queue
    pub fn reserve(&mut self) -> BeanstalkdResult<(u64, String)> {
        self.cmd(commands::reserve()).map(|r| (parse::id(r.clone()), parse::body(r)))
    }

    pub fn reserve_with_timeout(&mut self, timeout:u32) -> BeanstalkdResult<(u64, String)> {
        let ret = self.cmd(commands::reserve_with_timeout(timeout)).unwrap();
        if ret.status == Status::TIMED_OUT {
            Ok((0u64, "TIMED_OUT".to_string()))
        } else {
            Ok((parse::id(ret.clone()), parse::body(ret)))
        }
    }

    pub fn bury(&mut self, id: u64, priority: u32) -> BeanstalkdResult<(u64, String)> {
        let ret = self.cmd(commands::bury(id, priority)).unwrap();
        if ret.status == Status::NOT_FOUND {
            Ok((0u64, "NOT_FOUND".to_string()))
        } else if ret.status == Status::BURIED {
            Ok((id, "BURIED".to_string()))
        } else {
            Err(BeanstalkdError::RequestError)
        }
    }

    /// Deletes a message out of the queue
    pub fn delete(&mut self, id: u64) -> BeanstalkdResult<()> {
        self.cmd(commands::delete(id)).map(|_| ())
    }

    /// Returns all available stats
    pub fn stats(&mut self) -> BeanstalkdResult<HashMap<String, String>> {
        self.cmd(commands::stats()).map(parse::hashmap)
    }

    /// Add new tube to watch list
    pub fn watch(&mut self, tube: &str) -> BeanstalkdResult<u64> {
        self.cmd(commands::watch(tube)).map(parse::id)
    }

    /// Removes the named tube from the watch list for the current connection
    pub fn ignore(&mut self, tube: &str) -> BeanstalkdResult<Option<u64>> {
        self.cmd(commands::ignore(tube)).map(parse::count)
    }

    pub fn release(&mut self, id: u64, priority: u32, delay: u32) -> BeanstalkdResult<String> {
        self.cmd(commands::release(id, priority, delay)).map(parse::body)
    }
    
    pub fn touch(&mut self, id: u64) -> BeanstalkdResult<String> {
        self.cmd(commands::touch(id)).map(parse::body)
    }

    fn cmd(&mut self, message: String) -> BeanstalkdResult<Response> {
        let mut request = Request::new(&mut self.stream);
        request.send(message.as_bytes())
    }
}
