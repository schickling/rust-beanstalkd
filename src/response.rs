#[derive(PartialEq, Debug, Clone)]
pub enum Status {
    OK,
    RESERVED,
    INSERTED,
    USING,
    DELETED,
}

#[derive(Clone)]
pub struct Response {
    pub status: Status,
    pub data: String,
}
