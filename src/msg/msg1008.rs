
msg!(
    id: msg1008,
    type_name: Msg1008T,
    fields: [
        (ref_station_id, df003),
        (desc_counter, df029),
        (antenna_descriptor, df030, desc_counter),
        (antenna_setup_id, df031),
        (serial_number_counter, df032),
        (antenna_serial_number, df033, serial_number_counter)
    ],
);
