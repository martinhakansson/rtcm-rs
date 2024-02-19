msg! (
    id: msg1035_sat,
    type_name: Msg1035Sat,
    fields: [
        (glo_satellite_id, df_u6),
        (iod,df_u8),
        (geom_grad_north_ppm,df242_3),
        (geom_grad_east_ppm,df242_3),
        (iono_grad_north_ppm,df244_5),
        (iono_grad_east_ppm,df244_5)
    ],
);

frag_vec_with_len!(
    id: msg1035_sat_vec,
    frag_id: msg1035_sat,
    cap: SAT_CAP_FKP,
    len_bits: 5,
);

msg!(
    id: msg1035,
    type_name: Msg1035T,
    fields: [
        (reference_station_id,df_u12),
        (glo_fkp_epoch_time_s,df_u17),
        (satellites,msg1035_sat_vec)
    ],
);
