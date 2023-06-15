/*
  Building this program happens outside of the cargo process.
  We simply need to link against the Rust library and the
  system libraries it depends upon

  g++ -std=c++17 -o cpp_program src/main.cpp \
      -I .. -I target/cxxbridge \
      -L target/debug -l cpprust \
      -pthread -l dl
*/

#include "cpprust/src/lib.rs.h"
#include <benchmark/benchmark.h>
#include <vector>
#include <string>
#include <iostream>

static bool logger_initialized = false;

void init_logger_once() {
    if (!logger_initialized) {
        init_rust_logger();
        logger_initialized = true;
    }
}

static void BM_log_vector_from_rust_log_crate(benchmark::State& state)
{
    std::vector<Person> people;

    Person p1;
    p1.name = "John";
    p1.age = 30;
    people.push_back(p1);

    Person p2;
    p2.name = "Bob";
    p2.age = 40;
    people.push_back(p2);

    Person p3;
    p3.name = "Aaron";
    p3.age = 25;
    people.push_back(p3);

    init_logger_once();

    for (auto _ : state) {
        log_vector_from_rust_log_crate(people);
    }
}

BENCHMARK(BM_log_vector_from_rust_log_crate);

int main(int argc, char** argv)
{
    ::benchmark::Initialize(&argc, argv);
    ::benchmark::RunSpecifiedBenchmarks();
    // MyClass my_object(42);
    // print_value_from_rust(my_object);

    // std::vector<Person> people;

    // Person p1;
    // p1.name = "John";
    // p1.age = 30;
    // people.push_back(p1);

    // Person p2;
    // p2.name = "Bob";
    // p2.age = 40;
    // people.push_back(p2);

    // Person p3;
    // p3.name = "Aaron";
    // p3.age = 25;
    // people.push_back(p3);    

    
    // int level = 5;
    // std::string message = "This is a test message";
    // std::vector<std::string> attributes = {"Attribute1", "Attribute2", "Attribute3"};

    // init_rust_logger();
    // log_vector_from_rust_log_crate(people);
    // log_struct_from_rust_log_crate(p1);
    // log_message_from_rust_log_crate(level, message, attributes);
    return 0;
}