# rtcm-rs

[![Crates.io](https://img.shields.io/crates/v/rtcm-rs.svg)](https://crates.io/crates/rtcm-rs)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/martinhakansson/rtcm-rs#license)
[![CI](https://github.com/martinhakansson/rtcm-rs/workflows/CI/badge.svg)](https://github.com/martinhakansson/rtcm-rs/actions)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.66-green.svg)

![Unsafe-Zero-Percent](https://img.shields.io/badge/Unsafety-0%25-brightgreen.svg)

rtcm-rs is a powerful Rust library for decoding and encoding RTCM version 3 messages as defined in the RTCM Standard 10403.x. The library currently provides complete support up to and including the latest version 3.4 (10403.4) of the standard. Support for future versions will be added upon their release.

In the spirit of Rust's safety principles, this library is developed with #[forbid(unsafe_code)], ensuring that all operations are safe from undefined behavior, data races, and many common bugs. Thus, you can rely on rtcm-rs for not only its functionality but also its commitment to safety.

This library provides robust support for Serde, a powerful serialization/deserialization framework, facilitating conversion of RTCM messages into other formats such as JSON, XML and more. For an example of this functionality, see the [rtcm-json](https://github.com/martinhakansson/rtcm-json) crate.

Moreover, the library is `no_std` compatible and doesn't rely on dynamic memory allocations, making it suitable for use in embedded environments. With feature flags for each message type, rtcm-rs can be tailored to your needs, reducing library size when necessary.

## Features

- `serde`: For adding support for serialization and deserialization. To enable this, add the following to the rtcm-rs dependency in your Cargo.toml file:

```toml
rtcm-rs = { version = "0.11.0", features=["serde"] }
```

- Selective message support: To minimize the library size by supporting only certain RTCM messages. For instance, to only support messages 1004 and 1005, update your Cargo.toml as follows:

```toml
rtcm-rs = { version = "0.11.0", default-features=false, features=["msg1001","msg1005"] }
```

- No standard library: Usage of the standard library is controlled by the feauture `std`, which is defined as a default feature for the library. So to disable the standard library, make sure to set default-features to false. 
```toml
rtcm-rs = { version = "0.11.0", default-features=false }
```

- `test_gen`: This feature is used exclusively for generating tests during library development and is not necessary for library usage.

## Usage

To add rtcm-rs to your project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
rtcm-rs = "0.11.0"
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
    let result = message_builder.build_message(&Message::Msg1001(Msg1001T {
        reference_station_id: 100,
        gps_epoch_time_ms: 0,
        synchronous_gnss_msg_flag: 0,
        divergence_free_smoothing_flag: 0,
        smoothing_interval_index: 0,
        satellites: {
            let mut satellites = DataVec::new();
            satellites.push(Msg1001Sat {
                gps_satellite_id: 20,
                gps_l1_code_ind: 0,
                l1_pseudorange_m: Some(20000000.0),
                l1_phase_pseudorange_diff_m: Some(0.2),
                l1_lock_time_index: 0,
            });
            satellites.push(Msg1001Sat {
                gps_satellite_id: 21,
                gps_l1_code_ind: 0,
                l1_pseudorange_m: Some(26000000.0),
                l1_phase_pseudorange_diff_m: Some(0.4),
                l1_lock_time_index: 0,
            });
            satellites
        },
    }));

    // If the message is successfully built, print the encoded bytes
    if let Ok(bytes) = result {
        println!("Encoded message: {:?}", bytes);
    }
}
```

In this third example, we demonstrate how to encode an RTCM message. We start by initializing a new `MessageBuilder` instance and then use its `build_message` method to construct a new RTCM message. If the message is successfully built, the resulting byte array representing the message is printed. This example shows how to create and encode a complex RTCM message containing satellite information.

## Supported messages

All RTCM v. 3.4 (10403.4) messages supported.

## Roadmap to Version 1.0
- [x] Full coverage of all RTCM version 3 messages
  - [x] MSM Observable messages
  - [x] Station meta data messages
  - [x] Auxiliary info messages
  - [x] Ephemeris messages
  - [x] SSR messages
  - [x] Transformation parameters messages
  - [x] Projection parameters messages
  - [x] Network RTK corrections messages
  - [x] Legacy Observable messages
  - [x] Version 3.4 messages
- [x] Stabilize API (may break some backward compatibility)
  - [x] Consistent message data types (and message field names)
- [ ] Expand unit tests
- [ ] Performance optimizations
- [ ] Enhanced documentation

## Changelog

### Version 0.11.0
This version adds support for the following new messages:
- MSM messages for NavIC (1131-1137)
- New RTCM 3.4 messages (1300-1304)

Additionally, a few bugs have been fixed:
- Error in signal mapping for BeiDou MSM
- Possible rounding error for some messages containing fields of type f32 and f64.

### Version 0.10.0
This update includes changes to the API to make it more consistent. Unfortunately, this may also break backward compatibility with previous versions in some cases.

Changes:
- Revision of field names for some messages, and removal of redundant length fields for some messages with vectors or strings. Affected messages are:
  - All MSM (1071-1127) 
  - 1001
  - 1007
  - 1008
  - 1013
  - 1030
  - 1033
  - 1059
  - 1065
  - 1230
- Removed and merged some vector/string capacity constants 
- A new feature `std` was added which controls reliance on the standard library. The purpose is more convenient error handling as it allows the RtcmError type to implement the Error trait of the standard library. Feature `std` is defined as a default feature for the library, so to turn it off the `default-features` parameter needs to be set to false in Cargo.toml.

### Version 0.9.0

- Full RTCM v. 3.3 support

## License

MIT or Apache-2.0
