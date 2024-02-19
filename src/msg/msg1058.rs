msg! (
    id: msg1058_sat,
    type_name: Msg1058Sat,
    fields: [
        (gps_satellite_id, df_u6),
        (delta_clock_c0_m, df376),
        (delta_clock_c1_m_s, df377),
        (delta_clock_c2_m_s2, df378)
    ],
);

frag_vec_with_len!(
    id: msg1058_sat_vec,
    frag_id: msg1058_sat,
    cap: SAT_CAP_1058,
    len_bits: 6,
);

msg!(
    id: msg1058,
    type_name: Msg1058T,
    fields: [
        (gps_epoch_time_s, df385),
        (ssr_update_interval_index, df_u4),
        (multiple_message_flag, df_flag),
        (iod_ssr, df_u4),
        (ssr_provider_id, df_u16),
        (ssr_solution_id, df_u4),
        (satellites, msg1058_sat_vec)
    ],
);
