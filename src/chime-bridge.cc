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

int8_t do_struct(SomeStruct x) {
    return x.bar;
}

std::unique_ptr<std::string> hello_gang() {
    // std::string str = "Hello gang wsg";
    std::unique_ptr<std::string> str = std::make_unique<std::string>("hello wsg gang");
    return str;
}

using MeetingSessionCredentials = chime::MeetingSessionCredentials;
std::unique_ptr<MeetingSessionCredentials> make_session_credentials(rust::String attendee_id, rust::String external_user_id, rust::String join_token) {
    
    std::printf("Creating meeting session credentials w/%d, %d, %d\n", attendee_id, external_user_id, join_token);
    
    return std::make_unique<MeetingSessionCredentials>(
        MeetingSessionCredentials { attendee_id, external_user_id, join_token }
    );
}