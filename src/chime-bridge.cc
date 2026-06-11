#include <string>
#include <memory>
#include <functional>
#include <rust/cxx.h>
#include <chimer/src/main.rs.h>
#include "signaling/default_signaling_client_factory.h"
// struct SomeStruct{
//     rust::string foo;
//     int8_t bar;
// };

namespace chime_bridge {

    int8_t do_struct(SomeStruct x) {
        return x.bar;
    }

    std::unique_ptr<std::string> hello_gang() {
        // std::string str = "Hello gang wsg";
        std::unique_ptr<std::string> str = std::make_unique<std::string>("hello wsg gang");
        return str;
    }

    std::unique_ptr<chime::MeetingSessionCredentials> make_session_credentials(rust::String attendee_id, rust::String external_user_id, rust::String join_token) {

        std::cout << std::string(attendee_id) << " =attendee_id" << std::endl; 

        return std::make_unique<chime::MeetingSessionCredentials>(
            chime::MeetingSessionCredentials { 
                std::string(attendee_id),
                std::string(external_user_id),
                std::string(join_token)
            }
        );
    }
}

