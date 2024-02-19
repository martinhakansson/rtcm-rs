msg! (
    id: msg1063_sat,
    type_name: Msg1063Sat,
    fields: [
        (glo_satellite_id, df_u5),
        (iod, df_u8),
        (delta_radial_m, df365),
        (delta_along_track_m, df366),
        (delta_cross_track_m, df367),
        (dot_delta_radial_m_s, df368),
        (dot_delta_along_track_m_s, df369),
        (dot_delta_cross_track_m_s, df370)
    ],
);

frag_vec_with_len!(
    id: msg1063_sat_vec,
    frag_id: msg1063_sat,
    cap: SAT_CAP_1063,
    len_bits: 6,
);

msg!(
    id: msg1063,
    type_name: Msg1063T,
    fields: [
        (glo_epoch_time_s, df_u17),
        (ssr_update_interval_index, df_u4),
        (multiple_message_flag, df_flag),
        (satellite_reference_datum_ind, df_flag),
        (iod_ssr, df_u4),
        (ssr_provider_id, df_u16),
        (ssr_solution_id, df_u4),
        (satellites, msg1063_sat_vec)
    ],
);
