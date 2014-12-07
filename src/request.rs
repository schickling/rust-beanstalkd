use std::io::{BufferedStream, TcpStream, IoResult};

use response::{Response, Status};

pub struct Request<'a> {
    stream: &'a mut BufferedStream<TcpStream>,
}

impl<'a> Request<'a> {
    pub fn new<'a> (stream: &'a mut BufferedStream<TcpStream>) -> Request {
        Request { stream: stream }
    }

    pub fn send (&mut self, message: &[u8], read_body: bool) -> IoResult<Response> {
        self.stream.write(message);
        self.stream.flush();

        let line = match self.stream.read_line() {
            Ok(r) => r,
            Err(e) => { return Err(e); },
        };
        println!("{}", line);

        let trimmed_line = line.as_slice().trim_right();
        let fields: Vec<&str> = line.split(' ').collect();

        if fields.len() < 2 {
            return Err()
        }

        if read_body {

        
        }


        //if fields.len() > 0 {

            //let status = match fields[0] {
                //"OK" => Status::OK,
                //"RESERVED" => Status::RESERVED,
                //"INSERTED" => Status::INSERTED,
                //"NOT_IMPLEMENTED" => Status::NOT_IMPLEMENTED,
            //};



            ////match fields[0] {
                ////"OK" | "FOUND" | "RESERVED" => {
                    ////let bytes = from_str::<uint>(fields[fields.len()-1]).unwrap();
                    ////let payload = self.stream.read_exact(bytes+2).unwrap();
                    ////println!("{}", String::from_utf8(payload).unwrap().as_slice().trim_right());
                ////},
                ////_ => {}
            ////}
        //}


        let response = Response::new(Status::OK, 1, trimmed_line.to_string());

        Ok(response)
    }
}
