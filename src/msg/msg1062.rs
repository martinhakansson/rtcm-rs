msg! (
    id: msg1062_sat,
    type_name: Msg1062Sat,
    fields: [
        (gps_satellite_id, df_u6),
        (high_rate_clock_correction_m, df390)
    ],
);

frag_vec_with_len!(
    id: msg1062_sat_vec,
    frag_id: msg1062_sat,
    cap: SAT_CAP_1062,
    len_bits: 6,
);

msg!(
    id: msg1062,
    type_name: Msg1062T,
    fields: [
        (gps_epoch_time_s, df385),
        (ssr_update_interval_index, df_u4),
        (multiple_message_flag, df_flag),
        (iod_ssr, df_u4),
        (ssr_provider_id, df_u16),
        (ssr_solution_id, df_u4),
        (satellites, msg1062_sat_vec)
    ],
);
