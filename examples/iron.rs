extern crate beanstalkd;

use beanstalkd::Beanstalkd;

fn main() {
    let host = "mq-aws-us-east-1.iron.io";
    let token = "your token";
    let project_id = "your project id - not the name";

    let mut beanstalkd = Beanstalkd::connect(host, 11300).unwrap();
    let _ = beanstalkd.put(format!("oauth {} {}", token, project_id).as_slice(), 0, 0, 10000);
    let _ = beanstalkd.put("Hello World", 0, 0, 10000);
}
