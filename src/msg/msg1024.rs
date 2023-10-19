msg!(
    id: msg1023_gp_plane,
    type_name: Msg1023GpPlane,
    fields: [
        (delta_n_m,df201_209_10_11),
        (delta_e_m,df201_209_10_11),
        (delta_h_m,df201_209_10_11)
    ],
);

frag_grid16p!(
    id: grid_plane,
    frag_id: msg1023_gp_plane,
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
        (grid,grid_plane),
        (hor_interp_ind,df_u2),
        (ver_interp_ind,df_u2),
        (hor_grid_quality_ind,df_u3),
        (ver_grid_quality_ind,df_u3),
        (modified_julian_day_number,df051)
    ],
);
