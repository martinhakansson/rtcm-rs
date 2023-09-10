msg! (
    id: msg1068_sat,
    type_name: Msg1068Sat,
    fields: [
        (glo_satellite_id, df_u5),
        (high_rate_clock_correction_m, df390)
    ],
);

frag_vec_with_len!(
    id: msg1068_sat_vec,
    frag_id: msg1068_sat,
    cap: SAT_CAP_1068, 63,
    len_bits: 6,
);

msg!(
    id: msg1068,
    type_name: Msg1068T,
    fields: [
        (glo_epoch_time_s, df_u17),
        (ssr_update_interval_index, df_u4),
        (multiple_message_flag, df_flag),
        (iod_ssr, df_u4),
        (ssr_provider_id, df_u16),
        (ssr_solution_id, df_u4),
        (satellites, msg1068_sat_vec)
    ],
);
