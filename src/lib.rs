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
    struct Animal
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
        fn log_string_from_cpp_to_rust_log_crate(message: &CxxString);
        fn log_int_from_cpp_to_rust_log_crate(level: i32);
        fn log_vector_from_cpp_to_rust_log_crate(attributes: &CxxVector<CxxString>);
        fn log_struct_from_cpp_to_rust_log_crate(person: &Person);
        fn log_class_from_cpp_to_rust_log_crate(animal: &Animal);
        fn init_rust_logger() -> ();
    }

    unsafe extern "C++"
    {
        include!("cpprust/src/animal.hpp");
        type Animal;
        fn get_age(&self) -> i32;
    }
}

pub fn log_string_from_cpp_to_rust_log_crate(message: &CxxString)
{
    info!("{}", message);
}

pub fn log_int_from_cpp_to_rust_log_crate(level: i32)
{
    debug!("{}", level);
}

pub fn log_vector_from_cpp_to_rust_log_crate(attributes: &CxxVector<CxxString>)
{
    warn!("{:?}", attributes);
}

pub fn log_struct_from_cpp_to_rust_log_crate(person: &ffi::Person)
{
    trace!("Received persons: {} who is {} years old", person.name, person.age);
}

pub fn log_class_from_cpp_to_rust_log_crate(animal: &ffi::Animal)
{
    let value = animal.get_age();
    error!("{}", value);
}

pub fn init_rust_logger() -> ()
{
    let file = OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open("/dev/null")
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