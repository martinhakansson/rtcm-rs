msg! (
    id: msg1015_sat,
    type_name: Msg1015Sat,
    fields: [
        (gps_satellite_id, df_u6),
        (amb_status_ind,df_u2),
        (non_sync_count,df_u3),
        (iono_phase_diff_m,df069_70_237_38)
    ],
);

frag_vec_with_len!(
    id: msg1015_sat_vec,
    frag_id: msg1015_sat,
    cap: SAT_CAP_MAC,
    len_bits: 4,
);

msg!(
    id: msg1015,
    type_name: Msg1015T,
    fields: [
        (network_id,df_u8),
        (subnetwork_id,df_u4),
        (gps_epoch_time_s,df065),
        (multiple_message_flag,df_flag),
        (ma_reference_station_id,df_u12),
        (aux_reference_station_id,df_u12),
        (satellites,msg1015_sat_vec)
    ],
);
