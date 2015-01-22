#[derive(PartialEq, Show)]
pub enum Status {
    RESERVED,
    INSERTED,
    USING,
}

pub struct Response {
    pub status: Status,
    pub id: Option<u64>,
    pub body: Option<String>,
}

impl Response {
    pub fn new (status: Status, id: Option<u64>, body: Option<String>) -> Response {
        Response {
            status: status,
            id: id,
            body: body,
        }
    }
}
