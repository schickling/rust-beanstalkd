rust-beanstalkd [![Build Status](https://travis-ci.org/schickling/rust-beanstalkd.svg)](https://travis-ci.org/schickling/rust-beanstalkd)
===============

Easy-to-use beanstalkd client for Rust (IronMQ compatible)

## Install

Add this dependency to your `Cargo.toml`

```toml
beanstalkd = "*"
```

## Documentation

More documentation can be found [here](http://schickling.me/rust-beanstalkd).

## Usage

#### Producer

```rs
extern crate beanstalkd;

use beanstalkd::Beanstalkd;

fn main() {
    let mut beanstalkd = Beanstalkd::localhost().unwrap();
    beanstalkd.put("Hello World", 0, 0, 10000);
}
```

#### Consumer

```rs
extern crate beanstalkd;

use beanstalkd::Beanstalkd;

fn main() {
    let mut beanstalkd = Beanstalkd::localhost().unwrap();
    loop {
        let (id, body) = beanstalkd.reserve().unwrap();
        println!("{}", body);
        beanstalkd.delete(id);
    }
}
```

#### IronMQ example

```rs
// TODO
```

## License

[MIT License](http://opensource.org/licenses/MIT)
