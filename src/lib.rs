#[macro_use]
extern crate log;
extern crate env_logger;

#[cxx::bridge]
mod ffi {
    extern "Rust"
    {
        fn log_message_from_rust_log_crate() -> ();
        fn init_rust_logger() -> ();
    }
}
// pass an argument from cpp to rust (string, int and others std::map)
pub fn log_message_from_rust_log_crate() -> ()
{
    info!("Log from log crate library RUST");
}

pub fn init_rust_logger() -> ()
{
    env_logger::init();
}