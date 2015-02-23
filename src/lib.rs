//! # Easy-to-use beanstalkd client for Rust (IronMQ compatible)

#![feature(net)]
#![feature(io)]
#![feature(collections)]
#![feature(core)]

pub use beanstalkd::Beanstalkd;

mod beanstalkd;
mod commands;
mod error;
mod parse;
mod request;
mod response;
