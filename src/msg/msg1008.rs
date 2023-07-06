msg!(
    id: msg1008,
    type_name: Msg1008T,
    fields: [
        (reference_station_id, df003),
        (antenna_descriptor_len, df_desc_str_len),
        (antenna_descriptor_str, df_desc_str, antenna_descriptor_len),
        (antenna_setup_id, df031),
        (antenna_serial_number_len, df_desc_str_len),
        (antenna_serial_number_str, df_desc_str, antenna_serial_number_len)
    ],
);
