# rtcm-rs

[![Crates.io](https://img.shields.io/crates/v/rtcm-rs.svg)](https://crates.io/crates/rtcm-rs)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/martinhakansson/rtcm-rs#license)
[![CI](https://github.com/martinhakansson/rtcm-rs/workflows/CI/badge.svg)](https://github.com/martinhakansson/rtcm-rs/actions)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.66-green.svg)

![Unsafe-Zero-Percent](https://img.shields.io/badge/Unsafety-0%25-brightgreen.svg)

rtcm-rs is a powerful Rust library for decoding and encoding RTCM version 3 messages as defined in the RTCM Standard 10403.x. As of now, it supports a subset of the messages in the standard, but work is being done to cover them all in the near future.

In the spirit of Rust's safety principles, this library is developed with #[forbid(unsafe_code)], ensuring that all operations are safe from undefined behavior, data races, and many common bugs. Thus, you can rely on rtcm-rs for not only its functionality but also its commitment to safety.

This library provides robust support for Serde, a powerful serialization/deserialization framework, facilitating conversion of RTCM messages into other formats such as JSON, XML and more. For an example of this functionality, see the [rtcm-json](https://github.com/martinhakansson/rtcm-json) crate.

Moreover, the library is `no_std` compatible and doesn't rely on dynamic memory allocations, making it suitable for use in embedded environments. With feature flags for each message type, rtcm-rs can be tailored to your needs, reducing library size when necessary.

## Features

- `serde`: For adding support for serialization and deserialization. To enable this, add the following to the rtcm-rs dependency in your Cargo.toml file:

```toml
rtcm-rs = { version = "0.8.0", features=["serde"] }
```

- Selective message support: To minimize the library size by supporting only certain RTCM messages. For instance, to only support messages 1004 and 1005, update your Cargo.toml as follows:

```toml
rtcm-rs = { version = "0.8.0", default-features=false, features=["msg1001","msg1005"] }
```

- `test_gen`: This feature is used exclusively for generating tests during library development and is not necessary for library usage.

## Usage

To add rtcm-rs to your project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
rtcm-rs = "0.8.0"
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

## Supported messages

| Version | Message Support Added | Notes |
| --- | --- | --- |
| 0.1.x | 1001 1005 1007 1008 1030 1071 1074 | |
| 0.2.x | 1084 1094 1104 1114 1124 | MSM4 support |
| 0.3.x | 1072 1073 1075 1076 1077 1081 1082 1083 1085 1086 1087 1091 1092 1093 1095 1096 1097 1101 1102 1103 1105 1106 1107 1111 1112 1113 1115 1116 1117 1121 1122 1123 1125 1126 1127 | Full MSM support |
| 0.4.x | 1006 1013 1029 1032 1033 1230 | Station meta data and some auxiliary info messages |
| 0.5.x | 1019 1020 1041 1042 1044 1045 1046 | Ephemeris messages |
| 0.6.x | 1057 1058 1059 1060 1061 1062 1063 1064 1065 1066 1067 1068 | SSR messages |
| 0.7.x | 1021 1022 1023 1024 1025 1026 1027 | Transformation and projection messages |
| 0.8.x | 1014 1015 1016 1017 1031 1034 1035 1037 1038 1039 | Network RTK corrections messages |

## Roadmap to Version 1.0
- [ ] Full coverage of all RTCM version 3 messages
  - [x] MSM Observable messages
  - [x] Station meta data messages
  - [x] Auxiliary info messages
  - [x] Ephemeris messages
  - [x] SSR messages
  - [x] Transformation parameters messages
  - [x] Projection parameters messages
  - [x] Network RTK corrections messages
  - [ ] Legacy Observable messages
- [ ] Stabilize API (may break some backward compatibility)
  - [ ] Consistent message data types (and message field names)
- [ ] Expand unit tests
- [ ] Performance optimizations
- [ ] Enhanced documentation

## License

MIT or Apache-2.0
