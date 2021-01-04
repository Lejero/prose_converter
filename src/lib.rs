#![allow(dead_code)]
#![allow(unused_imports)]

pub mod http_parser;
pub mod prose_interpreter;
mod simple_delegates;

pub use simple_delegates::TerminateOnCloseDelegate;
