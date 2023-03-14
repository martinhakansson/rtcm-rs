use crate::crc_any::CRC;
use crate::msg::message::Message;
use crate::rtcm_error::RtcmError;

/// Represents the message frame of a RTCM 3.x message.
/// 
/// Holds a reference to the frame data as a slice for valid RTCM message frames,
/// and provides various methods for access to data segment, message number, and CRC24
pub struct MessageFrame<'a> {
    frame_data: &'a [u8],
    data: &'a [u8],
    crc: u32,
    message_number:Option<u16>,
}

impl<'a> MessageFrame<'a> {
    /// Creates a new `MessageFrame` from the given frame data.
    /// 
    /// Parameters:
    /// - `frame_data` - A slice of bytes representing the frame data.
    /// 
    /// Errors:
    /// - `RtcmError::Incomplete` - The given frame data is incomplete.
    /// - `RtcmError::NotValid` - The given frame data is not valid.
    /// 
    /// Returns:
    /// A `MessageFrame` if the given data represents a valid RTCM message frame.
    pub fn new(frame_data:&'a [u8]) -> Result<Self,RtcmError> {
        if frame_data.len() < 6 {
            return Err(RtcmError::Incomplete);
        }
        if frame_data[0] != 0xd3 {
            return Err(RtcmError::NotValid);
        }
        let length:usize = ((frame_data[1] as usize & 0b11) << 8) | (frame_data[2] as usize);
        if frame_data.len() < length + 6 {
            return Err(RtcmError::Incomplete);
        }
        let mut crc24 = CRC::crc24lte_a();
        crc24.digest(&frame_data[0..length+3]);
        
        let msg_crc = 
            ((frame_data[length+3] as u32) << 16) |
            ((frame_data[length+4] as u32) << 8) |
            (frame_data[length+5] as u32);
        if msg_crc != crc24.get_crc() as u32 {
            return Err(RtcmError::NotValid);
        }
        let message_number:Option<u16> = if frame_data.len() >= 8 {
            Some(((frame_data[3] as u16) << 4) | ((frame_data[4] as u16) >> 4))
        } else {
            None
        };
        Ok(MessageFrame {
            frame_data: &frame_data[..length+6],
            data: &frame_data[3..length+3],
            crc: msg_crc,
            message_number,
        })
    }
    /// Returns the data segment bytes of the RTCM frame as a slice
    pub fn data(&'a self) -> &'a [u8] {
        &*self.data
    }
    /// Returns the bytes of the whole RTCM frame as a slice
    pub fn frame_data(&'a self) -> &'a [u8] {
        &*self.frame_data
    }
    /// Returns the data segment byte length
    pub fn data_len(&self) -> usize {
        self.data.len()
    }
    /// Returns the RTCM message frame byte length
    pub fn frame_len(&self) -> usize {
        self.frame_data.len()
    }
    /// Returns the RTCM message frame CRC24 checksum
    pub fn crc(&self) -> u32 {
        self.crc
    }
    /// Returns the RTCM message number if the frame contains data. Otherwise returns None.
    pub fn message_number(&self) -> Option<u16> {
        self.message_number
    }
    /// Decodes the content of the RTCM message frame and returns the result as a `Message` enum.
    pub fn get_message(&self) -> Message {
        Message::from_message_frame(self)
    }
}