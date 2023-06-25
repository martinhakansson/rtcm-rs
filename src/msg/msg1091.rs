use super::msm123_sat::*;

msm_sig_frag!(
    id: msg1091_sig,
    type_name: Msm1091Sig,
    gnss: gal,
    fields: [(gnss_signal_fine_pseudorange_ms, df400)],
);

msm_data_seg_frag!(
    id: msg1091_data,
    type_name: Msg1091Data,
    gnss: gal,
    sat_id: msm123_sat,
    sig_id: msg1091_sig,
);

msg!(
    id: msg1091,
    type_name: Msg1091T,
    fields: [
        (reference_station_id, df003),
        (gal_epoch_time_ms, df248),
        (msm_multiple_message_flag, df393),
        (issue_of_data_station, df409),
        (reserved_58_7, df001_7bits),
        (clock_steering_ind, df411),
        (external_clock_ind, df412),
        (gnss_smoothing_type_ind, df417),
        (gnss_smoothing_interval_bitval, df418),
        (data_segment, msg1091_data)
    ],
);
