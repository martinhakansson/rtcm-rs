msg!(
    id: msg1021,
    type_name: Msg1021T,
    fields: [
        (source_name_str, df_desc_str_w_len),
        (target_name_str, df_desc_str_w_len),
        (system_id,df_u8),
        (utilized_trans_msg_ind,df_u10),
        (plate_number,df_u5),
        (computation_ind,df_u4),
        (height_ind,df_u2),
        (phi_v_asec,df152),
        (lambda_v_asec,df153),
        (delta_phi_v_asec,df154_5),
        (delta_lambda_v_asec,df154_5),
        (dx_m,df156_7_8),
        (dy_m,df156_7_8),
        (dz_m,df156_7_8),
        (r_1_asec,df159_60_61),
        (r_2_asec,df159_60_61),
        (r_3_asec,df159_60_61),
        (ds_ppm,df162),
        (a_s_m,df166_8),
        (b_s_m,df167_9),
        (a_t_m,df166_8),
        (b_t_m,df167_9),
        (hor_hel_mol_quality_ind,df_u3),
        (ver_hel_mol_quality_ind,df_u3)
    ],
);
