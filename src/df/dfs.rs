//Suffixes used in df field names:
//id - identification number
//ms - unit millisecond
//flag - flag, on or off for something
//bitval - bit value, i.e. bits cast as unsigned integer
//len - length, i.e. number of elements
//ind - indicator, values with specific meaning
//m - unit meter
//str - some string
//no - number
//dbhz - unit dBHz

//DF001 (1 bit1)
//field_name: reserved_[start bit]_[bit length]
df!(
    id: df001_1bit,
    dt: u8,
    it: U8,
    len: 1,
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
//field_name: reference_station_id
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
    it: U32,
    len: 20,
    res: 0.0005,
    round: true,
    inv: 0x80000,
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

//DF021: Reserved for ITRF Realization year
//field_name: reserved_itrf_real_year

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
//field_name: antenna_ref_point_ecef_x_m
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
//field_name: antenna_ref_point_ecef_y_m
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
//field_name: antenna_ref_point_ecef_z_m
df!(
    id: df027,
    dt: f64,
    it: I64,
    len: 38,
    res: 0.0001,
    round: true,
    ord: 0,
);

//DF029: Descriptor Counter
//field_name: antenna_desc_char_len
df!(
    id: df029,
    dt: usize,
    it: U8,
    len: 8,
    cap: 31,DF029_CAP,
    ord: 0,
);

//DF030: Antenna Descriptor
//field_name: antenna_descriptor_str
df_88591_string!(id: df030, cap_id: df029,);

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
//field_name: serial_number_len
df!(
    id: df032,
    dt: usize,
    it: U8,
    len: 8,
    cap: 31,DF032_CAP,
    ord: 0,
);

//DF033: Antenna Serial Number
//field_name: antenna_serial_number_str
df_88591_string!(id: df033, cap_id: df032,);

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
//field_name: s_od_mm
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
//field_name: s_oh_mm
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
//field_name: s_id_mm
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
//field_name: n_refs_no
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

//DF400: GNSS Signal fine Pseudorange
//field_name: gnss_signal_fine_pseudorange_ms
df!(
    id: df400,
    dt: f64,
    it: I16,
    len: 15,
    res: 1.0 / (16.0 * 1024.0 * 1024.0),
    round: true,
    inv: 0x4000,
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
    inv: 0x200000,
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

//DF409: IODS issue of data station
//field_name: reserved_issue_of_data_station
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

//DF420: Half-cycle ambiguity indicator
//field_name: half_cycle_ambiguity_ind
df!(
    id: df420,
    dt: u8,
    it: U8,
    len: 1,
    ord: 0,
);