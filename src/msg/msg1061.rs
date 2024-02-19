msg! (
    id: msg1061_sat,
    type_name: Msg1061Sat,
    fields: [
        (gps_satellite_id, df_u6),
        (ssr_ura_class, df_u3),
        (ssr_ura_value, df_u3)
    ],
);

frag_vec_with_len!(
    id: msg1061_sat_vec,
    frag_id: msg1061_sat,
    cap: SAT_CAP_1061,
    len_bits: 6,
);

msg!(
    id: msg1061,
    type_name: Msg1061T,
    fields: [
        (gps_epoch_time_s, df385),
        (ssr_update_interval_index, df_u4),
        (multiple_message_flag, df_flag),
        (iod_ssr, df_u4),
        (ssr_provider_id, df_u16),
        (ssr_solution_id, df_u4),
        (satellites, msg1061_sat_vec)
    ],
);
