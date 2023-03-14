msg!(
    id: msg1005,
    type_name: Msg1005T,
    fields: [
        //(message_number, df002),
        (reference_station_id, df003),
        (reserved_itrf_year_do_not_use, df001_6bits),
        (gps_indicator, df022),
        (glonass_indicator, df023),
        (reserved_galileo_indicator_do_not_use, df001_1bit),
        (reference_station_indicator, df141),
        (antenna_reference_point_ecef_x, df025),
        (single_receiver_oscillator_indicator, df142),
        (reserved, df001_1bit),
        (antenna_reference_point_ecef_y, df026),
        (quarter_cycle_indicator, df364),
        (antenna_reference_point_ecef_z, df027)
    ],
);