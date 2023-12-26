msg! (
    id: msg1003_sat,
    type_name: Msg1003Sat,
    fields: [
        (gps_satellite_id, df009),
        (gps_l1_code_ind, df010),
        (l1_pseudorange_m, df011),
        (l1_phase_pseudorange_diff_m, df012_18),
        (l1_lock_time_index, df013),
        (gps_l2_code_ind, df_u2),
        (l1_l2_pseudorange_diff_m, df017),
        (l2_phase_l1_pseudorange_diff_m, df012_18),
        (l2_lock_time_index, df_u7)
    ],
);

frag_vec!(
    id: msg1003_sat_vec,
    frag_id: msg1003_sat,
    cap_name: SAT_CAP_LEGACY,
);

msg_len_middle!(
    id: msg1003,
    type_name: Msg1003T,
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
    vec_field: satellites, msg1003_sat_vec,
);
