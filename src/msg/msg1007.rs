msg!(
    id: msg1007,
    type_name: Msg1007T,
    fields: [
        (reference_station_id, df003),
        (antenna_desc_char_len, df029),
        (antenna_descriptor_str, df030, antenna_desc_char_len),
        (antenna_setup_id, df031)
    ],
);
