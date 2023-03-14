msg! (
    id: msg1001_sat,
    type_name: Msg1001Sat,
    fields: [
        (gps_satellite_id, df009),
        (gps_l1_code_indicator, df010),
        (gps_l1_pseudorange_m, df011),
        (gps_l1_phase_pseudorange_m, df012),
        (gps_l1_lock_time_indicator, df013)
    ],
);

frag_vec!(id: msg1001_sat_vec, frag_id: msg1001_sat, cap_name: DF006_CAP,);

msg!(
    id: msg1001,
    type_name: Msg1001T,
    fields: [
        //(message_number, df002),
        (ref_station_id, df003),
        (gps_epoch_time, df004),
        (synchronous_gnss_message_flag, df005),
        (num_gps_sat_sig_proc, df006),
        (gps_div_free_smoothing_indicator, df007),
        (gps_smoothing_interval, df008),
        (satellites, msg1001_sat_vec, num_gps_sat_sig_proc)
    ],
);
