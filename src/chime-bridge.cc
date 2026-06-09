#include <string>
#include <memory>

std::unique_ptr<std::string> hello_gang() {
    // std::string str = "Hello gang wsg";
    std::unique_ptr<std::string> str = std::make_unique<std::string>("hello wsg gang");
    return str;
}