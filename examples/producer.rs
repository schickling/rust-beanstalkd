extern crate beanstalkd;

use beanstalkd::Beanstalkd;

fn main() {
    let mut beanstalkd = Beanstalkd::localhost().unwrap();
    let _ = beanstalkd.put("Hello World", 0, 0, 10000);
}
