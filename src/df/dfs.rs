//Suffixes used in df field names:
//id - ID
//ms - unit millisecond
//flag - flag, on or off for something
//bitval - df bits cast as unsigned integer
//len - length, i.e. number of elements in message
//ind - indicator, values with specific meaning
//m - unit meter
//mm - unit millimeter
//ppm - parts per million
//s - unit second
//ms - unit millisecond
//str - string
//n - number of
//dbhz - unit dBHz
//m_s - unit m/s

//DF001 (1 bit1)
//field_name: reserved_[start bit]_[bit length]
df!(
    id: df001_1bit,
    dt: u8,
    it: U8,
    len: 1,
    ord: 0,
);

//DF001 (1 bit1)
//field_name: reserved_[start bit]_[bit length]
df!(
    id: df001_4bits,
    dt: u8,
    it: U8,
    len: 4,
    ord: 0,
);

//DF001 (6 bits)
//field_name: reserved_[start bit]_[bit length]
df!(
    id: df001_6bits,
    dt: u8,
    it: U8,
    len: 6,
    ord: 0,
);

//DF001 (7 bits)
//field_name: reserved_[start bit]_[bit length]
df!(
    id: df001_7bits,
    dt: u8,
    it: U8,
    len: 7,
    ord: 0,
);

//DF002: Message number
df!(
    id: df002,
    dt: u16,
    it: U16,
    len: 12,
    ord: 0,
);

//DF003: Reference Station ID
//field_name: reference_station_id or non_physical_reference_station_id
df!(
    id: df003,
    dt: u16,
    it: U16,
    len: 12,
    ord: 0,
);

//DF004: GPS Epoch Time (TOW) in ms
//field_name: gps_epoch_time_ms
df!(
    id: df004,
    dt: u32,
    it: U32,
    len: 30,
    ord: 0,
);

//DF005: Synchronous GNSS Message Flag
//field_name: synchronous_gnss_msg_flag
df!(
    id: df005,
    dt: u8,
    it: U8,
    len: 1,
    ord: 0,
);

//DF006: No. of GPS Satellite Signals Processed
//field_name: satellites_len
df!(
    id: df006,
    dt: usize,
    it: U8,
    len: 5,
    cap: 31,DF006_CAP,
    ord: 0,
);

//DF007: GPS Divergence-free Smoothing Indicator
//field_name: gps_divergence_free_smoothing_flag
df!(
    id: df007,
    dt: u8,
    it: U8,
    len: 1,
    ord: 0,
);

//DF008: GPS Smoothing Interval
//field_name: gps_smoothing_interval_bitval
df!(
    id: df008,
    dt: u8,
    it: U8,
    len: 3,
    ord: 0,
);

//DF009: GPS Satellite ID
//field_name: gps_satellite_id
df!(
    id: df009,
    dt: u8,
    it: U8,
    len: 6,
    ord: 0,
);

//DF010: GPS L1 Code Indicator
//field_name: gps_l1_code_ind
df!(
    id: df010,
    dt: u8,
    it: U8,
    len: 1,
    ord: 0,
);

//DF011: GPS L1 Pseudorange
//field_name: gps_l1_pseudorange_m
df!(
    id: df011,
    dt: f64,
    it: U32,
    len: 24,
    res: 0.02,
    round: true,
    inv: 0xffffff,
);

//DF012: GPS L1 Phaserange - L1 Pseudorange
//field_name: gps_l1_phase_pseudorange_diff_m
df!(
    id: df012,
    dt: f64,
    it: I32,
    len: 20,
    res: 0.0005,
    round: true,
    inv: -0x80000,
);

//DF013: GPS L1 Lock Time Indicator
//field_name: gps_l1_lock_time_bitval
df!(
    id: df013,
    dt: u8,
    it: U8,
    len: 7,
    ord: 0,
);

//DF022: GPS Indicator
//field_name: gps_flag
df!(
    id: df022,
    dt: u8,
    it: U8,
    len: 1,
    ord: 0,
);

//DF023: GLONASS Indicator
//field_name: glonass_flag
df!(
    id: df023,
    dt: u8,
    it: U8,
    len: 1,
    ord: 0,
);

//DF024: Galileo Indicator
//field_name: galileo_flag
df!(
    id: df024,
    dt: u8,
    it: U8,
    len: 1,
    ord: 0,
);

//DF025: Antenna Ref. Point ECEF-X
//field_name: antenna_ref_point_ecef_x_m or phys_ref_arp_ecef_x_m
df!(
    id: df025,
    dt: f64,
    it: I64,
    len: 38,
    res: 0.0001,
    round: true,
    ord: 0,
);

//DF026: Antenna Ref. Point ECEF-Y
//field_name: antenna_ref_point_ecef_y_m or phys_ref_arp_ecef_y_m
df!(
    id: df026,
    dt: f64,
    it: I64,
    len: 38,
    res: 0.0001,
    round: true,
    ord: 0,
);

//DF027: Antenna Ref. Point ECEF-Z
//field_name: antenna_ref_point_ecef_z_m or phys_ref_arp_ecef_z_m
df!(
    id: df027,
    dt: f64,
    it: I64,
    len: 38,
    res: 0.0001,
    round: true,
    ord: 0,
);

//DF028: Antenna Height
//field_name: antenna_height_m
df!(
    id: df028,
    dt: f64,
    it: U16,
    len: 16,
    res: 0.0001,
    round: true,
    ord: 0,
);

//DF029: Descriptor Counter
//field_name: antenna_desc_char_len (refactor as antenna_descriptor_len)
//refactor as uses df_desc_str_len, se below
// df!(
//     id: df029,
//     dt: usize,
//     it: U8,
//     len: 8,
//     cap: 31,DF029_CAP,
//     ord: 0,
// );

//DF030: Antenna Descriptor
//field_name: antenna_descriptor_str
//refactor as uses df_desc_str, se below
//df_88591_string!(id: df030, cap_id: df029,);

//DF031: Antenna Setup ID
//field_name: antenna_setup_id
df!(
    id: df031,
    dt: u8,
    it: U8,
    len: 8,
    ord: 0,
);

//DF032: Serial Number Counter
//field_name: serial_number_len (refactor as antenna_serial_number_len)
//refactor as uses df_desc_str_len, se below
// df!(
//     id: df032,
//     dt: usize,
//     it: U8,
//     len: 8,
//     cap: 31,DF032_CAP,
//     ord: 0,
// );

//DF033: Antenna Serial Number
//field_name: antenna_serial_number_str
//refactor as uses df_desc_str, se below
//df_88591_string!(id: df033, cap_id: df032,);

//DF034: GLONASS Epoch Time
//field_name: glo_epoch_time_ms
df!(
    id: df034,
    dt: u32,
    it: U32,
    len: 27,
    ord: 0,
);

//DF141: Reference-Station Indicator
//field_name: reference_station_ind
df!(
    id: df141,
    dt: u8,
    it: U8,
    len: 1,
    ord: 0,
);

//DF142: Single Receiver Oscillator Indicator
//field_name: single_receiver_osc_ind
df!(
    id: df142,
    dt: u8,
    it: U8,
    len: 1,
    ord: 0,
);

//DF218: Constant term of non-dispersive standard deviation (s_oc) in mm
//field_name: s_oc_mm
df!(
    id: df218,
    dt: f32,
    it: U8,
    len: 8,
    res: 0.5,
    round: true,
    ord: 0,
);

//DF219: Distance dependent term of non-dispersive standard deviation (s_od) in ppm
//field_name: s_od_ppm
df!(
    id: df219,
    dt: f32,
    it: U16,
    len: 9,
    res: 0.01,
    round: true,
    ord: 0,
);

//DF220: Height dependent term of non-dispersive standard deviation (s_oh) in ppm
//field_name: s_oh_ppm
df!(
    id: df220,
    dt: f32,
    it: U8,
    len: 6,
    res: 0.1,
    round: true,
    ord: 0,
);

//DF221: Constant term of dispersive (as affecting GPS L1) standard deviation (s_ic) in mm
//field_name: s_ic_mm
df!(
    id: df221,
    dt: f32,
    it: U16,
    len: 10,
    res: 0.5,
    round: true,
    ord: 0,
);

//DF222: Distance dependent term of dispersive standard deviation (s_id) in ppm
//field_name: s_id_ppm
df!(
    id: df222,
    dt: f32,
    it: U16,
    len: 10,
    res: 0.01,
    round: true,
    ord: 0,
);

//DF223: Number of reference stations used to derive residual statistics
//field_name: refs_n
df!(
    id: df223,
    dt: u8,
    it: U8,
    len: 7,
    ord: 0,
);

//DF224: GPS Residuals Epoch Time (s)
//field_name: gps_resid_epoch_time_s
df!(
    id: df224,
    dt: u32,
    it: U32,
    len: 20,
    ord: 0,
);

//DF226: Physical Reference Station ID
//field_name: physical_reference_station_id
//df003 used as it is identical

//DF227: Receiver Type Descriptor Counter
//field_name: receiver_type_descriptor_len
//uses df_desc_str_len, se below

//DF228: Receiver Type Descriptor
//field_name: receiver_type_descriptor_str
//uses df_desc_str, se below

//DF229: Receiver Firmware Version Counter
//field_name: receiver_firmware_version_len
//uses df_desc_str_len, se below

//DF230: Receiver Firmware Version
//field_name: receiver_firmware_version_str
//uses df_desc_str, se below

//DF231: Receiver Serial Number Counter
//field_name: receiver_serial_number_len
//uses df_desc_str_len, se below

//DF232: Receiver Serial Number
//field_name: receiver_serial_number_str
//uses df_desc_str, se below

//DF248: Galileo Epoch Time
//field_name: gal_epoch_time_ms
df!(
    id: df248,
    dt: u32,
    it: U32,
    len: 30,
    ord: 0,
);

//DF364: Quarter Cycle Indicator
//field_name: quarter_cycle_ind
df!(
    id: df364,
    dt: u8,
    it: U8,
    len: 2,
    ord: 0,
);

//DF393: MSM multiple message bit
//field_name: msm_multiple_message_flag
df!(
    id: df393,
    dt: u8,
    it: U8,
    len: 1,
    ord: 0,
);

//DF394, DF395, DF396 handled in msm specific macros in msg module

//DF397: Integer Milliseconds GNSS rough range
//field_name: gnss_satellite_rough_range_integer_ms
df!(
    id: df397,
    dt: u8,
    it: U8,
    len: 8,
    inv: 255,
);

//DF398: GNSS Satellite rough range modulo 1 millisecond
//field_name: gnss_satellite_rough_range_mod1ms_ms
df!(
    id: df398,
    dt: f64,
    it: U16,
    len: 10,
    res: 1.0 / 1024.0,
    round: true,
    ord: 0,
);

//DF399: GNSS Satellite rough Phaserange rates
//field_name: gnss_satellite_rough_phaserange_rates_m_s
df!(
    id: df399,
    dt: i16,
    it: I16,
    len: 14,
    inv: -0x2000,
);

//DF400: GNSS Signal fine Pseudorange
//field_name: gnss_signal_fine_pseudorange_ms
df!(
    id: df400,
    dt: f64,
    it: I16,
    len: 15,
    res: 1.0 / (16.0 * 1024.0 * 1024.0),
    round: true,
    inv: -0x4000,
);

//DF401: GNSS Signal fine phaserange
//field_name: gnss_signal_fine_phaserange_ms
df!(
    id: df401,
    dt: f64,
    it: I32,
    len: 22,
    res: 1.0 / (512.0 * 1024.0 * 1024.0),
    round: true,
    inv: -0x200000,
);

//DF402: GNSS Phaserange Lock Time Indicator
//field_name: gnss_phaserange_lock_time_ind
df!(
    id: df402,
    dt: u8,
    it: U8,
    len: 4,
    ord: 0,
);

//DF403: GNSS Signal CNR
//field_name: gnss_signal_cnr_dbhz
df!(
    id: df403,
    dt: u8,
    it: U8,
    len: 6,
    inv: 0,
);

//DF404: GNSS signal fine Phaserange Rate
//field_name: gnss_signal_fine_phaserange_rate_m_s
df!(
    id: df404,
    dt: f64,
    it: I16,
    len: 15,
    res: 0.0001,
    round: true,
    inv: -0x4000,
);

//DF405: GNSS signal fine Pseudorange with extended resolution
//field_name: gnss_signal_fine_pseudorange_ext_ms
df!(
    id: df405,
    dt: f64,
    it: I32,
    len: 20,
    res: 1.0 / (512.0 * 1024.0 * 1024.0),
    round: true,
    inv: -0x80000,
);

//DF406: GNSS signal fine phaserange with extended resolution
//field_name: gnss_signal_fine_phaserange_ext_ms
df!(
    id: df406,
    dt: f64,
    it: I32,
    len: 24,
    res: 1.0 / (2.0 * 1024.0 * 1024.0 * 1024.0),
    round: true,
    inv: -0x800000,
);

//DF407: GNSS Phaserange Lock Time Indicator with extended range and resolution
//field_name: gnss_phaserange_lock_time_ext_ind
df!(
    id: df407,
    dt: u16,
    it: U16,
    len: 10,
    ord: 0,
);

//DF408: GNSS signal CNR with extended resolution
//field_name: gnss_signal_cnr_ext_dbhz
df!(
    id: df408,
    dt: f64,
    it: U16,
    len: 10,
    res: 1.0 / 16.0,
    inv: 0,
);

//DF409: IODS issue of data station
//field_name: issue_of_data_station
df!(
    id: df409,
    dt: u8,
    it: U8,
    len: 3,
    inv: 0,
);

//DF411: Clock steering indicator
//field_name: clock_steering_ind
df!(
    id: df411,
    dt: u8,
    it: U8,
    len: 2,
    ord: 0,
);

//DF412: External Clock Indicator
//field_name: external_clock_ind
df!(
    id: df412,
    dt: u8,
    it: U8,
    len: 2,
    ord: 0,
);

//DF416: GLONASS Day Of Week
//field_name: glo_day_of_week
df!(
    id: df416,
    dt: u8,
    it: U8,
    len: 3,
    inv: 7,
);

//DF417: GNSS smoothing type indicator
//field_name: gnss_smoothing_type_ind
df!(
    id: df417,
    dt: u8,
    it: U8,
    len: 1,
    ord: 0,
);

//DF418: GNSS smoothing interval
//field_name: gnss_smoothing_interval_bitval
df!(
    id: df418,
    dt: u8,
    it: U8,
    len: 3,
    ord: 0,
);

//DF419: GLONASS Satellite Frequency Channel Number
//field_name: glonass_satellite_frequency_channel_number
df!(
    id: df419,
    dt: i8,
    it: U8,
    len: 4,
    bias: -7,
    inv: 15,
);

//DF420: Half-cycle ambiguity indicator
//field_name: half_cycle_ambiguity_ind
df!(
    id: df420,
    dt: u8,
    it: U8,
    len: 1,
    ord: 0,
);

//DF427: BeiDou Epoch Time
//field_name: bds_epoch_time_ms
df!(
    id: df427,
    dt: u32,
    it: U32,
    len: 30,
    ord: 0,
);

//DF428: QZSS Epoch Time
//field_name: qzss_epoch_time_ms
df!(
    id: df428,
    dt: u32,
    it: U32,
    len: 30,
    ord: 0,
);

//Reusable data fields

//Length of descriptor strings for data fields: DF029, DF032, DF227, DF229, DF231
df!(
    id: df_desc_str_len,
    dt: usize,
    it: U8,
    len: 8,
    cap: 31,DESC_STR_CAP,
    ord: 0,
);

//Descriptor strings for data fields: DF030, DF033, DF228, DF230, DF232
df_88591_string!(id: df_desc_str, cap_name: DESC_STR_CAP,);