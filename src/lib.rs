#[macro_use]
extern crate log;
extern crate env_logger;
use cxx::{CxxString, CxxVector};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

#[cxx::bridge]
mod ffi
{    
    struct MyClass
    {
        value: i32,
    }

    struct Person
    {
        name: String,
        age: i32,
    }

    extern "Rust"
    {
        fn log_class_from_cpp_to_rust_log_crate(my_object: &MyClass);
        fn log_vector_from_cpp_to_rust_log_crate(people: &CxxVector<Person>);
        fn log_struct_from_cpp_to_rust_log_crate(person: &Person);
        fn log_message_from_cpp_to_rust_log_crate(level: i32, message: &CxxString, attributes: &CxxVector<CxxString>);
        fn init_rust_logger() -> ();
    }

    unsafe extern "C++"
    {
        include!("cpprust/src/my_class.hpp");
        type MyClass;
        fn get_value(&self) -> i32;
    }
}

pub fn log_class_from_cpp_to_rust_log_crate(my_object: &ffi::MyClass)
{
    let value = my_object.get_value();
    warn!("Value: {}", value);
}

pub fn log_vector_from_cpp_to_rust_log_crate(people: &CxxVector<ffi::Person>)
{
    for person in people
    {
        info!("Received person: {} who is {} years old", person.name, person.age);
    }
}

pub fn log_struct_from_cpp_to_rust_log_crate(person: &ffi::Person)
{
    trace!("Received persons: {} who is {} years old", person.name, person.age);
}

pub fn log_message_from_cpp_to_rust_log_crate(level: i32, message: &CxxString, attributes: &CxxVector<CxxString>)
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
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("output.log")
        .unwrap();

    let file = Mutex::new(file);

    let logger = env_logger::Builder::new()
        .format(move |_buf, record| {
            let mut file = file.lock().unwrap();
            writeln!(file, "{}: {}", record.level(), record.args())
        })
        .filter(None, log::LevelFilter::Trace)
        .build();
    
    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
}