
macro_rules! test_msg {
    [
        $( (test_id: $test_id:ident, in_file: $in_file:literal, rtcm_file: $rtcm_file:literal) ),+
    ] => {
        use $crate::rtcm3::preamble::*;
        $(
            mod $test_id {
                fn test_encode() {
                    let msg = include!($in_file);
                    let msg_bytes = include_bytes!($rtcm_file);
                    let mut msg_builder = MessageBuilder::new();
                    let encoded_msg = if let Ok(encoded_msg) = msg_builder.build_message(&msg) {
                        encoded_msg
                    } else {
                        panic!("Could not encode message");
                    }
                    assert_eq!(msg_bytes, encoded_msg);
                }
                fn test_decode() {

                }
            }
        )+
    };
}