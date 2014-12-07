use std::io::{IoResult};

use connection::Connection;
use response::{Response, Status};
use request::Request;

pub struct Tube {
    connection: Connection,
    name: &'static str,
}

impl Tube {
    pub fn new (connection: Connection, name: &'static str) -> Tube {
        Tube {
            connection: connection,
            name: name,
        }
    }

    pub fn put (&mut self, body: &[u8], priority: u32, delay: u32, ttr: u32) -> IoResult<u64> {
        let result = self.connection.cmd("put", vec!(priority.to_string(), delay.to_string(), ttr.to_string(), body.len().to_string()), body);
        match result {
            Ok(r) => Ok(r.id),
            Err(e) => Err(e),
        }
    }

    pub fn reserve (&mut self, seconds: u32) -> IoResult<(u64, String)> {
        let result = self.connection.cmd("reserve-with-timeout", vec!(seconds.to_string()), &[]);
        match result {
            Ok(r) => Ok((r.id, r.body)),
            Err(e) => Err(e),
        }
    }

    pub fn delete (&mut self, id: u64) -> IoResult<()> {
        let result = self.connection.cmd("delete", vec!(id.to_string()), &[]);
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

}
