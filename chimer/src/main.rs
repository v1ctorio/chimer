use chime_sdk_rs::ffi::{MeetingSessionCredentials,SomeStruct,do_struct,make_session_credentials};

fn main() {
    println!("Hello, world!");
    let ret = chime_sdk_rs::ffi::hello_gang();
    println!("{:?}", ret);
    let something = SomeStruct {bar: 69, foo: "quecosa".into() };
    let ret = do_struct(something);
    println!("{:?}", ret);

    let credentials = make_session_credentials(
        "what".into(),
        "external_user_id".into(),
        "join_token".into(),
        );

    println!("Credentials pointer = {:p}", credentials.as_ptr());
}
