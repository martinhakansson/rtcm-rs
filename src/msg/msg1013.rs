msg!(
    id: msg1013_msg_id,
    type_name: Msg1013MsgId,
    fields: [
        (message_id, df002),
        (message_sync_flag, df056),
        (message_transmission_interval_s, df057)
    ],
);

frag_vec!(
    id: msg_1013_msg_id_vec,
    frag_id: msg1013_msg_id,
    cap_name: SAT_CAP_1013,
);

msg_len_middle!(
    id: msg1013,
    type_name: Msg1013T,
    fields1: [
        (reference_station_id, df003),
        (modified_julian_day_number, df051),
        (seconds_of_day_s, df052)
    ],
    len_field: df053,
    fields2: [
        (leap_seconds_gps_utc_s, df054)
    ],
    vec_field: message_id_announcements, msg_1013_msg_id_vec,
);

// msg!(
//     id: msg1013,
//     type_name: Msg1013T,
//     fields: [
//         (reference_station_id, df003),
//         (modified_julian_day_number, df051),
//         (seconds_of_day_s, df052),
//         (message_id_announcements_len, df053),
//         (leap_seconds_gps_utc_s, df054),
//         (message_id_announcements, msg_1013_msg_id_vec, message_id_announcements_len)
//     ],
// );
