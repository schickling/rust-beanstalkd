//! # Easy-to-use beanstalkd client for Rust (IronMQ compatible)
//!
//!
//! ## Usage
//!
//! #### Producer
//!
//! ```rs
//! extern crate beanstalkd;
//!
//! use beanstalkd::Beanstalkd;
//!
//! fn main() {
//!     let mut beanstalkd = Beanstalkd::localhost().unwrap();
//!     beanstalkd.put("Hello World", 0, 0, 10000);
//! }
//! ```
//!
//! #### Consumer
//!
//! ```rs
//! extern crate beanstalkd;
//!
//! use beanstalkd::Beanstalkd;
//!
//! fn main() {
//!     let mut beanstalkd = Beanstalkd::localhost().unwrap();
//!     loop {
//!         let (id, body) = beanstalkd.reserve().unwrap();
//!         println!("{}", body);
//!         beanstalkd.delete(id);
//!     }
//! }
//! ```
//!

#![allow(unstable)]

pub use beanstalkd::Beanstalkd;

mod beanstalkd;
mod error;
mod request;
mod response;
mod commands;
