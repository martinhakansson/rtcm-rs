use super::msm123_sat::*;

msm_sig_frag!(
    id: msg1131_sig,
    type_name: Msg1131Sig,
    gnss: navic,
    fields: [(gnss_signal_fine_pseudorange_ms, df400)],
);

msm_data_seg_frag!(
    id: msg1131_data,
    type_name: Msg1131Data,
    gnss: navic,
    sat_id: msm123_sat,
    sig_id: msg1131_sig,
);

msg!(
    id: msg1131,
    type_name: Msg1131T,
    fields: [
        (reference_station_id, df003),
        (navic_epoch_time_ms, df_u30),
        (msm_multiple_message_flag, df393),
        (issue_of_data_station, df409),
        (reserved_58_7, df001_7bits),
        (clock_steering_ind, df411),
        (external_clock_ind, df412),
        (gnss_smoothing_type_ind, df417),
        (gnss_smoothing_interval_index, df_u3),
        (data_segment, msg1131_data)
    ],
);
