use super::msm123_sat::*;

msm_sig_frag!(
    id: msg1081_sig,
    type_name: Msg1081Sig,
    gnss: glo,
    fields: [(gnss_signal_fine_pseudorange_ms, df400)],
);

msm_data_seg_frag!(
    id: msg1081_data,
    type_name: Msg1081Data,
    gnss: glo,
    sat_id: msm123_sat,
    sig_id: msg1081_sig,
);

msg!(
    id: msg1081,
    type_name: Msg1081T,
    fields: [
        (reference_station_id, df003),
        (glo_day_of_week, df416),
        (glo_epoch_time_ms, df034),
        (msm_multiple_message_flag, df393),
        (issue_of_data_station, df409),
        (reserved_58_7, df001_7bits),
        (clock_steering_ind, df411),
        (external_clock_ind, df412),
        (gnss_smoothing_type_ind, df417),
        (gnss_smoothing_interval_index, df_u3),
        (data_segment, msg1081_data)
    ],
);
