#pragma once
#include <string>
#include <memory>
#include <rust/cxx.h>
#include "signaling/default_signaling_client_factory.h"
#include "session/meeting_session_credentials.h"
#include <chimer/src/main.rs.h>

// struct SomeStruct{
//     rust::String foo;
//     int8_t bar;
// };

struct SomeStruct;

namespace chime_bridge {


    int8_t do_struct(SomeStruct x);

    std::unique_ptr<std::string> hello_gang();


    using MeetingSessionCredentials = chime::MeetingSessionCredentials;
    std::unique_ptr<MeetingSessionCredentials> make_session_credentials(rust::String attendee_id, rust::String external_user_id, rust::String join_token);
}
