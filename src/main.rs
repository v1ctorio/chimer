fn main() {
    println!("Hello, world!");
    let ret = ffi::hello_gang();
    println!("{:?}", ret);
}

#[cxx::bridge]
mod ffi {




    unsafe extern "C++" {
        // include!(<thirdparty/chime-sdk-signaling-cpp/src/signaling/default_signaling_client_factory.h>);
        include!(<chime-bridge.h>);

        fn hello_gang() -> UniquePtr<CxxString>;

        // type SingalingClient;
        // fn CreateSignalingClient()->UniquePtr<SignalingClient>;
// 
        // type MeetingSessionCredentials;
        // type MeetingSessionURLs;
        // type MeetingSessionConfiguration;
        // type SignalingClientConfiguration;
        // type DefaultSignalingDependencies;

    }
}