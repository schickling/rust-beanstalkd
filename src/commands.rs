pub fn tube(tube: &str) -> String {
    build("use", vec![tube.to_string()], "")
}

pub fn put(body: &str, priority: u32, delay: u32, ttr: u32) -> String {
    build("put",
          vec![priority.to_string(), delay.to_string(), ttr.to_string()],
          body)
}

pub fn put_u8(body: &Vec<u8>, priority: u32, delay: u32, ttr: u32) -> String {
    let args = vec![priority.to_string(), delay.to_string(), ttr.to_string()];
    let line_break = "\r\n";
    let space = " ";
    let mut message = String::from("put");

    if args.len() > 0 {
        message = message + space + &(args.join(space));
    }

    if body.len() > 0 {
        message = message + space + &(body.len().to_string()) + line_break;
    }
    let u8body = unsafe {
        String::from_utf8_unchecked(body.clone())
    };
    message.push_str(u8body.as_str());

    message = message + line_break;

    message  
}

pub fn reserve() -> String {
    build("reserve", vec![], "")
}

pub fn reserve_with_timeout(timeout: u32) -> String {
    build("reserve-with-timeout", vec![timeout.to_string()], "")
}

pub fn bury(id: u64, priority: u32) -> String {
    build("bury", vec![id.to_string(), priority.to_string()], "")
}

pub fn delete(id: u64) -> String {
    build("delete", vec![id.to_string()], "")
}

pub fn stats() -> String {
    build("stats", vec![], "")
}

pub fn watch(tube: &str) -> String {
    build("watch", vec![tube.to_string()], "")
}

pub fn ignore(tube: &str) -> String {
    build("ignore", vec![tube.to_string()], "")
}

pub fn release(id: u64, priority: u32, delay: u32) -> String {
    build("release", vec![id.to_string(), priority.to_string(), delay.to_string()], "")
}

pub fn touch(id: u64) -> String {
    build("touch", vec![id.to_string()], "")
}

fn build(op: &str, args: Vec<String>, body: &str) -> String {
    let line_break = "\r\n";
    let space = " ";
    let mut message = String::new() + op;

    if args.len() > 0 {
        message = message + space + &(args.join(space));
    }

    if body.len() > 0 {
        message = message + space + &(body.len().to_string()) + line_break + body;
    }

    message = message + line_break;

    message
}

#[test]
fn tube_test() {
    assert_eq!(tube("custom_tube"), "use custom_tube\r\n".to_string());
}

#[test]
fn put_test() {
    assert_eq!(put("some message", 0, 2, 10000),
               "put 0 2 10000 12\r\nsome message\r\n".to_string());
}

#[test]
fn reserve_test() {
    assert_eq!(reserve(), "reserve\r\n".to_string());
}

#[test]
fn delete_test() {
    assert_eq!(delete(1), "delete 1\r\n".to_string());
}

#[test]
fn stats_test() {
    assert_eq!(stats(), "stats\r\n".to_string());
}

#[test]
fn watch_test() {
    assert_eq!(watch("hello_tube"), "watch hello_tube\r\n".to_string());
}

#[test]
fn ignore_test() {
    assert_eq!(ignore("hello_tube"), "ignore hello_tube\r\n".to_string());
}
