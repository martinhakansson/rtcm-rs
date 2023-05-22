msg! (
    id: msg1001_sat,
    type_name: Msg1001Sat,
    fields: [
        (gps_satellite_id, df009),
        (gps_l1_code_ind, df010),
        (gps_l1_pseudorange_m, df011),
        (gps_l1_phase_pseudorange_diff_m, df012),
        (gps_l1_lock_time_bitval, df013)
    ],
);

frag_vec!(
    id: msg1001_sat_vec,
    frag_id: msg1001_sat,
    cap_name: DF006_CAP,
);

msg!(
    id: msg1001,
    type_name: Msg1001T,
    fields: [
        //(message_number, df002),
        (reference_station_id, df003),
        (gps_epoch_time_ms, df004),
        (synchronous_gnss_msg_flag, df005),
        (satellites_len, df006),
        (gps_divergence_free_smoothing_flag, df007),
        (gps_smoothing_interval_bitval, df008),
        (satellites, msg1001_sat_vec, satellites_len)
    ],
);
