msg! (
    id: msg1002_sat,
    type_name: Msg1002Sat,
    fields: [
        (gps_satellite_id, df009),
        (gps_l1_code_ind, df010),
        (l1_pseudorange_m, df011),
        (l1_phase_pseudorange_diff_m, df012_18),
        (l1_lock_time_index, df013),
        (l1_pseudorange_amb_m, df014),
        (l1_cnr_dbhz, df015)
    ],
);

frag_vec!(
    id: msg1002_sat_vec,
    frag_id: msg1002_sat,
    cap_name: SAT_CAP_LEGACY,
);

msg_len_middle!(
    id: msg1002,
    type_name: Msg1002T,
    fields1: [
        (reference_station_id, df003),
        (gps_epoch_time_ms, df004),
        (synchronous_gnss_msg_flag, df005)
    ],
    len_field: df_leg_sat_len,
    fields2: [
        (divergence_free_smoothing_flag, df007),
        (smoothing_interval_index, df008)
    ],
    vec_field: satellites, msg1002_sat_vec,
);
