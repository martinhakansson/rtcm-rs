use rtcm_rs::prelude::*;
use std::io::Read;

fn main() {
    // Open the test data file containing RTCM messages
    let mut rtcm_file = std::fs::File::open("testdata/msg1001_3.rtcm").unwrap();
    let mut buffer = Vec::<u8>::new();
    
    // Read the entire file into the buffer
    if let Ok(_) = rtcm_file.read_to_end(&mut buffer) {
        // Use next_msg_frame function to extract the next RTCM message frame from the buffer
        if let (_, Some(message_frame)) = next_msg_frame(buffer.as_slice()) {            
            // Get the actual RTCM message from the frame
            let msg = message_frame.get_message();
            // Print the message
            println!("{:?}", msg);
        }
    }
}