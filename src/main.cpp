/*
  Building this program happens outside of the cargo process.
  We simply need to link against the Rust library and the
  system libraries it depends upon

  g++ -std=c++17 -o cpp_program src/main.cpp \
      -I .. -I target/cxxbridge \
      -L target/debug -l cpprust \
      -pthread -l dl
*/

// consider the ffi part of Rust code
#include "cpprust/src/lib.rs.h"
#include <vector>
#include <string>

#include <iostream>

int main()
{
    int level = 2;
    std::string message = "This is a test message";
    std::vector<std::string> attributes = {"Attribute1", "Attribute2", "Attribute3"}; 
    init_rust_logger();
    std::cout << "starting from C++\n";
    log_message_from_rust_log_crate(level, message, attributes);
    std::cout << "finishing with C++\n";
    return 0;
}