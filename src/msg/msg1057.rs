msg! (
    id: msg1057_sat,
    type_name: Msg1057Sat,
    fields: [
        (gps_satellite_id, df_u6),
        (iode, df071),
        (delta_radial_m, df365),
        (delta_along_track_m, df366),
        (delta_cross_track_m, df367),
        (dot_delta_radial_m_s, df368),
        (dot_delta_along_track_m_s, df369),
        (dot_delta_cross_track_m_s, df370)
    ],
);

frag_vec!(
    id: msg1057_sat_vec,
    frag_id: msg1057_sat,
    cap_name: DF387_1057_CAP,
);

msg!(
    id: msg1057,
    type_name: Msg1057T,
    fields: [
        (gps_epoch_time_s, df385),
        (ssr_update_interval_index, df_u4),
        (multiple_message_flag, df_flag),
        (satellite_reference_datum_ind, df_flag),
        (iod_ssr, df_u4),
        (ssr_provider_id, df_u16),
        (ssr_solution_id, df_u4),
        (satellites_len, df387_1057),
        (satellites, msg1057_sat_vec, satellites_len)
    ],
);
