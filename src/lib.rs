#[macro_use]
extern crate log;
extern crate env_logger;
use cxx::{CxxString, CxxVector};

#[cxx::bridge]
mod ffi {
    struct Person
    {
        name: String,
        age: i32,
    }

    extern "Rust"
    {
        fn take_person(person: &Person);
        fn log_message_from_rust_log_crate(level: i32, message: &CxxString, attributes: &CxxVector<CxxString>);
        fn init_rust_logger() -> ();
    }
}

pub fn take_person(person: &ffi::Person)
{
    println!("Received person: {} who is {} years old", person.name, person.age);
}

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