#[macro_use]
extern crate log;
extern crate env_logger;

extern crate serde;
extern crate serde_json;

include!(concat!(env!("OUT_DIR"), "/main.rs"));
