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
    TIMED_OUT,
    FOUND,
    NOT_FOUND,
    BURIED,
}

#[derive(Clone, Debug)]
pub struct Response {
    pub status: Status,
    pub data: String,
}
