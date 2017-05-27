#[derive(PartialEq, Debug, Clone)]
pub enum Status {
    OK,
    RESERVED,
    INSERTED,
    USING,
    DELETED,
    WATCHING,
    NOT_IGNORED,
}

#[derive(Clone)]
pub struct Response {
    pub status: Status,
    pub data: String,
}
