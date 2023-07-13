use rtcm_rs::prelude::*;
use std::io::Read;

fn main() {
    // Open the test data file containing RTCM messages
    let mut rtcm_file = std::fs::File::open("testdata/msgs_1.rtcm").unwrap();
    let mut buffer = Vec::<u8>::new();

    if let Ok(_) = rtcm_file.read_to_end(&mut buffer) {
        let mut iterator = MsgFrameIter::new(buffer.as_slice());

        for message_frame in &mut iterator {
            println!("Message {}", message_frame.message_number().unwrap());
        }
    }
}
