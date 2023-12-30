msg! (
    id: msg1011_sat,
    type_name: Msg1011Sat,
    fields: [
        (glo_satellite_id, df_u6),
        (glo_l1_code_ind, df_flag),
        (glo_satellite_freq_chan_number, df040),
        (l1_pseudorange_m, df041),
        (l1_phase_pseudorange_diff_m, df012_18_42_48),
        (l1_lock_time_index, df_u7),
        (glo_l2_code_ind, df_u2),
        (l2_l1_pseudorange_diff_m, df017_47),
        (l2_phase_l1_pseudorange_diff_m, df012_18_42_48),
        (l2_lock_time_index, df_u7)
    ],
);

frag_vec!(
    id: msg1011_sat_vec,
    frag_id: msg1011_sat,
    cap_name: SAT_CAP_LEGACY,
);

msg_len_middle!(
    id: msg1011,
    type_name: Msg1011T,
    fields1: [
        (reference_station_id, df003),
        (glo_epoch_time_ms, df_u27),
        (synchronous_gnss_msg_flag, df005)
    ],
    len_field: df_leg_sat_len,
    fields2: [
        (divergence_free_smoothing_flag, df_flag),
        (smoothing_interval_index, df_u3)
    ],
    vec_field: satellites, msg1011_sat_vec,
);
