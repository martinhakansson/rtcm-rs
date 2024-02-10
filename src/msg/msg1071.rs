use super::msm123_sat::*;

msm_sig_frag!(
    id: msg1071_sig,
    type_name: Msg1071Sig,
    gnss: gps,
    fields: [(gnss_signal_fine_pseudorange_ms, df400)],
);

msm_data_seg_frag!(
    id: msg1071_data,
    type_name: Msg1071Data,
    gnss: gps,
    sat_id: msm123_sat,
    sig_id: msg1071_sig,
);

msg!(
    id: msg1071,
    type_name: Msg1071T,
    fields: [
        (reference_station_id, df003),
        (gps_epoch_time_ms, df004),
        (msm_multiple_message_flag, df393),
        (issue_of_data_station, df409),
        (reserved_58_7, df001_7bits),
        (clock_steering_ind, df411),
        (external_clock_ind, df412),
        (gnss_smoothing_type_ind, df417),
        (gnss_smoothing_interval_index, df_u3),
        (data_segment, msg1071_data)
    ],
);
