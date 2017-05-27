extern crate beanstalkd;

use beanstalkd::Beanstalkd;

fn main() {
    let mut beanstalkd = Beanstalkd::localhost().unwrap();
    let count = beanstalkd.watch("hello_tube");
    println!("{:?}", count);
}