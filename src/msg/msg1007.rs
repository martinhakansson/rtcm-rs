msg!(
    id: msg1007,
    type_name: Msg1007T,
    fields: [
        (reference_station_id, df003),
        (antenna_descriptor_len, df_desc_str_len),
        (antenna_descriptor_str, df_desc_str, antenna_descriptor_len),
        (antenna_setup_id, df031)
    ],
);
