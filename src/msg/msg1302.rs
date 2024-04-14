msg! (
    id: msg1302_link,
    type_name: Msg1302Link,
    fields: [
        (database_link_str, df_desc_str_w_len)
    ],
);

frag_vec_with_len!(
    id: msg1302_link_vec,
    frag_id: msg1302_link,
    cap: DB_LINK_CAP,
    len_bits: 3,
);

msg!(
    id: msg1302,
    type_name: Msg1302T,
    fields: [
        (rtcm_crs_name_str, df_desc_str_w_len),
        (anchor_flag, df_flag),
        (plate_number, df_u5),
        (db_links, msg1302_link_vec)
    ],
);
