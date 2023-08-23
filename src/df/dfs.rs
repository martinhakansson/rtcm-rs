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
//sc - unit semi-circles
//sc_s - unit semi-circles/s
//rad - unit radian
//sqrt_m - unit sqrt(meter)
//h - unit hour
//min - unit minute
//d - unit day

//DF001 (1 bit)
//field_name: reserved_[start bit]_[bit length]
df!(
    id: df001_1bit,
    dt: u8,
    it: U8,
    len: 1,
    ord: 0,
);

//DF001 (2 bits)
//field_name: reserved_[start bit]_[bit length]
df!(
    id: df001_2bits,
    dt: u8,
    it: U8,
    len: 2,
    ord: 0,
);

//DF001 (4 bits)
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

//DF001 (10 bits)
//field_name: reserved_[start bit]_[bit length]
df!(
    id: df001_10bits,
    dt: u16,
    it: U16,
    len: 10,
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

//DF038: GLONASS Satellite ID
//field_name: glo_satellite_id
df!(
    id: df038,
    dt: u8,
    it: U8,
    len: 6,
    ord: 0,
);

//DF040: GLONASS Satellite Frequency Channel Number
//field_name: glo_satellite_freq_chan_number
df!(
    id: df040,
    dt: i8,
    it: U8,
    len: 5,
    bias: -7,
    ord: 0,
);

//DF051: Modified Julian Day (MJD) Number
//field_name: modified_julian_day_number
df!(
    id: df051,
    dt: u16,
    it: U16,
    len: 16,
    ord: 0,
);

//DF052: Seconds of Day (UTC)
//field_name: seconds_of_day_s
df!(
    id: df052,
    dt: u32,
    it: U32,
    len: 17,
    ord: 0,
);

//DF053: Number of Message ID Announcements to Follow
//field_name: message_id_announcements_len
df!(
    id: df053,
    dt: usize,
    it: U8,
    len: 5,
    cap: 31, DF053_CAP,
    ord: 0,
);

//DF054: Leap Seconds GPS-UTC
//field_name: leap_seconds_gps_utc_s
df!(
    id: df054,
    dt: u8,
    it: U8,
    len: 8,
    inv: 255,
);

//DF055: Message ID
//field_name: message_id
//df002 also used for DF055

//DF056: Message Sync Flag
//field_name: message_sync_flag
df!(
    id: df056,
    dt: u8,
    it: U8,
    len: 1,
    ord: 0,
);

//DF057: Message Transmission Interval
//field_name: message_transmission_interval_s
df!(
    id: df057,
    dt: f32,
    it: U16,
    len: 16,
    res: 0.1,
    round: true,
    inv: 0,
);

//DF068: GPS Satellite ID
//field_name: gps_satellite_id (same name as df009)
//df: df_u6

//DF071: GPS IODE
//field_name: iode
df!(
    id: df071,
    dt: u8,
    it: U8,
    len: 8,
    ord: 0,
);

//DF076: GPS Week number
//field_name: gps_week_number
df!(
    id: df076,
    dt: u16,
    it: U16,
    len: 10,
    ord: 0,
);

//DF077: GPS SV Acc. URA
//field_name: ura_index
df!(
    id: df077,
    dt: u8,
    it: U8,
    len: 4,
    ord: 0,
);

//DF078: GPS Code on L2
//field_name: code_on_l2_ind
df!(
    id: df078,
    dt: u8,
    it: U8,
    len: 2,
    ord: 0,
);

//DF079: GPS IDOT
//field_name: idot_sc_s
df!(
    id: df079,
    dt: f64,
    it: I16,
    len: 14,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*8.0),
    round: true,
    ord: 0,
);

//DF080: Reserved

//DF081: GPS toc
//field_name: toc_s
df!(
    id: df081,
    dt: f32,
    it: U16,
    len: 16,
    res: 16.0,
    round: true,
    ord: 0,
);

//DF082: GPS af2
//field_name: af2_s_s2
df!(
    id: df082,
    dt: f32,
    it: I8,
    len: 8,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*1024.0*32.0),
    round: true,
    ord: 0,
);

//DF083: GPS af1
//field_name: af1_s_s
df!(
    id: df083,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*8.0),
    round: true,
    ord: 0,
);

//DF084: GPS af0
//field_name: af0_s
df!(
    id: df084,
    dt: f64,
    it: I32,
    len: 22,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF085: GPS IODC
//field_name: iodc
df!(
    id: df085,
    dt: u16,
    it: U16,
    len: 10,
    ord: 0,
);

//DF086: GPS crs
//field_name: crs_m
df!(
    id: df086,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/32.0,
    round: true,
    ord: 0,
);

//DF087: GPS DELTA n
//field_name. delta_n_sc_s
df!(
    id: df087,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*8.0),
    round: true,
    ord: 0,
);

//DF088: GPS M0
//field_name: m0_sc
df!(
    id: df088,
    dt: f64,
    it: I32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF089: GPS cuc
//field_name: cuc_rad
df!(
    id: df089,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/(1024.0*1024.0*512.0),
    round: true,
    ord: 0,
);

//DF090: GPS Eccentricity
//field_name: eccentricity
df!(
    id: df090,
    dt: f64,
    it: U32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*8.0),
    round: true,
    ord: 0,
);

//DF091: GPS cus
//field_name: cus_rad
df!(
    id: df091,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/(1024.0*1024.0*512.0),
    round: true,
    ord: 0,
);

//DF092: GPS sqrt A
//field_name: sqrt_a_sqrt_m
df!(
    id: df092,
    dt: f64,
    it: U32,
    len: 32,
    res: 1.0/(1024.0*512.0),
    round: true,
    ord: 0,
);

//DF093: GPS toe
//field_name toe_s
df!(
    id: df093,
    dt: f32,
    it: U16,
    len: 16,
    res: 16.0,
    round: true,
    ord: 0,
);

//DF094: GPS cic
//field_name: cic_rad
df!(
    id: df094,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/(1024.0*1024.0*512.0),
    round: true,
    ord: 0,
);

//DF095: GPS OMEGA0
//field_name: omega0_sc
df!(
    id: df095,
    dt: f64,
    it: I32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF096: GPS cis
//field_name: cis_rad
df!(
    id: df096,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/(1024.0*1024.0*512.0),
    round: true,
    ord: 0,
);

//DF097: GPS i0
//field_name: i0_sc
df!(
    id: df097,
    dt: f64,
    it: I32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF098: GPS crc
//field_name: crc_m
df!(
    id: df098,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/32.0,
    round: true,
    ord: 0,
);

//DF099: GPS omega
//field_name: omega_sc
df!(
    id: df099,
    dt: f64,
    it: I32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF100: GPS OMEGADOT
//field_name: omegadot_sc_s
df!(
    id: df100,
    dt: f64,
    it: I32,
    len: 24,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*8.0),
    round: true,
    ord: 0,
);

//DF101: GPS tgd
//field_name: tgd_s
df!(
    id: df101,
    dt: f32,
    it: I8,
    len: 8,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF102: GPS SV HEALTH
//field_name: sv_health_ind
df!(
    id: df102,
    dt: u8,
    it: U8,
    len: 6,
    ord: 0,
);

//DF103: GPS L2 P data flag
//field_name: l2_p_data_flag (0 = on, 1 = off)
df!(
    id: df103,
    dt: u8,
    it: U8,
    len: 1,
    ord: 0,
);

//DF104: GLONASS almanac health
//field_name: glo_alm_health_flag
//df: df_flag

//DF105: GLONASS almanac health availability indicator
//field_name: glo_alm_health_avail_flag
//df: df_flag

//DF106: GLONASS P1
//field_name: p1_ind
df!(
    id: df106,
    dt: u8,
    it: U8,
    len: 2,
    ord: 0,
);

//DF107: GLONASS tk
//field_name: tk_h
df!(
    id: df107_hour,
    dt: u8,
    it: U8,
    len: 5,
    ord: 0,
);
//field_name: tk_min
df!(
    id: df107_min,
    dt: u8,
    it: U8,
    len: 6,
    ord: 0,
);
//field_name: tk_s
df!(
    id: df107_s,
    dt: u8,
    it: U8,    
    len: 1,
    res: 30,
    ord: 0,
);

//DF108: GLONASS Msb of bn
//field_name: glo_eph_health_flag
//df: df_flag

//DF109: GLONASS P2
//field_name: p2_flag
//df: df_flag

//DF110: GLONASS tb
//field_name: tb_min
df!(
    id: df110,
    dt: u16,
    it: U8,    
    len: 7,
    res: 15,
    ord: 0,
);

//DF111: GLONASS xn(tb), first derivative
//field_name: xn_first_deriv_km_s
df!(
    id: df111,
    dt: f64,
    it: SM32,
    len: 24,
    res: 1.0/(1024.0*1024.0),
    ord: 0,
);

//DF112: GLONASS xn(tb)
//field_name: xn_km
df!(
    id: df112,
    dt: f64,
    it: SM32,
    len: 27,
    res: 1.0/(1024.0*2.0),
    ord: 0,
);

//DF113: GLONASS xn(tb), second derivative
//field_name: xn_second_deriv_km_s2
df!(
    id: df113,
    dt: f32,
    it: SM8,
    len: 5,
    res: 1.0/(1024.0*1024.0*1024.0),
    ord: 0,
);

//DF114: GLONASS yn(tb), first derivative
//field_name: yn_first_deriv_km_s
df!(
    id: df114,
    dt: f64,
    it: SM32,
    len: 24,
    res: 1.0/(1024.0*1024.0),
    ord: 0,
);

//DF115: GLONASS yn(tb)
//field_name: yn_km
df!(
    id: df115,
    dt: f64,
    it: SM32,
    len: 27,
    res: 1.0/(1024.0*2.0),
    ord: 0,
);

//DF116: GLONASS yn(tb), second derivative
//field_name: yn_second_deriv_km_s2
df!(
    id: df116,
    dt: f32,
    it: SM8,
    len: 5,
    res: 1.0/(1024.0*1024.0*1024.0),
    ord: 0,
);

//DF117: GLONASS zn(tb), first derivative
//field_name: zn_first_deriv_km_s
df!(
    id: df117,
    dt: f64,
    it: SM32,
    len: 24,
    res: 1.0/(1024.0*1024.0),
    ord: 0,
);

//DF118: GLONASS zn(tb)
//field_name: zn_km
df!(
    id: df118,
    dt: f64,
    it: SM32,
    len: 27,
    res: 1.0/(1024.0*2.0),
    ord: 0,
);

//DF119: GLONASS zn(tb), second derivative
//field_name: zn_second_deriv_km_s2
df!(
    id: df119,
    dt: f32,
    it: SM8,
    len: 5,
    res: 1.0/(1024.0*1024.0*1024.0),
    ord: 0,
);

//DF120: GLONASS P3
//field_name: p3_flag
//df: df_flag

//DF121: GLONASS gamma_n
//field_name: gamma_n
df!(
    id: df121,
    dt: f32,
    it: SM16,
    len: 11,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0),
    ord: 0,
);

//DF122: GLONASS-M P
//field_name: glo_m_p_ind
df!(
    id: df122,
    dt: u8,
    it: U8,
    len: 2,
    ord: 0,
);

//DF123: GLONASS-M ln (third string)
//field_name: glo_m_3str_ln_flag
//df: df_flag

//DF124: GLONASS tau_n
//field_name: tau_n_s
df!(
    id: df124,
    dt: f64,
    it: SM32,
    len: 22,
    res: 1.0/(1024.0*1024.0*1024.0),
    ord: 0,
);

//DF125: GLONASS-M Delta tau_n
//field_name: glo_m_delta_tau_n_s
df!(
    id: df125,
    dt: f32,
    it: SM8,
    len: 5,
    res: 1.0/(1024.0*1024.0*1024.0),
    ord: 0,
);

//DF126: GLONASS En
//field_name: en_d
df!(
    id: df126,
    dt: u8,
    it: U8,
    len: 5,
    ord: 0,
);

//DF127: GLONASS-M P4
//field_name: glo_m_p4_flag
//df: df_flag

//DF128: GLONASS-M FT
//field_name: glo_m_ft_ind
df!(
    id: df128,
    dt: u8,
    it: U8,
    len: 4,
    ord: 0,
);

//DF129: GLONASS-M NT
//field_name: glo_m_nt_d
df!(
    id: df129,
    dt: u16,
    it: U16,
    len: 11,
    ord: 0,
);

//DF130: GLONASS-M M
//field_name: glo_m_m_ind
df!(
    id: df130,
    dt: u8,
    it: U8,
    len: 2,
    ord: 0,
);

//DF131: GLONASS The Availability of Additional Data
//field_name: additional_data_flag
//df: df_flag

//DF132: GLONASS NA
//field_name: na_d
df!(
    id: df132,
    dt: u16,
    it: U16,
    len: 11,
    ord: 0,
);

//DF133: GLONASS tau_c
//field_name: tau_c_s
df!(
    id: df133,
    dt: f64,
    it: SM32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    ord: 0,
);

//DF134: GLONASS-M N4
//field_name: glo_m_n4_year
df!(
    id: df134,
    dt: u16,
    it: U8,
    len: 5,
    res: 4,
    bias: 1992,
    ord: 0,
);

//DF135: GLONASS-M tau GPS
//field_name: glo_m_tau_gps_s
df!(
    id: df135,
    dt: f64,
    it: SM32,
    len: 22,
    res: 1.0/(1024.0*1024.0*1024.0),
    ord: 0,
);

//DF136: GLONASS-M ln (fifth string)
//field_name: glo_m_5str_ln_flag
//df: df_flag

//DF137: GPS Fit Interval
//field_name: fit_interval_ind
//df: df_flag

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

//DF252: Galileo Satellite ID
//field_name: gal_satellite_id
df!(
    id: df252,
    dt: u8,
    it: U8,
    len: 6,
    ord: 0,
);

//DF286: Galileo SISA Index (E1,E5b)
//field_name: sisa_e1_e5b_index
df!(
    id: df286,
    dt: u8,
    it: U8,
    len: 8,
    ord: 0,
);

//DF287: Galileo E1-B Signal Health Status
//field_name: e1_b_sig_health_ind
df!(
    id: df287,
    dt: u8,
    it: U8,
    len: 2,
    ord: 0,
);

//DF288: Galileo E1-B Data Validity Status
//field_name: e1_b_data_validity_flag
//df: df_flag

//DF289: Galileo Week number
//field_name: gal_week_number
df!(
    id: df289,
    dt: u16,
    it: U16,
    len: 12,
    ord: 0,
);

//DF290: Galileo IODnav
//field_name: iodnav
df!(
    id: df290,
    dt: u16,
    it: U16,
    len: 10,
    ord: 0,
);

//DF291: Galileo SV SISA
//field_name: sisa_e1_e5a_index
df!(
    id: df291,
    dt: u8,
    it: U8,
    len: 8,
    ord: 0,
);

//DF292: Galileo IDOT
//field_name: idot_sc_s
df!(
    id: df292,
    dt: f32,
    it: I16,
    len: 14,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*8.0),
    round: true,
    ord: 0,
);

//DF293: Galileo toc
//field_name: toc_s
df!(
    id: df293,
    dt: f32,
    it: U16,
    len: 14,
    res: 60.0,
    round: true,
    ord: 0,
);

//DF294: Galileo af2
//field_name: af2_s_s2
df!(
    id: df294,
    dt: f32,
    it: I8,
    len: 6,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*1024.0*512.0),
    round: true,
    ord: 0,
);

//DF295: Galileo af1
//field_name: af1_s_s
df!(
    id: df295,
    dt: f64,
    it: I32,
    len: 21,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*64.0),
    round: true,
    ord: 0,
);

//DF296: Galileo af0
//field_name: af0_s
df!(
    id: df296,
    dt: f64,
    it: I32,
    len: 31,
    res: 1.0/(1024.0*1024.0*1024.0*16.0),
    round: true,
    ord: 0,
);

//DF297: Galileo Crs
//field_name: crs_m
df!(
    id: df297,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/32.0,
    round: true,
    ord: 0,
);

//DF298: Galileo Delta_n
//field_name: delta_n_sc_s
df!(
    id: df298,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*8.0),
    round: true,
    ord: 0,
);

//DF299: Galileo M0
//field_name: m0_sc
df!(
    id: df299,
    dt: f64,
    it: I32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF300: Galileo Cuc
//field_name: cuc_rad
df!(
    id: df300,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/(1024.0*1024.0*512.0),
    round: true,
    ord: 0,
);

//DF301: Galileo e
//field_name: eccentricity
df!(
    id: df301,
    dt: f64,
    it: U32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*8.0),
    round: true,
    ord: 0,
);

//DF302: Galileo Cus
//field_name: cus_rad
df!(
    id: df302,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/(1024.0*1024.0*512.0),
    round: true,
    ord: 0,
);

//DF303: Galileo sqrt A
//field_name: sqrt_a_sqrt_m
df!(
    id: df303,
    dt: f64,
    it: U32,
    len: 32,
    res: 1.0/(1024.0*512.0),
    round: true,
    ord: 0,
);

//DF304: Galileo toe
//field_name: toe_s
df!(
    id: df304,
    dt: f32,
    it: U16,
    len: 14,
    res: 60.0,
    round: true,
    ord: 0,
);

//DF305: Galileo Cic
//field_name: cic_rad
df!(
    id: df305,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/(1024.0*1024.0*512.0),
    round: true,
    ord: 0,
);

//DF306: Galileo OMEGA0
//field_name: omega0_sc
df!(
    id: df306,
    dt: f64,
    it: I32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF307: Galileo Cis
//field_name: cis_rad
df!(
    id: df307,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/(1024.0*1024.0*512.0),
    round: true,
    ord: 0,
);

//DF308: Galileo i0
//field_name: i0_sc
df!(
    id: df308,
    dt: f64,
    it: I32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF309: Galileo Crc
//field_name: crc_m
df!(
    id: df309,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/32.0,
    round: true,
    ord: 0,
);

//DF310: Galileo omega
//field_name: omega_sc
df!(
    id: df310,
    dt: f64,
    it: I32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF311: Galileo OMEGADOT
//field_name: omegadot_sc_s
df!(
    id: df311,
    dt: f64,
    it: I32,
    len: 24,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*8.0),
    round: true,
    ord: 0,
);

//DF312: Galileo BGD (E1/E5a)
//field_name: bgd_e1_e5a_s
df!(
    id: df312,
    dt: f32,
    it: I16,
    len: 10,
    res: 1.0/(1024.0*1024.0*1024.0*4.0),
    round: true,
    ord: 0,
);

//DF313: Galileo BGD (E5b/E1) (Reserved)
//field_name: bgd_e1_e5b_s
df!(
    id: df313,
    dt: f32,
    it: I16,
    len: 10,
    res: 1.0/(1024.0*1024.0*1024.0*4.0),
    round: true,
    ord: 0,
);

//DF314: E5a SIGNAL Health Status
//field_name: e5a_sig_health_ind
df!(
    id: df314,
    dt: u8,
    it: U8,
    len: 2,
    ord: 0,
);

//DF315: E5a Data Validity Status
//field_name: e5a_data_validity_flag
//df: df_flag

//DF316: Galileo SOL NAV Signal Health Status (SOLHS)
//field_name: e5b_sig_health_ind
df!(
    id: df316,
    dt: u8,
    it: U8,
    len: 2,
    ord: 0,
);

//DF317: Galileo SOL NAV Data Validity Status (SOLDVS)
//field_name: e5b_data_validity_flag
//df: df_flag

//DF364: Quarter Cycle Indicator
//field_name: quarter_cycle_ind
df!(
    id: df364,
    dt: u8,
    it: U8,
    len: 2,
    ord: 0,
);

//DF365: Delta Radial
//field_name: delta_radial_m
df!(
    id: df365,
    dt: f64,
    it: I32,
    len: 22,
    res: 0.0001,
    round: true,
    ord: 0,
);

//DF366: Delta Along-Track
//field_name: delta_along_track_m
df!(
    id: df366,
    dt: f64,
    it: I32,
    len: 20,
    res: 0.0004,
    round: true,
    ord: 0,
);

//DF367: Delta Cross-Track
//field_name: delta_cross_track_m
df!(
    id: df367,
    dt: f64,
    it: I32,
    len: 20,
    res: 0.0004,
    round: true,
    ord: 0,
);

//DF368: Dot Delta Radial
//field_name: dot_delta_radial_m_s
df!(
    id: df368,
    dt: f64,
    it: I32,
    len: 21,
    res: 1.0e-6,
    round: true,
    ord: 0,
);

//DF369: Dot Delta Along-Track
//field_name: dot_delta_along_track_m_s
df!(
    id: df369,
    dt: f64,
    it: I32,
    len: 19,
    res: 4.0e-6,
    round: true,
    ord: 0,
);

//DF370: Dot Delta Cross-Track
//field_name: dot_delta_cross_track_m_s
df!(
    id: df370,
    dt: f64,
    it: I32,
    len: 19,
    res: 4.0e-6,
    round: true,
    ord: 0,
);

//DF375: Satellite Reference Datum
//field_name: satellite_reference_datum_ind
//df: df_flag

//DF376: Delta Clock C0
//field_name: delta_clock_c0_m
df!(
    id: df376,
    dt: f64,
    it: I32,
    len: 22,
    res: 1.0e-4,
    round: true,
    ord: 0,
);

//DF377: Delta Clock C1
//field_name: delta_clock_c1_m_s
df!(
    id: df377,
    dt: f64,
    it: I32,
    len: 21,
    res: 1.0e-6,
    round: true,
    ord: 0,
);

//DF378: Delta Clock C2
//field_name: delta_clock_c2_m_s2
df!(
    id: df378,
    dt: f64,
    it: I32,
    len: 27,
    res: 2.0e-8,
    round: true,
    ord: 0,
);

//DF385: GPS Epoch Time (TOW) in s
//field_name: gps_epoch_time_s
df!(
    id: df385,
    dt: u32,
    it: U32,
    len: 20,
    ord: 0,
);

//DF387: No. of Satellites
//field_name: satellites_len
df!(
    id: df387,
    dt: usize,
    it: U8,
    len: 6,
    cap: 63,DF387_CAP,
    ord: 0,
);

//DF387: No. of Satellites
//field_name: satellites_len
df!(
    id: df387_1057,
    dt: usize,
    it: U8,
    len: 6,
    cap: 60,DF387_1057_CAP,
    ord: 0,
); 

//DF388: Multiple Message Indicator
//field_name: multiple_message_flag
//df: df_flag

//DF391: SSR Update Interval
//field_name: ssr_update_interval_index
//df: df_u4

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

//DF413: IOD SSR
//field_name: iod_ssr
//df: df_u4

//DF414: SSR Provider ID
//field_name: ssr_provider_id
//df: df_u16

//DF415: SSR Solution ID
//field_name: ssr_solution_id
//df: df_u4

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

//DF421: GLONASS Code-Phase Bias Indicator
//field_name: glonass_code_phase_bias_ind
df!(
    id: df421,
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

//DF429: QZSS Satellite ID
//field_name: qzss_satellite_id
df!(
    id: df429,
    dt: u8,
    it: U8,
    len: 4,
    ord: 0,
);

//DF430: QZSS toc
//field_name: toc_s
df!(
    id: df430,
    dt: f32,
    it: U16,
    len: 16,
    res: 16.0,
    round: true,
    ord: 0,
);

//DF431: QZSS af2
//field_name: af2_s_s2
df!(
    id: df431,
    dt: f32,
    it: I8,
    len: 8,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*1024.0*32.0),
    round: true,
    ord: 0,
);

//DF432: QZSS af1
//field_name: af1_s_s
df!(
    id: df432,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*8.0),
    round: true,
    ord: 0,
);

//DF433: QZSS af0
//field_name: af0_s
df!(
    id: df433,
    dt: f64,
    it: I32,
    len: 22,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF434: QZSS IODE
//field_name: iode
df!(
    id: df434,
    dt: u8,
    it: U8,
    len: 8,
    ord: 0,
);

//DF435: QZSS Crs
//field_name: crs_m
df!(
    id: df435,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/32.0,
    round: true,
    ord: 0,
);

//DF436: QZSS Delta_n
//field_name. delta_n_sc_s
df!(
    id: df436,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*8.0),
    round: true,
    ord: 0,
);

//DF437: QZSS M0
//field_name: m0_sc
df!(
    id: df437,
    dt: f64,
    it: I32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF438: QZSS Cuc
//field_name: cuc_rad
df!(
    id: df438,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/(1024.0*1024.0*512.0),
    round: true,
    ord: 0,
);

//DF439: QZSS e
//field_name: eccentricity
df!(
    id: df439,
    dt: f64,
    it: U32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*8.0),
    round: true,
    ord: 0,
);

//DF440: QZSS Cus
//field_name: cus_rad
df!(
    id: df440,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/(1024.0*1024.0*512.0),
    round: true,
    ord: 0,
);

//DF441: QZSS sqrt A
//field_name: sqrt_a_sqrt_m
df!(
    id: df441,
    dt: f64,
    it: U32,
    len: 32,
    res: 1.0/(1024.0*512.0),
    round: true,
    ord: 0,
);

//DF442: QZSS toe
//field_name toe_s
df!(
    id: df442,
    dt: f32,
    it: U16,
    len: 16,
    res: 16.0,
    round: true,
    ord: 0,
);

//DF443: QZSS Cic
//field_name: cic_rad
df!(
    id: df443,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/(1024.0*1024.0*512.0),
    round: true,
    ord: 0,
);

//DF444: QZSS OMEGA0
//field_name: omega0_sc
df!(
    id: df444,
    dt: f64,
    it: I32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF445: QZSS Cis
//field_name: cis_rad
df!(
    id: df445,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/(1024.0*1024.0*512.0),
    round: true,
    ord: 0,
);

//DF446: QZSS i0
//field_name: i0_sc
df!(
    id: df446,
    dt: f64,
    it: I32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF447: QZSS Crc
//field_name: crc_m
df!(
    id: df447,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/32.0,
    round: true,
    ord: 0,
);

//DF448: QZSS omega
//field_name: omega_sc
df!(
    id: df448,
    dt: f64,
    it: I32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF449: QZSS OMEGADOT
//field_name: omegadot_sc_s
df!(
    id: df449,
    dt: f64,
    it: I32,
    len: 24,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*8.0),
    round: true,
    ord: 0,
);

//DF450: QZSS i0-DOT
//field_name: idot_sc_s
df!(
    id: df450,
    dt: f64,
    it: I16,
    len: 14,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*8.0),
    round: true,
    ord: 0,
);

//DF451: QZSS Codes on L2 Channel
//field_name: code_on_l2_ind
df!(
    id: df451,
    dt: u8,
    it: U8,
    len: 2,
    ord: 0,
);

//DF452: QZSS Week number
//field_name: qzss_week_number
df!(
    id: df452,
    dt: u16,
    it: U16,
    len: 10,
    ord: 0,
);

//DF453: QZSS URA
//field_name: ura_index
df!(
    id: df453,
    dt: u8,
    it: U8,
    len: 4,
    ord: 0,
);

//DF454: QZSS SV health
//field_name: sv_health_ind
df!(
    id: df454,
    dt: u8,
    it: U8,
    len: 6,
    ord: 0,
);

//DF455: QZSS TGD
//field_name: tgd_s
df!(
    id: df455,
    dt: f32,
    it: I8,
    len: 8,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF456: QZSS IODC
//field_name: iodc
df!(
    id: df456,
    dt: u16,
    it: U16,
    len: 10,
    ord: 0,
);

//DF457: QZSS Fit Interval
//field_name: fit_interval_ind
//df: df_flag

//DF488: BDS Satellite ID
//field_name: bds_satellite_id
df!(
    id: df488,
    dt: u8,
    it: U8,
    len: 6,
    ord: 0,
);

//DF489: BDS Week Number
//field_name: bds_week_number
df!(
    id: df489,
    dt: u16,
    it: U16,
    len: 13,
    ord: 0,
);

//DF490: BDS URAI
//field_name: ura_index
df!(
    id: df490,
    dt: u8,
    it: U8,
    len: 4,
    ord: 0,
);

//DF491: BDS IDOT
//field_name: idot_sc_s
df!(
    id: df491,
    dt: f64,
    it: I16,
    len: 14,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*8.0),
    round: true,
    ord: 0,
);

//DF492: BDS AODE
//field_name: aode
df!(
    id: df492,
    dt: u8,
    it: U8,
    len: 5,
    ord: 0,
);

//DF493: BDS toc
//field_name: toc_s
df!(
    id: df493,
    dt: f32,
    it: U32,
    len: 17,
    res: 8.0,
    round: true,
    ord: 0,
);

//DF494: BDS a2
//field_name: a2_s_s2
df!(
    id: df494,
    dt: f32,
    it: I16,
    len: 11,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*1024.0*1024.0*64.0),
    round: true,
    ord: 0,
);

//DF495: BDS a1
//field_name: a1_s_s
df!(
    id: df495,
    dt: f64,
    it: I32,
    len: 22,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*1024.0),
    round: true,
    ord: 0,
);

//DF496: BDS a0
//field_name: a0_s
df!(
    id: df496,
    dt: f64,
    it: I32,
    len: 24,
    res: 1.0/(1024.0*1024.0*1024.0*8.0),
    round: true,
    ord: 0,
);

//DF497: BDS AODC
//field_name: aodc
df!(
    id: df497,
    dt: u8,
    it: U8,
    len: 5,
    ord: 0,
);

//DF498: BDS Crs
//field_name: crs_m
df!(
    id: df498,
    dt: f32,
    it: I32,
    len: 18,
    res: 1.0/64.0,
    round: true,
    ord: 0,
);

//DF499: BDS DELTA n
//field_name. delta_n_sc_s
df!(
    id: df499,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*8.0),
    round: true,
    ord: 0,
);

//DF500: BDS M0
//field_name: m0_sc
df!(
    id: df500,
    dt: f64,
    it: I32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF501: BDS Cuc
//field_name: cuc_rad
df!(
    id: df501,
    dt: f32,
    it: I32,
    len: 18,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF502: BDS e
//field_name: eccentricity
df!(
    id: df502,
    dt: f64,
    it: U32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*8.0),
    round: true,
    ord: 0,
);

//DF503: BDS Cus
//field_name: cus_rad
df!(
    id: df503,
    dt: f32,
    it: I32,
    len: 18,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);


//DF504: BDS sqrt A
//field_name: sqrt_a_sqrt_m
df!(
    id: df504,
    dt: f64,
    it: U32,
    len: 32,
    res: 1.0/(1024.0*512.0),
    round: true,
    ord: 0,
);

//DF505: BDS toe
//field_name: toe_s
df!(
    id: df505,
    dt: f32,
    it: U32,
    len: 17,
    res: 8.0,
    round: true,
    ord: 0,
);

//DF506: BDS Cic
//field_name: cic_rad
df!(
    id: df506,
    dt: f32,
    it: I32,
    len: 18,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF507: BDS OMEGA0
//field_name: omega0_sc
df!(
    id: df507,
    dt: f64,
    it: I32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF508: BDS Cis
//field_name: cis_rad
df!(
    id: df508,
    dt: f32,
    it: I32,
    len: 18,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF509: BDS i0
//field_name: i0_sc
df!(
    id: df509,
    dt: f64,
    it: I32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF510: BDS Crc
//field_name: crc_m
df!(
    id: df510,
    dt: f32,
    it: I32,
    len: 18,
    res: 1.0/64.0,
    round: true,
    ord: 0,
);

//DF511: BDS omega
//field_name: omega_sc
df!(
    id: df511,
    dt: f64,
    it: I32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF512: BDS OMEGADOT
//field_name: omegadot_sc_s
df!(
    id: df512,
    dt: f64,
    it: I32,
    len: 24,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*8.0),
    round: true,
    ord: 0,
);

//DF513: BDS TGD1
//field_name: tgd1_s
df!(
    id: df513,
    dt: f32,
    it: I16,
    len: 10,
    res: 1.0/10_000_000_000.0,
    round: true,
    ord: 0,
);

//DF514: BDS TGD1
//field_name: tgd2_s
df!(
    id: df514,
    dt: f32,
    it: I16,
    len: 10,
    res: 1.0/10_000_000_000.0,
    round: true,
    ord: 0,
);

//DF515: BDS SV Health
//field_name: sv_health_flag
//df: df_flag

//DF516: NavIC/IRNSS Satellite ID
//field_name: navic_satellite_id
df!(
    id: df516,
    dt: u8,
    it: U8,
    len: 6,
    ord: 0,
);

//DF517: NavIC/IRNSS Week Number
//field_name: navic_week_number
df!(
    id: df517,
    dt: u16,
    it: U16,
    len: 10,
    ord: 0,
);

//DF518: NavIC/IRNSS af0
//field_name: af0_s
df!(
    id: df518,
    dt: f64,
    it: I32,
    len: 22,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF519: NavIC/IRNSS af1
//field_name: af1_s_s
df!(
    id: df519,
    dt: f32,
    it: I16,
    len: 16,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*8.0),
    round: true,
    ord: 0,
);

//DF520: NavIC/IRNSS af2
//field_name: af2_s_s2
df!(
    id: df520,
    dt: f32,
    it: I8,
    len: 8,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*1024.0*32.0),
    round: true,
    ord: 0,
);

//DF521: NavIC/IRNSS SV Accuracy (URA)
//field_name: ura_index
df!(
    id: df521,
    dt: u8,
    it: U8,
    len: 4,
    ord: 0,
);

//DF522: NavIC/IRNSS Time of clock (toc)
//field_name: toc_s
df!(
    id: df522,
    dt: f32,
    it: U16,
    len: 16,
    res: 16.0,
    round: true,
    ord: 0,
);

//DF523: NavIC/IRNSS Total Group Delay (TGD)
//field_name: tgd_s
df!(
    id: df523,
    dt: f32,
    it: I8,
    len: 8,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF524: NavIC/IRNSS DELTA n
//field_name. delta_n_sc_s
df!(
    id: df524,
    dt: f64,
    it: I32,
    len: 22,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF525: NavIC/IRNSS Issue of Data Ephemeris and Clock
//field_name: iodec
df!(
    id: df525,
    dt: u8,
    it: U8,
    len: 8,
    ord: 0,
);

//DF526: Reserved (10 bits)

//DF527: NavIC/IRNSS L5 Flag
//field_name: l5_flag
//df: df_flag

//DF528: NavIC/IRNSS S Flag
//field_name: s_flag
//df: df_flag

//DF529: NavIC/IRNSS Cuc
//field_name: cuc_rad
df!(
    id: df529,
    dt: f32,
    it: I16,
    len: 15,
    res: 1.0/(1024.0*1024.0*256.0),
    round: true,
    ord: 0,
);

//DF530: NavIC/IRNSS Cus
//field_name: cus_rad
df!(
    id: df530,
    dt: f32,
    it: I16,
    len: 15,
    res: 1.0/(1024.0*1024.0*256.0),
    round: true,
    ord: 0,
);

//DF531: NavIC/IRNSS Cic
//field_name: cic_rad
df!(
    id: df531,
    dt: f32,
    it: I16,
    len: 15,
    res: 1.0/(1024.0*1024.0*256.0),
    round: true,
    ord: 0,
);

//DF532: NavIC/IRNSS Cis
//field_name: cis_rad
df!(
    id: df532,
    dt: f32,
    it: I16,
    len: 15,
    res: 1.0/(1024.0*1024.0*256.0),
    round: true,
    ord: 0,
);

//DF533: NavIC/IRNSS Crc
//field_name: crc_m
df!(
    id: df533,
    dt: f32,
    it: I16,
    len: 15,
    res: 1.0/16.0,
    round: true,
    ord: 0,
);

//DF534: NavIC/IRNSS Crs
//field_name: crs_m
df!(
    id: df534,
    dt: f32,
    it: I16,
    len: 15,
    res: 1.0/16.0,
    round: true,
    ord: 0,
);

//DF535: NavIC/IRNSS IDOT
//field_name: idot_sc_s
df!(
    id: df535,
    dt: f64,
    it: I16,
    len: 14,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*8.0),
    round: true,
    ord: 0,
);

//DF536: NavIC/IRNSS M0
//field_name: m0_sc
df!(
    id: df536,
    dt: f64,
    it: I32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF537: NavIC/IRNSS toe
//field_name toe_s
df!(
    id: df537,
    dt: f32,
    it: U16,
    len: 16,
    res: 16.0,
    round: true,
    ord: 0,
);

//DF538: NavIC/IRNSS Eccentricity
//field_name: eccentricity
df!(
    id: df538,
    dt: f64,
    it: U32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*8.0),
    round: true,
    ord: 0,
);

//DF539: NavIC/IRNSS sqrt A
//field_name: sqrt_a_sqrt_m
df!(
    id: df539,
    dt: f64,
    it: U32,
    len: 32,
    res: 1.0/(1024.0*512.0),
    round: true,
    ord: 0,
);

//DF540: NavIC/IRNSS OMEGA0
//field_name: omega0_sc
df!(
    id: df540,
    dt: f64,
    it: I32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF541: NavIC/IRNSS omega
//field_name: omega_sc
df!(
    id: df541,
    dt: f64,
    it: I32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF542: NavIC/IRNSS OMEGADOT
//field_name: omegadot_sc_s
df!(
    id: df542,
    dt: f64,
    it: I32,
    len: 22,
    res: 1.0/(1024.0*1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF543: NavIC/IRNSS i0
//field_name: i0_sc
df!(
    id: df543,
    dt: f64,
    it: I32,
    len: 32,
    res: 1.0/(1024.0*1024.0*1024.0*2.0),
    round: true,
    ord: 0,
);

//DF544: NavIC/IRNSS 2 spare bits after IDOT
//field_name: spare_idot

//DF545: NavIC/IRNSS 2 spare bits after I0
//field_name: spare_i0

//DF546: NavIC/IRNSS Epoch Time (TOW)
//field_name: navic_epoch_time_ms
df!(
    id: df546,
    dt: u32,
    it: U32,
    len: 30,
    ord: 0,
);

//Reusable data fields

//flag
df!(
    id: df_flag,
    dt: u8,
    it: U8,
    len: 1,
    ord: 0,
);

//unsigned integers
df!(
    id: df_u2,
    dt: u8,
    it: U8,
    len: 2,
    ord: 0,
);

df!(
    id: df_u3,
    dt: u8,
    it: U8,
    len: 3,
    ord: 0,
);

df!(
    id: df_u4,
    dt: u8,
    it: U8,
    len: 4,
    ord: 0,
);

df!(
    id: df_u5,
    dt: u8,
    it: U8,
    len: 5,
    ord: 0,
);

df!(
    id: df_u6,
    dt: u8,
    it: U8,
    len: 6,
    ord: 0,
);

df!(
    id: df_u7,
    dt: u8,
    it: U8,
    len: 7,
    ord: 0,
);

df!(
    id: df_u8,
    dt: u8,
    it: U8,
    len: 8,
    ord: 0,
);

df!(
    id: df_u16,
    dt: u16,
    it: U16,
    len: 16,
    ord: 0,
);

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

//Data fields specific to one message

pub mod df_msg1029_utf8_str;
pub mod df_msg1230_biases;
