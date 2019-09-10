extern crate beanstalkd;

use beanstalkd::Beanstalkd;

fn main() {
    let mut beanstalkd = Beanstalkd::localhost().unwrap();

    match beanstalkd.reserve_with_timeout(2) {
        Ok(Some((id, body))) => println!("id: {} body: {}", id, body),
        Ok(None) => println!("timedout"),
        Err(err) => println!("{}", err),
    }
}
