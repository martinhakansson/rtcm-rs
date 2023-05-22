msg!(
    id: msg1008,
    type_name: Msg1008T,
    fields: [
        (reference_station_id, df003),
        (antenna_desc_char_len, df029),
        (antenna_descriptor_str, df030, antenna_desc_char_len),
        (antenna_setup_id, df031),
        (serial_number_len, df032),
        (antenna_serial_number_str, df033, serial_number_len)
    ],
);
