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
    "msg1002": Msg1002(msg1002) = 1002,
    "msg1003": Msg1003(msg1003) = 1003,
    "msg1004": Msg1004(msg1004) = 1004,
    "msg1005": Msg1005(msg1005) = 1005,
    "msg1006": Msg1006(msg1006) = 1006,
    "msg1007": Msg1007(msg1007) = 1007,
    "msg1008": Msg1008(msg1008) = 1008,
    "msg1009": Msg1009(msg1009) = 1009,
    "msg1010": Msg1010(msg1010) = 1010,
    "msg1011": Msg1011(msg1011) = 1011,
    "msg1012": Msg1012(msg1012) = 1012,
    "msg1013": Msg1013(msg1013) = 1013,
    "msg1014": Msg1014(msg1014) = 1014,
    "msg1015": Msg1015(msg1015) = 1015,
    "msg1016": Msg1016(msg1016) = 1016,
    "msg1017": Msg1017(msg1017) = 1017,
    "msg1019": Msg1019(msg1019) = 1019,
    "msg1020": Msg1020(msg1020) = 1020,
    "msg1021": Msg1021(msg1021) = 1021,
    "msg1022": Msg1022(msg1022) = 1022,
    "msg1023": Msg1023(msg1023) = 1023,
    "msg1024": Msg1024(msg1024) = 1024,
    "msg1025": Msg1025(msg1025) = 1025,
    "msg1026": Msg1026(msg1026) = 1026,
    "msg1027": Msg1027(msg1027) = 1027,
    "msg1029": Msg1029(msg1029) = 1029,
    "msg1030": Msg1030(msg1030) = 1030,
    "msg1031": Msg1031(msg1031) = 1031,
    "msg1032": Msg1032(msg1032) = 1032,
    "msg1033": Msg1033(msg1033) = 1033,
    "msg1034": Msg1034(msg1034) = 1034,
    "msg1035": Msg1035(msg1035) = 1035,
    "msg1037": Msg1037(msg1037) = 1037,
    "msg1038": Msg1038(msg1038) = 1038,
    "msg1039": Msg1039(msg1039) = 1039,
    "msg1041": Msg1041(msg1041) = 1041,
    "msg1042": Msg1042(msg1042) = 1042,
    "msg1044": Msg1044(msg1044) = 1044,
    "msg1045": Msg1045(msg1045) = 1045,
    "msg1046": Msg1046(msg1046) = 1046,
    "msg1057": Msg1057(msg1057) = 1057,
    "msg1058": Msg1058(msg1058) = 1058,
    "msg1059": Msg1059(msg1059) = 1059,
    "msg1060": Msg1060(msg1060) = 1060,
    "msg1061": Msg1061(msg1061) = 1061,
    "msg1062": Msg1062(msg1062) = 1062,
    "msg1063": Msg1063(msg1063) = 1063,
    "msg1064": Msg1064(msg1064) = 1064,
    "msg1065": Msg1065(msg1065) = 1065,
    "msg1066": Msg1066(msg1066) = 1066,
    "msg1067": Msg1067(msg1067) = 1067,
    "msg1068": Msg1068(msg1068) = 1068,
    "msg1071": Msg1071(msg1071) = 1071,
    "msg1072": Msg1072(msg1072) = 1072,
    "msg1073": Msg1073(msg1073) = 1073,
    "msg1074": Msg1074(msg1074) = 1074,
    "msg1075": Msg1075(msg1075) = 1075,
    "msg1076": Msg1076(msg1076) = 1076,
    "msg1077": Msg1077(msg1077) = 1077,
    "msg1081": Msg1081(msg1081) = 1081,
    "msg1082": Msg1082(msg1082) = 1082,
    "msg1083": Msg1083(msg1083) = 1083,
    "msg1084": Msg1084(msg1084) = 1084,
    "msg1085": Msg1085(msg1085) = 1085,
    "msg1086": Msg1086(msg1086) = 1086,
    "msg1087": Msg1087(msg1087) = 1087,
    "msg1091": Msg1091(msg1091) = 1091,
    "msg1092": Msg1092(msg1092) = 1092,
    "msg1093": Msg1093(msg1093) = 1093,
    "msg1094": Msg1094(msg1094) = 1094,
    "msg1095": Msg1095(msg1095) = 1095,
    "msg1096": Msg1096(msg1096) = 1096,
    "msg1097": Msg1097(msg1097) = 1097,
    "msg1101": Msg1101(msg1101) = 1101,
    "msg1102": Msg1102(msg1102) = 1102,
    "msg1103": Msg1103(msg1103) = 1103,
    "msg1104": Msg1104(msg1104) = 1104,
    "msg1105": Msg1105(msg1105) = 1105,
    "msg1106": Msg1106(msg1106) = 1106,
    "msg1107": Msg1107(msg1107) = 1107,
    "msg1111": Msg1111(msg1111) = 1111,
    "msg1112": Msg1112(msg1112) = 1112,
    "msg1113": Msg1113(msg1113) = 1113,
    "msg1114": Msg1114(msg1114) = 1114,
    "msg1115": Msg1115(msg1115) = 1115,
    "msg1116": Msg1116(msg1116) = 1116,
    "msg1117": Msg1117(msg1117) = 1117,
    "msg1121": Msg1121(msg1121) = 1121,
    "msg1122": Msg1122(msg1122) = 1122,
    "msg1123": Msg1123(msg1123) = 1123,
    "msg1124": Msg1124(msg1124) = 1124,
    "msg1125": Msg1125(msg1125) = 1125,
    "msg1126": Msg1126(msg1126) = 1126,
    "msg1127": Msg1127(msg1127) = 1127,
    "msg1230": Msg1230(msg1230) = 1230
);
