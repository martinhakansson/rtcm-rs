msg! (
    id: msg1031_sat,
    type_name: Msg1031Sat,
    fields: [
        (glo_satellite_id, df038),
        (s_oc_mm, df218),
        (s_od_ppm, df219),
        (s_oh_ppm, df220),
        (s_ic_mm, df221),
        (s_id_ppm, df222)
    ],
);

frag_vec_with_len!(
    id: msg1031_sat_vec,
    frag_id: msg1031_sat,
    cap: SAT_CAP_RES,
    len_bits: 5,
);

msg!(
    id: msg1031,
    type_name: Msg1031T,
    fields: [
        (glo_resid_epoch_time_s, df_u17),
        (reference_station_id, df003),
        (refs_n, df223),
        (satellites, msg1031_sat_vec)
    ],
);
