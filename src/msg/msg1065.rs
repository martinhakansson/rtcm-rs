msg!(
    id: msg1065,
    type_name: Msg1065T,
    fields: [
        (glo_epoch_time_s, df_u17),
        (ssr_update_interval_index, df_u4),
        (multiple_message_flag, df_flag),
        (iod_ssr, df_u4),
        (ssr_provider_id, df_u16),
        (ssr_solution_id, df_u4),
        (biases, df_msg1065_biases)
    ],
);