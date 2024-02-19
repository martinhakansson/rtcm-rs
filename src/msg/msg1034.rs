msg! (
    id: msg1034_sat,
    type_name: Msg1034Sat,
    fields: [
        (gps_satellite_id, df_u6),
        (iode,df_u8),
        (geom_grad_north_ppm,df242_3),
        (geom_grad_east_ppm,df242_3),
        (iono_grad_north_ppm,df244_5),
        (iono_grad_east_ppm,df244_5)
    ],
);

frag_vec_with_len!(
    id: msg1034_sat_vec,
    frag_id: msg1034_sat,
    cap: SAT_CAP_FKP,
    len_bits: 5,
);

msg!(
    id: msg1034,
    type_name: Msg1034T,
    fields: [
        (reference_station_id,df_u12),
        (gps_fkp_epoch_time_s,df_u20),
        (satellites,msg1034_sat_vec)
    ],
);
