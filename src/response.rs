#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug, Clone)]
pub enum Status {
    OK,
    RESERVED,
    INSERTED,
    USING,
    DELETED,
    WATCHING,
    NOT_IGNORED,
    FOUND,
    NOT_FOUND
}

#[derive(Clone)]
pub struct Response {
    pub status: Status,
    pub data: String,
}
