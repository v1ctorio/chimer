fn main() {
    println!("Hello, world!");
    ffi::foo()
}

#[cxx::bridge]
mod ffi {
    use cxx::UniquePtr;


    unsafe extern "C++" {
        include!(<thirdparty/chime-sdk-signaling-cpp/src/signaling/default_signaling_client_factory.h>);

        type SingalingClient;
        fn CreateSignalingClient()->UniquePtr<SignalingClient>;

        type MeetingSessionCredentials;
        type MeetingSessionURLs;
        type MeetingSessionConfiguration;
        type SignalingClientConfiguration;
        type DefaultSignalingDependencies;

    }
}