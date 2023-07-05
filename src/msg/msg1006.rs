msg!(
    id: msg1006,
    type_name: Msg1006T,
    fields: [
        (reference_station_id, df003),
        (reserved_24_6, df001_6bits),
        (gps_flag, df022),
        (glonass_flag, df023),
        (galileo_flag, df024),
        (reference_station_ind, df141),
        (antenna_ref_point_ecef_x_m, df025),
        (single_receiver_osc_ind, df142),
        (reserved_73_1, df001_1bit),
        (antenna_ref_point_ecef_y_m, df026),
        (quarter_cycle_ind, df364),
        (antenna_ref_point_ecef_z_m, df027),
        (antenna_height_m, df028)
    ],
);