#include <string>
#include <memory>
#include <functional>
#include <rust/cxx.h>
#include <chimer/src/main.rs.h>
// struct SomeStruct{
//     rust::string foo;
//     int8_t bar;
// };

int8_t do_struct(SomeStruct x) {
    return x.bar;
}

std::unique_ptr<std::string> hello_gang() {
    // std::string str = "Hello gang wsg";
    std::unique_ptr<std::string> str = std::make_unique<std::string>("hello wsg gang");
    return str;
}
