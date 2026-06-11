#pragma once
#include <string>
#include <memory>
#include <rust/cxx.h>
#include <chimer/src/main.rs.h>
std::unique_ptr<std::string> hello_gang();

// struct SomeStruct{
//     rust::String foo;
//     int8_t bar;
// };
struct SomeStruct;

int8_t do_struct(SomeStruct x);

// const auto&
std::unique_ptr<std::string> hello_gang();