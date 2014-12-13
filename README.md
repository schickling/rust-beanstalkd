rust-beanstalkd [![Build Status](https://travis-ci.org/schickling/rust-beanstalkd.svg)](https://travis-ci.org/schickling/rust-beanstalkd)
===============

Easy-to-use beanstalkd client for Rust (IronMQ compatible)

## Install

Add this dependency to your `Cargo.toml`

```
beanstalkd = "~0.0.0"
```

## Documentation

More documentation can be found [here](http://schickling.me/rust-beanstalkd).

## Usage

#### Producer

```rs
extern crate beanstalkd;

use beanstalkd::{Connection, Tube};

let connection = Connection::new("localhost", 11300).unwrap();
let mut tube = Tube::new(connection, "default");
tube.put(b"Hello World \n This is a new line!", 0, 0, 10000);
```

#### Consumer

```rs
extern crate beanstalkd;

use beanstalkd::{Connection, Tube};

let connection = Connection::new("localhost", 11300).unwrap();
let mut tube = Tube::new(connection, "default");
match tube.reserve().unwrap() {
    Some((id, body)) => tube.delete(id),
    None => {},
};
```

#### IronMQ example

```rs
// TODO
```

## License

[MIT License](http://opensource.org/licenses/MIT)
