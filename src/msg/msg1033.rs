msg!(
    id: msg1033,
    type_name: Msg1033T,
    fields: [
        (reference_station_id, df003),
        (antenna_descriptor_len, df_desc_str_len),
        (antenna_descriptor_str, df_desc_str, antenna_descriptor_len),
        (antenna_setup_id, df031),
        (antenna_serial_number_len, df_desc_str_len),
        (antenna_serial_number_str, df_desc_str, antenna_serial_number_len),
        (receiver_type_descriptor_len, df_desc_str_len),
        (receiver_type_descriptor_str, df_desc_str, receiver_type_descriptor_len),
        (receiver_firmware_version_len, df_desc_str_len),
        (receiver_firmware_version_str, df_desc_str, receiver_firmware_version_len),
        (receiver_serial_number_len, df_desc_str_len),
        (receiver_serial_number_str, df_desc_str, receiver_serial_number_len)
    ],
);