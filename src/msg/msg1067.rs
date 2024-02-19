msg! (
    id: msg1067_sat,
    type_name: Msg1067Sat,
    fields: [
        (glo_satellite_id, df_u5),
        (ssr_ura_class, df_u3),
        (ssr_ura_value, df_u3)
    ],
);

frag_vec_with_len!(
    id: msg1067_sat_vec,
    frag_id: msg1067_sat,
    cap: SAT_CAP_1067,
    len_bits: 6,
);

msg!(
    id: msg1067,
    type_name: Msg1067T,
    fields: [
        (glo_epoch_time_s, df_u17),
        (ssr_update_interval_index, df_u4),
        (multiple_message_flag, df_flag),
        (iod_ssr, df_u4),
        (ssr_provider_id, df_u16),
        (ssr_solution_id, df_u4),
        (satellites, msg1067_sat_vec)
    ],
);
