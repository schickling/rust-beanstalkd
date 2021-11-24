use std::collections::HashMap;
use std::str::FromStr;

use response::Response;

pub fn id(response: Response) -> u64 {
    let line_segments: Vec<&str> = response.data.trim().split(' ').collect();

    if line_segments.len() != 1 {
        return FromStr::from_str(line_segments[1]).unwrap();
    }

    0
}

pub fn body(response: Response) -> String {
    let start_pos = response.data.trim().find('\n');
    if !(start_pos.is_none()) {
        let body_start = start_pos.unwrap() + 1;

        response.data.trim()[body_start..].to_string()
    } else {
        "".to_string()
    }
}

pub fn hashmap(response: Response) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in response.data.trim().split('\n').skip(2) {
        let line_segments: Vec<&str> = line.trim().split(": ").collect();
        let key = line_segments[0].to_string();
        let value = line_segments[1].to_string();
        map.insert(key, value);
    }
    map
}

pub fn count(response: Response) -> Option<u64> {
    let line_segments: Vec<&str> = response.data.trim().split(' ').collect();
    if line_segments.len() == 1 {
        None
    } else {
        let id: u64 = FromStr::from_str(line_segments[1]).unwrap();
        Some(id)
    }
}

#[test]
fn id_test() {
    let response = Response {
        status: ::response::Status::INSERTED,
        data: "INSERTED 3\r\n".to_string(),
    };
    assert_eq!(id(response), 3);
}

#[test]
fn body_test() {
    let response = Response {
        status: ::response::Status::RESERVED,
        data: "RESERVED 3 4\r\ntest\r\nbody\r\n".to_string(),
    };
    assert_eq!(body(response), "test\r\nbody".to_string());
}

#[test]
fn hashmap_test() {
    let response = Response {
        status: ::response::Status::INSERTED,
        data: "OK 15\r\n---\r\na: b\r\nc: d\r\n".to_string(),
    };
    let mut expected_hashmap = HashMap::new();
    expected_hashmap.insert("a".to_string(), "b".to_string());
    expected_hashmap.insert("c".to_string(), "d".to_string());
    assert_eq!(hashmap(response), expected_hashmap);
}

#[test]
fn count_test() {
    let response_ok = Response {
        status: ::response::Status::WATCHING,
        data: "WATCHING 2\r\n".to_string(),
    };
    assert_eq!(count(response_ok), Some(2));

    let response_fail = Response {
        status: ::response::Status::NOT_IGNORED,
        data: "NOT_IGNORED\r\n".to_string(),
    };
    assert_eq!(count(response_fail), None);
}
