# cpprust
A simple application written in C++, which uses log crate via FFI

### commands to run
1. export RUST_LOG = debug
2. cargo build
3. g++ -std=c++17 -o cpp_program src/main.cpp -I .. -I target/cxxbridge/ -L target/debug/ -pthread -l dl -l cpprust
4. ./cpp_program
