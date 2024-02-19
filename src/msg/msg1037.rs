msg! (
    id: msg1037_sat,
    type_name: Msg1037Sat,
    fields: [
        (glo_satellite_id, df_u6),
        (amb_status_ind,df_u2),
        (non_sync_count,df_u3),
        (iono_phase_diff_m,df069_70_237_38)
    ],
);

frag_vec_with_len!(
    id: msg1037_sat_vec,
    frag_id: msg1037_sat,
    cap: SAT_CAP_MAC,
    len_bits: 4,
);

msg!(
    id: msg1037,
    type_name: Msg1037T,
    fields: [
        (network_id,df_u8),
        (subnetwork_id,df_u4),
        (glo_epoch_time_s,df233),
        (multiple_message_flag,df_flag),
        (ma_reference_station_id,df_u12),
        (aux_reference_station_id,df_u12),
        (satellites,msg1037_sat_vec)
    ],
);
