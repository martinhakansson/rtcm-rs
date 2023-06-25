#[non_exhaustive]
#[derive(Debug)]
pub enum RtcmError {
    NotValid,
    Incomplete,
    BufferOverflow,
    CapacityExceeded,
    EncodingNotSupported,
    DuplicateSatellite,
    InvalidSatelliteId,
    InvalidSignalId,
    SatelliteMismatch,
    DuplicateSatelliteSignal,
    InvalidSatelliteSignalCount,
    OutOfRange,
}

impl core::fmt::Display for RtcmError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            RtcmError::NotValid => write!(f, "Provided slice does not begin with a valid RTCM message"),
            RtcmError::Incomplete => write!(f, "Provided slice does not contain a complete RTCM message"),
            RtcmError::BufferOverflow => write!(f, "Attempted to read or write outside the buffer boundaries"),
            RtcmError::CapacityExceeded => write!(f, "Attempted to exceed the vector's capacity"),
            RtcmError::EncodingNotSupported => write!(f, "Attempted to encode empty or corrupt RTCM message"),
            RtcmError::DuplicateSatellite => write!(f, "The same satellite appears twice in the vector"),
            RtcmError::InvalidSatelliteId => write!(f, "Satellite ID is outside of its valid range"),
            RtcmError::InvalidSignalId => write!(f, "Signal ID is not valid"),
            RtcmError::SatelliteMismatch => write!(f, "Mismatch between the satellites in the satellite data vector and the satellites in the signal data vector"),
            RtcmError::DuplicateSatelliteSignal => write!(f, "The same satellite-signal combination appears twice in the vector"),
            RtcmError::InvalidSatelliteSignalCount => write!(f, "The product of total number of satellites and total number of signals exceeds 64 or equals 0"),
            RtcmError::OutOfRange => write!(f, "Value falls outside of range of valid values"),
            //_ => unreachable!(),
        }
    }
}
//should Error implementation be hidden behind a std feature guard?