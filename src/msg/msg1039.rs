msg! (
    id: msg1039_sat,
    type_name: Msg1039Sat,
    fields: [
        (glo_satellite_id, df_u6),
        (amb_status_ind,df_u2),
        (non_sync_count,df_u3),
        (geom_phase_diff_m,df069_70_237_38),
        (iod,df_u8),
        (iono_phase_diff_m,df069_70_237_38)
    ],
);

frag_vec_with_len!(
    id: msg1039_sat_vec,
    frag_id: msg1039_sat,
    cap: SAT_CAP_1039, 15,
    len_bits: 4,
);

msg!(
    id: msg1039,
    type_name: Msg1039T,
    fields: [
        (network_id,df_u8),
        (subnetwork_id,df_u4),
        (glo_epoch_time_s,df233),
        (multiple_message_flag,df_flag),
        (ma_reference_station_id,df_u12),
        (aux_reference_station_id,df_u12),
        (satellites,msg1039_sat_vec)
    ],
);
