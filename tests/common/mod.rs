macro_rules! test_msg {
    [
        $( $test_id:ident ),+
    ] => {
        $(
            mod $test_id {
                use rtcm3::preamble::*;
                use rtcm3::msg::*;
                use rtcm3::util::*;
                #[test]
                fn test_encode() {
                    let msg = include!(concat!("../testdata/",stringify!($test_id),".in"));
                    let msg_bytes = include_bytes!(concat!("../testdata/",stringify!($test_id),".rtcm"));
                    let mut msg_builder = MessageBuilder::new();
                    let encoded_msg = if let Ok(encoded_msg) = msg_builder.build_message(&msg) {
                        encoded_msg
                    } else {
                        panic!("Could not encode message");
                    };
                    assert_eq!(msg_bytes, encoded_msg);
                }
                #[test]
                fn test_decode() {
                    let msg = include!(concat!("../testdata/",stringify!($test_id),".in"));
                    let msg_bytes = include_bytes!(concat!("../testdata/",stringify!($test_id),".rtcm"));
                    if let (_, Some(msg_frame)) = next_msg_frame(msg_bytes) {
                        let decoded_msg = msg_frame.get_message();
                        assert_eq!(msg, decoded_msg);
                    } else {
                        panic!("Could not decode message");
                    }
                }
            }
        )+
    };
}
