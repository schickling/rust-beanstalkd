use std::collections::HashMap;
use std::str::FromStr;

use response::{Response, Status};

pub fn id(response: Response) -> u64 {
    let line_segments: Vec<&str> = response.data.trim().split(' ').collect();
    let id: u64 = FromStr::from_str(line_segments[1]).unwrap();
    id
}

pub fn body(response: Response) -> String {
    let body_start = response.data.find('\n').unwrap() + 1;
    response.data.slice_from(body_start).to_string()
}

pub fn hashmap(response: Response) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in response.data.split('\n').skip(2) {
        let line_segments: Vec<&str> = line.split_str(": ").collect();
        let key = line_segments[0].to_string();
        let value = line_segments[1].to_string();
        map.insert(key, value);
    }
    map
}
