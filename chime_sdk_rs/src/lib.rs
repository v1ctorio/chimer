use crate::ffi::{SomeStruct, do_struct, hello_gang, make_session_credentials};



#[cxx::bridge]
pub mod ffi {



    pub struct SomeStruct{
        foo: String,
        bar: i8,
    }
    

    #[namespace = "chime_bridge"]
    unsafe extern "C++" {
        // include!(<thirdparty/chime-sdk-signaling-cpp/src/signaling/default_signaling_client_factory.h>);
        include!(<chime_bridge.h>);

        pub fn hello_gang() -> UniquePtr<CxxString>;
        pub fn do_struct(x: SomeStruct) -> i8;
        // type SingalingClient;
        // fn CreateSignalingClient()->UniquePtr<SignalingClient>;
// 
        // type MeetingSessionCredentials;
        // type MeetingSessionURLs;
        // type MeetingSessionConfiguration;
        // type SignalingClientConfiguration;
        // type DefaultSignalingDependencies;
        pub fn make_session_credentials(attendee_id: String, external_user_id: String, join_token: String) -> UniquePtr<MeetingSessionCredentials>;

    }

    #[namespace = "chime"]
    unsafe extern "C++" {
        include!(<chime_bridge.h>);

        pub type MeetingSessionCredentials;

    }
}
