use connection::Connection;
use error::BeanstalkdResult;

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

    pub fn put (&mut self, body: &[u8], priority: u32, delay: u32, ttr: u32) -> BeanstalkdResult<u64> {
        let result = self.connection.cmd("put", vec!(priority.to_string(), delay.to_string(), ttr.to_string(), body.len().to_string()), body, false);
        match result {
            Ok(r) => Ok(r.id.unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn reserve (&mut self) -> BeanstalkdResult<Option<(u64, String)>> {
        let result = self.connection.cmd("reserve", vec!(), &[], true);
        match result {
            Ok(r) => {
                if r.id.is_some() && r.body.is_some() {
                    Ok(Some((r.id.unwrap(), r.body.unwrap())))
                } else {
                    Ok(None)
                }
            },
            Err(e) => Err(e),
        }
    }

    pub fn delete (&mut self, id: u64) -> BeanstalkdResult<()> {
        let result = self.connection.cmd("delete", vec!(id.to_string()), &[], false);
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

}
