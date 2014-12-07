pub enum Status {
    OK,
    RESERVED,
    INSERTED,
    NOT_IMPLEMENTED,
}

pub struct Response {
    pub status: Status,
    pub id: u64,
    pub body: String,
}

impl Response {
    pub fn new (status: Status, id: u64, body: String) -> Response {
        Response {
            status: status,
            id: id,
            body: body,
        }
    }
}
