frag_grid16p!(
    id: grid209_10_11,
    frag_id: df201_209_10_11,
);

msg!(
    id: msg1024,
    type_name: Msg1024T,
    fields: [
        (system_id,df_u8),
        (hor_shift_flag,df_flag),
        (ver_shift_flag,df_flag),
        (n_0_m,df202),
        (e_0_m,df203),
        (delta_n_m,df204_5),
        (delta_e_m,df204_5),
        (mean_delta_n_m,df206_7),
        (mean_delta_e_m,df206_7),
        (mean_delta_h_m,df198_208),
        (grid_delta_n_m,grid209_10_11),
        (grid_delta_e_m,grid209_10_11),
        (grid_delta_h_m,grid209_10_11),
        (hor_interp_ind,df_u2),
        (ver_interp_ind,df_u2),
        (hor_grid_quality_ind,df_u3),
        (ver_grid_quality_ind,df_u3),
        (modified_julian_day_number,df051)
    ],
);
