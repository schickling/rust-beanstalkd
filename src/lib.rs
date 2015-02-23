//! # Easy-to-use beanstalkd client for Rust (IronMQ compatible)

#![allow(unstable)]
#![feature(net)]
#![feature(io)]

pub use beanstalkd::Beanstalkd;

mod beanstalkd;
mod commands;
mod error;
mod parse;
mod request;
mod response;
