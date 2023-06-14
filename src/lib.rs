#[macro_use]
extern crate log;
extern crate env_logger;
use cxx::{CxxString, CxxVector};

// use a class and use some method 
#[cxx::bridge]
mod ffi {
    
    struct Person
    {
        name: String,
        age: i32,
    }

    extern "Rust"
    {
        fn log_vector_from_rust_log_crate(people: &CxxVector<Person>);
        fn log_struct_from_rust_log_crate(person: &Person);
        fn log_message_from_rust_log_crate(level: i32, message: &CxxString, attributes: &CxxVector<CxxString>);
        fn init_rust_logger() -> ();
    }

    extern unsafe "C++"
    {
        
    }
}

pub fn log_vector_from_rust_log_crate(people: &CxxVector<ffi::Person>)
{
    for person in people{
        info!("Received person: {} who is {} years old", person.name, person.age);
    }
}

pub fn log_struct_from_rust_log_crate(person: &ffi::Person)
{
    trace!("Received persons: {} who is {} years old", person.name, person.age);
}

pub fn log_message_from_rust_log_crate(level: i32, message: &CxxString, attributes: &CxxVector<CxxString>)
{
    match level
    {
        1 => info!("{} {:?}", message, attributes),
        2 => debug!("{} {:?}", message, attributes),

        _ => warn!("Invalid log level: {} Message: {} Attributes: {:?}", level, message, attributes),
    }
}

pub fn init_rust_logger() -> ()
{
    env_logger::init();
}