//! # rtcm-rs
//!
//! `rtcm-rs` is a Rust library for decoding and encoding RTCM version 3 messages as defined in the RTCM Standard 10403.x.
//! This library aims to provide an efficient, safe, and easy-to-use way to handle RTCM messages. Currently, the library
//! supports a subset of the RTCM standard's messages, but we're working to extend the support to all messages.
//!
//! This library uses `#[forbid(unsafe_code)]` attribute, ensuring that all operations are safe from undefined behavior,
//! data races, and many common bugs. Therefore, you can rely on `rtcm-rs` for not only its functionality but also its commitment to safety.
//!
//! The library also provides support for `serde`, a powerful serialization and deserialization framework, allowing users to convert RTCM
//! messages into various formats such as JSON, XML and more. Furthermore, `rtcm-rs` is `no_std` compatible and doesn't rely
//! on dynamic memory allocations, which makes it suitable for use in embedded systems.
//!
//! Here are some examples of how you can use `rtcm-rs`:
//!
//! ## Decoding a RTCM message
//! ```no_run
//! use rtcm_rs::prelude::*;
//! use std::io::Read;
//!
//! # fn main() {
//! let mut rtcm_file = std::fs::File::open("testdata/msg1001_3.rtcm").unwrap();
//! let mut buffer = Vec::<u8>::new();
//! rtcm_file.read_to_end(&mut buffer).unwrap();
//!
//! if let (_, Some(message_frame)) = next_msg_frame(buffer.as_slice()) {
//!     let msg = message_frame.get_message();
//!     println!("{:?}", msg);
//! }
//! # }
//! ```
//!
//! ## Encoding a RTCM message
//! ```no_run
//! use rtcm_rs::prelude::*;
//! use rtcm_rs::msg::{Msg1001T, Msg1001Sat};
//! use rtcm_rs::util::DataVec;
//!
//! # fn main() {
//! let mut message_builder = MessageBuilder::new();
//!
//! let result = message_builder.build_message(
//!     &Message::Msg1001(
//!         Msg1001T {
//!             reference_station_id: 100,
//!             gps_epoch_time_ms: 0,
//!             synchronous_gnss_msg_flag: 0,
//!             satellites_len: 2,
//!             gps_divergence_free_smoothing_flag: 0,
//!             gps_smoothing_interval_bitval: 0,
//!             satellites:  {
//!                 let mut satellites = DataVec::new();
//!                 satellites.push(Msg1001Sat {
//!                     gps_satellite_id: 20,
//!                     gps_l1_code_ind: 0,
//!                     gps_l1_pseudorange_m: Some(20000000.0),
//!                     gps_l1_phase_pseudorange_diff_m: Some(0.2),
//!                     gps_l1_lock_time_bitval: 0 });
//!                 satellites.push(Msg1001Sat {
//!                     gps_satellite_id: 21,
//!                     gps_l1_code_ind: 0,
//!                     gps_l1_pseudorange_m: Some(26000000.0),
//!                     gps_l1_phase_pseudorange_diff_m: Some(0.4),
//!                     gps_l1_lock_time_bitval: 0 });
//!                 satellites
//!             }}));
//!
//! if let Ok(bytes) = result {
//!     println!("Encoded message: {:?}", bytes);
//! }
//! # }
//! ```
//!
//! For a full list of features and capabilities, see the [README](https://github.com/martinhakansson/rtcm-rs/blob/master/README.md).

#![no_std]
#![forbid(unsafe_code)]
//use message::{Message, MessageBuilder};
//use preamble::MessageFrame;
use crc_any;
#[cfg(feature = "serde")]
use sd::{de::Visitor, Deserialize, Serialize};
use tinyvec;

#[cfg(feature = "test_gen")]
mod source_repr;
#[cfg(feature = "test_gen")]
pub mod val_gen;

mod df;
pub mod msg;
pub mod rtcm_error;
pub mod util;

pub use msg::message::{Message, MessageBuilder};

mod message_frame;
pub use message_frame::MessageFrame;

pub mod prelude {
    pub use crate::rtcm_error::RtcmError;
    #[cfg(feature = "test_gen")]
    pub use crate::source_repr::{SourceOutput, SourceRepr};
    pub use crate::{next_msg_frame, Message, MessageBuilder, MessageFrame, MsgFrameIter};
}

/// next_msg_frame takes a slice of bytes and returns a tuple containing the number of bytes consumed, and a `MessageFrame` if one was found.
///
/// # Parameters
///
/// * `data` - A `&[u8]` to search for a `MessageFrame`
///
/// # Returns
///
/// A tuple containing the number of bytes consumed, and a `MessageFrame` if one was found.
pub fn next_msg_frame(data: &[u8]) -> (usize, Option<MessageFrame>) {
    for (i, b) in data.iter().enumerate() {
        if *b == 0xd3 {
            match MessageFrame::new(&data[i..]) {
                Ok(m) => return (i + m.frame_len(), Some(m)),
                Err(rtcm_error::RtcmError::Incomplete) => return (i, None),
                Err(rtcm_error::RtcmError::NotValid) => {
                    continue;
                },
                _ => unreachable!(),
            }
        }
    }
    (data.len(), None)
}

/// MsgFrameIter is an iterator that returns `MessageFrame`s found in a `&[u8]`.
pub struct MsgFrameIter<'a> {
    data: &'a [u8],
    index: usize,
}
impl<'a> MsgFrameIter<'a> {
    /// Creates a new MsgFrameIter from a `&[u8]`
    ///
    /// # Parameters
    ///
    /// * `data` - A `&[u8]` to search for `MessageFrame`s
    pub fn new(data: &'a [u8]) -> Self {
        MsgFrameIter { data, index: 0 }
    }
    /// Returns the number of bytes consumed by the iterator so far
    pub fn consumed(&self) -> usize {
        self.index
    }
}
impl<'a> core::iter::Iterator for &mut MsgFrameIter<'a> {
    type Item = MessageFrame<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.data.len() {
            return None;
        }
        let (consumed, mf) = next_msg_frame(&self.data[self.index..]);
        self.index += consumed;
        mf
    }
}

#[cfg(test)]
mod test {
    use crate::util::DataVec;

    use super::*;
    // fn test_create_1001() {
    //     let mut msg = msg::Msg1071T {
    //         reference_station_id: todo!(),
    //         epoch_time: todo!(),
    //         multiple_message_bit: todo!(),
    //         issue_of_data_station: todo!(),
    //         reserved: todo!(),
    //         clock_steering_indicator: todo!(),
    //         external_clock_indicator: todo!(),
    //         divergence_free_smoothing_indicator: todo!(),
    //         smoothing_interval: todo!(),
    //         data_segment: msg::Msg1071Data { sat_data: DataVec::new(), sig_data: DataVec::new() },
    //     };
    //     msg.data_segment.sat_data.push(msg::Msm123Sat::default());
    // }
}
