#![no_std]
#![forbid(unsafe_code)]
//use message::{Message, MessageBuilder};
//use preamble::MessageFrame;
use tinyvec;
use crc_any;
#[cfg(feature = "serde")]
use sd::{Serialize,Deserialize,de::Visitor};

#[cfg(feature = "test_gen")]
mod source_repr;

pub mod rtcm_error;
mod df;
pub mod msg;
pub mod util;

pub use msg::message::{Message, MessageBuilder};

mod message_frame;
pub use message_frame::MessageFrame;

pub mod preamble {
    pub use crate::rtcm_error::RtcmError;
    pub use crate::{Message, MessageBuilder, MessageFrame, MsgFrameIter};
    #[cfg(feature = "test_gen")]
    pub use crate::source_repr::{SourceOutput,SourceRepr};
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
pub fn next_msg_frame(data:&[u8]) -> (usize,Option<MessageFrame>) {
    for (i,b) in data.iter().enumerate() {
        if *b == 0xd3 {
            match MessageFrame::new(&data[i..]) {
                Ok(m) => return (i+1+m.frame_len(),Some(m)),
                Err(rtcm_error::RtcmError::Incomplete) => { return (i, None)},
                Err(rtcm_error::RtcmError::NotValid) => { continue; },
            }
        }
    }
    (data.len(), None)
}

/// MsgFrameIter is an iterator that returns `MessageFrame`s found in a `&[u8]`.
pub struct MsgFrameIter<'a> {
    data:&'a [u8],
    index:usize,
}
impl<'a> MsgFrameIter<'a> {
    /// Creates a new MsgFrameIter from a `&[u8]`
    /// 
    /// # Parameters
    /// 
    /// * `data` - A `&[u8]` to search for `MessageFrame`s
    pub fn new(data:&'a [u8]) -> Self {
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
        if self.index >= self.data.len() { return None; }
        let (consumed,mf) = next_msg_frame(&self.data[self.index..]);
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