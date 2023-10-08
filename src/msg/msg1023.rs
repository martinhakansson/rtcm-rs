frag_grid16p!(
    id: grid199_200,
    frag_id: df199_200,
);

frag_grid16p!(
    id: grid201,
    frag_id: df201,
);

msg!(
    id: msg1023,
    type_name: Msg1023T,
    fields: [
        (system_id,df_u8),
        (hor_shift_flag,df_flag),
        (ver_shift_flag,df_flag),
        (phi_0_asec,df192),
        (lambda_0_asec,df193),
        (delta_phi_asec,df194_5),
        (delta_lambda_asec,df194_5),
        (mean_delta_phi_asec,df196_7),
        (mean_delta_lambda_asec,df196_7),
        (mean_delta_h_m,df198),
        (grid_delta_phi_asec,grid199_200),
        (grid_delta_lambda_asec,grid199_200),
        (grid_delta_h_m,grid201),
        (hor_interp_ind,df_u2),
        (ver_interp_ind,df_u2),
        (hor_grid_quality_ind,df_u3),
        (ver_grid_quality_ind,df_u3),
        (modified_julian_day_number,df051)
    ],
);
