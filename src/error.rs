use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum BeanstalkdError { ConnectionError, RequestError }

impl Error for BeanstalkdError {
    fn description(&self) -> &str {
        match *self {
            BeanstalkdError::ConnectionError => "Connection error occurred",
            BeanstalkdError::RequestError => "Request error occurred",
        }
    }
}

impl Display for BeanstalkdError {
    fn fmt(&self, formatter: &mut Formatter) -> ::std::fmt::Result {
        self.to_string().fmt(formatter)
    }
}

pub type BeanstalkdResult<T> = Result<T, BeanstalkdError>;
