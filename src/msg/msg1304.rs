msg! (
    id: msg1304_sat,
    type_name: Msg1304Sat,
    fields: [
        (gal_satellite_id, df_u6),
        (s_oc_mm, df218),
        (s_od_ppm, df219),
        (s_oh_ppm, df220),
        (s_ic_mm, df221),
        (s_id_ppm, df222)
    ],
);

frag_vec_with_len!(
    id: msg1304_sat_vec,
    frag_id: msg1304_sat,
    cap: SAT_CAP_RES,
    len_bits: 5,
);

msg!(
    id: msg1304,
    type_name: Msg1304T,
    fields: [
        (gal_resid_epoch_time_s, df_u20),
        (reference_station_id, df003),
        (refs_n, df223),
        (satellites, msg1304_sat_vec)
    ],
);
