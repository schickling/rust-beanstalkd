pub fn tube(tube: &str) -> String {
    build("use", vec![tube.to_string()], "")
}

pub fn put(body: &str, priority: u32, delay: u32, ttr: u32) -> String {
    build("put",
          vec![priority.to_string(), delay.to_string(), ttr.to_string()],
          body)
}

pub fn peek(peek_type_or_id: &str) -> String {
    let mut cmd = "peek";

    if peek_type_or_id == "ready" {
        cmd = "peek-ready";
    } else if peek_type_or_id == "delayed" {
        cmd = "peek-delayed";
    } else if peek_type_or_id == "buried" {
        cmd = "peek-buried"
    } else {
        return build(cmd, vec![peek_type_or_id.to_string()], "")
    }

    build(cmd, vec![], "")
}

pub fn reserve() -> String {
    build("reserve", vec![], "")
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
fn peek_test() {
    assert_eq!(peek("ready"), "peek-ready\r\n".to_string());
    assert_eq!(peek("delayed"), "peek-delayed\r\n".to_string());
    assert_eq!(peek("buried"), "peek-buried\r\n".to_string());
    assert_eq!(peek("1"), "peek 1\r\n".to_string());
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
