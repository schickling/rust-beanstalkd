#[derive(PartialEq, Show, Clone)]
pub enum Status {
    OK,
    RESERVED,
    INSERTED,
    USING,
}

#[derive(Clone)]
pub struct Response {
    pub status: Status,
    pub data: String,
}
