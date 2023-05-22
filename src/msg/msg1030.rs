msg! (
    id: msg1030_sat,
    type_name: Msg1030Sat,
    fields: [
        (gps_satellite_id, df009),
        (s_oc_mm, df218),
        (s_od_ppm, df219),
        (s_oh_ppm, df220),
        (s_ic_mm, df221),
        (s_id_ppm, df222)
    ],
);

frag_vec!(
    id: msg1030_sat_vec,
    frag_id: msg1030_sat,
    cap_name: DF006_CAP,
);

msg!(
    id: msg1030,
    type_name: Msg1030T,
    fields: [
        (gps_resid_epoch_time_s, df224),
        (reference_station_id, df003),
        (refs_n, df223),
        (satellites_len, df006),
        (satellites, msg1030_sat_vec, satellites_len)
    ],
);
