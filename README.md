# cpprust
A simple application written in C++, which uses log crate via FFI

### commands to run
1. export RUST_LOG=debug
2. cargo build --release
3. g++ -std=c++11 -o cpp_program src/main.cpp -I .. -I target/cxxbridge -I benchmark/include -L target/debug -L benchmark/build/src -l cpprust -l benchmark -l benchmark_main -pthread -l dl
4. ./cpp_program
