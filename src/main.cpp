#include "cpprust/src/lib.rs.h"
#include <benchmark/benchmark.h>
#include <vector>
#include <string>
#include <iostream>

static void BM_log_vector_from_cpp_to_rust_log_crate(benchmark::State& state)
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

    for (auto _ : state)
    {
        log_vector_from_cpp_to_rust_log_crate(people);
    }
}

static void BM_log_message_from_cpp_to_rust_log_crate(benchmark::State& state)
{
    int level = 1;
    std::string message = "This is a test message";
    std::vector<std::string> attributes = {"A1", "A2", "A3"};

    for (auto _ : state)
    {
        log_message_from_cpp_to_rust_log_crate(level, message, attributes);
    }

}

static void BM_log_struct_from_cpp_to_rust_log_crate(benchmark::State& state)
{
    Person p1;
    p1.name = "John";
    p1.age = 30;

    for(auto _ : state)
    {
        log_struct_from_cpp_to_rust_log_crate(p1);
    }
}

static void BM_log_class_from_cpp_to_rust_log_crate(benchmark::State& state)
{
    MyClass my_object(42);

    for(auto _ : state)
    {
        log_class_from_cpp_to_rust_log_crate(my_object);
    }
}

BENCHMARK(BM_log_vector_from_cpp_to_rust_log_crate);
BENCHMARK(BM_log_message_from_cpp_to_rust_log_crate);
BENCHMARK(BM_log_struct_from_cpp_to_rust_log_crate);
BENCHMARK(BM_log_class_from_cpp_to_rust_log_crate);

int main(int argc, char** argv)
{
    init_rust_logger();
    ::benchmark::Initialize(&argc, argv);
    ::benchmark::RunSpecifiedBenchmarks();
    
    return 0;
}