#pragma once
#include <string>
#include <memory>
#include <rust/cxx.h>
#include <chimer/src/main.rs.h>
#include "signaling/default_signaling_client_factory.h"
#include "session/meeting_session_credentials.h"
std::unique_ptr<std::string> hello_gang();

// struct SomeStruct{
//     rust::String foo;
//     int8_t bar;
// };
struct SomeStruct;

int8_t do_struct(SomeStruct x);

// const auto&
std::unique_ptr<std::string> hello_gang();


using MeetingSessionCredentials = chime::MeetingSessionCredentials;
std::unique_ptr<MeetingSessionCredentials> make_session_credentials(rust::String attendee_id, rust::String external_user_id, rust::String join_token);