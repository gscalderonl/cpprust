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

void init_logger_once()
{
    if (!logger_initialized)
    {
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

    for (auto _ : state)
    {
        log_vector_from_rust_log_crate(people);
    }
}

static void BM_log_message_from_rust_log_crate(benchmark::State& state)
{
    int level = 1;
    std::string message = "This is a test message";
    std::vector<std::string> attributes = {"A1", "A2", "A3"};

    init_logger_once();

    for (auto _ : state)
    {
        log_message_from_rust_log_crate(level, message, attributes);
    }

}

static void BM_log_struct_from_rust_log_crate(benchmark::State& state)
{
    MyClass my_object(42);

    for(auto _ : state)
    {
        print_value_from_rust(my_object);
    }
}

static void BM_print_value_from_rust(benchmark::State& state)
{
    Person p1;
    p1.name = "John";
    p1.age = 30;

    init_logger_once();

    for(auto _ : state)
    {
        log_struct_from_rust_log_crate(p1);
    }
}

BENCHMARK(BM_log_vector_from_rust_log_crate);
BENCHMARK(BM_log_message_from_rust_log_crate);
BENCHMARK(BM_log_struct_from_rust_log_crate);
BENCHMARK(BM_print_value_from_rust);

int main(int argc, char** argv)
{
    ::benchmark::Initialize(&argc, argv);
    ::benchmark::RunSpecifiedBenchmarks();
    
    return 0;
}