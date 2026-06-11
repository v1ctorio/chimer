use crate::ffi::{SomeStruct, do_struct, hello_gang, make_session_credentials};

fn main() {
    println!("Hello, world!");
    let ret = ffi::hello_gang();
    println!("{:?}", ret);
    let something = SomeStruct {bar: 69, foo: "quecosa".into() };
    let ret = do_struct(something);
    println!("{:?}", ret);

    let credentials = make_session_credentials(
        "what".into(),
        "external_user_id".into(),
        "join_token".into(),
        );

    println!("Credentials pointer = {}", credentials);
}

#[cxx::bridge]
mod ffi {



    struct SomeStruct{
        foo: String,
        bar: i8,
    }
    


    unsafe extern "C++" {
        // include!(<thirdparty/chime-sdk-signaling-cpp/src/signaling/default_signaling_client_factory.h>);
        include!(<chime-bridge.h>);

        fn hello_gang() -> UniquePtr<CxxString>;
        fn do_struct(x: SomeStruct) -> i8;
        // type SingalingClient;
        // fn CreateSignalingClient()->UniquePtr<SignalingClient>;
// 
        // type MeetingSessionCredentials;
        // type MeetingSessionURLs;
        // type MeetingSessionConfiguration;
        // type SignalingClientConfiguration;
        // type DefaultSignalingDependencies;

    }

    unsafe extern "C++" {
        include!(<chime-bridge.h>);

        type MeetingSessionCredentials;
        fn make_session_credentials(attendee_id: String, external_user_id: String, join_token: String) -> UniquePtr<MeetingSessionCredentials>;

    }
}
