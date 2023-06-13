#[macro_use]
extern crate log;
extern crate env_logger;
use cxx::{CxxString, CxxVector};

#[cxx::bridge]
mod ffi {
    extern "Rust"
    {
        fn log_message_from_rust_log_crate(level: i32, message: &CxxString, attributes: &CxxVector<CxxString>);
        fn init_rust_logger() -> ();
    }
}
// pass an argument from cpp to rust (string, int and others std::map)
pub fn log_message_from_rust_log_crate(level: i32, message: &CxxString, attributes: &CxxVector<CxxString>)
{
    match level
    {
        1 => info!("{} {:?}", message, attributes),
        2 => debug!("{} {:?}", message, attributes),

        _ => warn!("Invalid log level: {} Message: {} Attributes: {:?}", level, message, attributes),
    }
    // info!("Log from log crate library RUST");
}

pub fn init_rust_logger() -> ()
{
    env_logger::init();
}