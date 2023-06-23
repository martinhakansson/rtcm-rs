use crate::crc_any::CRC;
use crate::df::bit_value::U16;
use crate::df::{assembler::Assembler, parser::Parser};
use crate::message_frame::MessageFrame;
use crate::rtcm_error::RtcmError;
#[cfg(feature = "serde")]
use sd::{Deserialize, Serialize};

macro_rules! message {
    (
        $( $feature:literal: $enum_v:ident($msg_id:ident) = $enum_pv:literal ),*
    ) => {
        $(
            #[cfg(feature = $feature)]
            use $crate::msg::$msg_id::$msg_id;
        )*
        #[non_exhaustive]
        #[repr(u16)]
        #[cfg_attr(feature="serde",derive(Serialize,Deserialize),serde(crate = "sd"))]
        #[derive(Debug,PartialEq)]
        pub enum Message {
            Empty = 5000,
            Corrupt = 6000,
            MsgNotSupported(MsgNotSupportedT) = 7000,
            $(
                #[cfg(feature = $feature)]
                $enum_v($msg_id::DataType) = $enum_pv
            ),*
        }
        impl Message {
            pub fn from_message_frame(message_frame: &MessageFrame) -> Self {
                let message_number = if let Some(message_number) = message_frame.message_number() {
                    message_number
                } else {
                    return Message::Empty;
                };
                let mut parser = Parser::new(message_frame.data(), 12);
                match message_number {
                    $(
                        #[cfg(feature = $feature)]
                        $enum_pv => if let Ok(value) = $msg_id::decode(&mut parser) {
                            Message::$enum_v(value)
                        } else {
                            Message::Corrupt
                        },
                    )*
                    _ => Message::MsgNotSupported(MsgNotSupportedT {
                        message_number
                    }),
                }
            }
            pub fn number(&self) -> Option<u16> {
                match self {
                    $(
                        #[cfg(feature = $feature)]
                        Message::$enum_v(_) => {
                            Some($enum_pv)
                        }
                    )*
                    _ => {
                        None
                    },
                }
            }
        }
        #[cfg_attr(feature="serde",derive(Serialize,Deserialize),serde(crate = "sd"))]
        #[derive(Debug,PartialEq)]
        pub struct MsgNotSupportedT {
            pub message_number:u16,
        }

        #[cfg(feature = "test_gen")]
        impl $crate::source_repr::SourceRepr for Message {
            fn to_source(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                use core::fmt::Write;
                match self {
                    Message::Empty => {
                        f.write_str("Message::Empty")
                    },
                    Message::Corrupt => {
                        f.write_str("Message::Corrupt")
                    },
                    $(
                        #[cfg(feature = $feature)]
                        Message::$enum_v(msgt) => {
                            write!(f, "Message::{}(", stringify!($enum_v))?;
                            msgt.to_source(f)?;
                            f.write_char(')')
                        },
                    )*
                    Message::MsgNotSupported(msgt) => {
                        write!(f, "Message::MsgNotSupported(MsgNotSupportedT{{ message_number:{} }})", msgt.message_number)
                    },
                }
            }
        }

        pub struct MessageBuilder {
            data:[u8;1029],
            has_run:bool,
        }
        #[cfg(feature = "test_gen")]
        use $crate::val_gen::ValGen;

        impl MessageBuilder {
            pub fn new() -> Self {
                let mut ret = MessageBuilder {
                    data: [0;1029],
                    has_run:false,
                };
                ret.data[0] = 0xd3;
                ret
            }
            pub fn build_message(&mut self, message:&Message) -> Result<&[u8],RtcmError> {
                if self.has_run {
                    self.clear_data();
                }
                self.has_run = true;
                let mut asm = Assembler::new(&mut self.data[3..1026], 0);
                //encode message number into message
                if let Some(number) = message.number() {
                    asm.put::<U16>(number,12)?;
                } else {
                    return Err(RtcmError::EncodingNotSupported);
                }

                match message {
                    $(
                        #[cfg(feature = $feature)]
                        Message::$enum_v(dt) => {
                            $msg_id::encode(&mut asm,&dt)?;
                        }
                    )*
                    _ => {
                        unreachable!();
                    },
                }
                //encode data length
                let data_len = (asm.offset() - 1)/8 + 1;
                self.data[1] = (data_len >> 8) as u8;
                self.data[2] = (data_len & 0xff) as u8;
                //encode crc
                let mut crc = CRC::crc24lte_a();
                crc.digest(&self.data[..data_len+3]);
                let crc = crc.get_crc();
                self.data[data_len+3] = ((crc >> 16) & 0xff) as u8;
                self.data[data_len+4] = ((crc >> 8) & 0xff) as u8;
                self.data[data_len+5] = (crc & 0xff) as u8;

                Ok(&self.data[..data_len+6])
            }
            
            #[cfg(feature = "test_gen")]
            pub fn build_generated_message<FR,LR,RR>(&mut self, val_gen:&mut ValGen<FR,LR,RR>, message_number:u16) -> Result<&[u8],RtcmError>
            where FR:rand::Rng, LR:rand::Rng, RR:rand::Rng {
                if self.has_run {
                    self.clear_data();
                }
                self.has_run = true;
                let mut asm = Assembler::new(&mut self.data[3..1026], 0);
                asm.put::<U16>(message_number,12)?;

                match message_number {
                    $(
                        #[cfg(feature = $feature)]
                        $enum_pv => {
                            $msg_id::generate(&mut asm, val_gen)?;
                        }
                    )*
                    _ => return Err(RtcmError::EncodingNotSupported)
                }
                //encode data length
                let data_len = (asm.offset() - 1)/8 + 1;
                self.data[1] = (data_len >> 8) as u8;
                self.data[2] = (data_len & 0xff) as u8;
                //encode crc
                let mut crc = CRC::crc24lte_a();
                crc.digest(&self.data[..data_len+3]);
                let crc = crc.get_crc();
                self.data[data_len+3] = ((crc >> 16) & 0xff) as u8;
                self.data[data_len+4] = ((crc >> 8) & 0xff) as u8;
                self.data[data_len+5] = (crc & 0xff) as u8;

                Ok(&self.data[..data_len+6])
            }
            fn clear_data(&mut self) {                
                for d in self.data[1..].iter_mut() {
                    *d = 0;
                }                
            }
        }
    };
}

message!(
    "msg1001": Msg1001(msg1001) = 1001,
    "msg1005": Msg1005(msg1005) = 1005,
    "msg1007": Msg1007(msg1007) = 1007,
    "msg1008": Msg1008(msg1008) = 1008,
    "msg1030": Msg1030(msg1030) = 1030,
    "msg1071": Msg1071(msg1071) = 1071,
    "msg1074": Msg1074(msg1074) = 1074,
    "msg1084": Msg1084(msg1084) = 1084,
    "msg1094": Msg1094(msg1094) = 1094,
    "msg1104": Msg1104(msg1104) = 1104,
    "msg1114": Msg1114(msg1114) = 1114,
    "msg1124": Msg1124(msg1124) = 1124
);
