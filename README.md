# rtcm-rs

[![Crates.io](https://img.shields.io/crates/v/rtcm-rs.svg)](https://crates.io/crates/rtcm-rs)

rtcm-rs is a powerful Rust library for decoding and encoding RTCM version 3 messages as defined in the RTCM Standard 10403.x. As of now, it supports a subset of the messages in the standard, but work is being done to cover them all in the near future.

In the spirit of Rust's safety principles, this library is developed with #[forbid(unsafe_code)], ensuring that all operations are safe from undefined behavior, data races, and many common bugs. Thus, you can rely on rtcm-rs for not only its functionality but also its commitment to safety.

This library provides robust support for Serde, a powerful serialization/deserialization framework, facilitating conversion of RTCM messages into other formats such as JSON, XML and more. For an example of this functionality, see the [rtcm-json](https://github.com/martinhakansson/rtcm-json) crate.

Moreover, the library is `no_std` compatible and doesn't rely on dynamic memory allocations, making it suitable for use in embedded environments. With feature flags for each message type, rtcm-rs can be tailored to your needs, reducing library size when necessary.

## Features

- `serde`: For adding support for serialization and deserialization. To enable this, add the following to the rtcm-rs dependency in your Cargo.toml file:

```toml
rtcm-rs = { version = "0.2.0", features=["serde"] }
```

- Selective message support: To minimize the library size by supporting only certain RTCM messages. For instance, to only support messages 1004 and 1005, update your Cargo.toml as follows:

```toml
rtcm-rs = { version = "0.2.0", default-features=false, features=["msg1004","msg1005"] }
```

- `test_gen`: This feature is used exclusively for generating tests during library development and is not necessary for library usage.

## Usage

To add rtcm-rs to your project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
rtcm-rs = "0.2.0"
```

Remember that hyphenated crate names translate to underscored crate names in rust source code, i.e. `rtcm_rs` for this crate. For instance, add the following to import from the prelude:

```rust
use rtcm_rs::prelude::*;
```

## Examples

Here are some examples of how to decode and encode RTCM messages using this library:

### Basic RTCM message decoding

```rust
use rtcm_rs::prelude::*;
use std::io::Read;

fn main() {
    // Open the test data file containing RTCM messages
    let mut rtcm_file = std::fs::File::open("testdata/msg1001_3.rtcm").unwrap();
    let mut buffer = Vec::<u8>::new();
    
    // Read the entire file into the buffer
    if let Ok(_) = rtcm_file.read_to_end(&mut buffer) {
        // Use next_msg_frame function to extract the next RTCM message from the buffer
        if let (_, Some(message_frame)) = next_msg_frame(buffer.as_slice()) {            
            // Get the actual RTCM message from the frame
            let msg = message_frame.get_message();
            // Print the message
            println!("{:?}", msg);
        }
    }
}
```

### Using iterator functionality for decoding multiple RTCM messages

```rust
use rtcm_rs::prelude::*;
use std::io::Read;

fn main() {
    // Open the test data file containing RTCM messages
    let mut rtcm_file = std::fs::File::open("testdata/msgs_1.rtcm").unwrap();
    let mut buffer = Vec::<u8>::new();

    if let Ok(_) = rtcm_file.read_to_end(&mut buffer) {
        // Create an iterator over the RTCM message frames in the buffer
        let mut iterator = MsgFrameIter::new(buffer.as_slice());

        // Iterate over the message frames and print the message number
        for message_frame in &mut iterator {
            println!("Message {}", message_frame.message_number().unwrap());
        }
    }    
}
```

### Encoding a RTCM message

```rust
use rtcm_rs::prelude::*;
use rtcm_rs::msg::{Msg1001T, Msg1001Sat};
use rtcm_rs::util::DataVec;

fn main() {
    // Initialize a new message builder
    let mut message_builder = MessageBuilder::new();
    
    // Build the RTCM message
    let result = message_builder.build_message(
        &Message::Msg1001(
            Msg1001T {
                reference_station_id: 100, 
                gps_epoch_time_ms: 0, 
                synchronous_gnss_msg_flag: 0, 
                satellites_len: 2, 
                gps_divergence_free_smoothing_flag: 0, 
                gps_smoothing_interval_bitval: 0, 
                satellites:  {
                    let mut satellites = DataVec::new();
                    satellites.push(Msg1001Sat {
                        gps_satellite_id: 20,
                        gps_l1_code_ind: 0, 
                        gps_l1_pseudorange_m: Some(20000000.0), 
                        gps_l1_phase_pseudorange_diff_m: Some(0.2), 
                        gps_l1_lock_time_bitval: 0 });
                    satellites.push(Msg1001Sat {
                        gps_satellite_id: 21,
                        gps_l1_code_ind: 0, 
                        gps_l1_pseudorange_m: Some(26000000.0), 
                        gps_l1_phase_pseudorange_diff_m: Some(0.4), 
                        gps_l1_lock_time_bitval: 0 });
                    satellites
                }
            }
        )
    );
    
    // If the message is successfully built, print the encoded bytes
    if let Ok(bytes) = result {
        println!("Encoded message: {:?}", bytes);
    }
}
```

In this third example, we demonstrate how to encode an RTCM message. We start by initializing a new `MessageBuilder` instance and then use its `build_message` method to construct a new RTCM message. If the message is successfully built, the resulting byte array representing the message is printed. This example shows how to create and encode a complex RTCM message containing satellite information.

## License

MIT or Apache-2.0