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
