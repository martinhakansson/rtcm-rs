use super::msm123_sat::*;

msm_sig_frag!(
    id: msg1093_sig,
    type_name: Msg1093Sig,
    gnss: gal,
    fields: [
        (gnss_signal_fine_pseudorange_ms,df400),
        (gnss_signal_fine_phaserange_ms,df401),
        (gnss_phaserange_lock_time_ind,df402),
        (half_cycle_ambiguity_ind,df420)
        ],
);

msm_data_seg_frag!(
    id: msg1093_data,
    type_name: Msg1093Data,
    gnss: gal,
    sat_id: msm123_sat,
    sig_id: msg1093_sig,
);

msg!(
    id: msg1093,
    type_name: Msg1093T,
    fields: [
        (reference_station_id, df003),
        (gal_epoch_time_ms, df248),
        (msm_multiple_message_flag, df393),
        (issue_of_data_station, df409),
        (reserved_58_7, df001_7bits),
        (clock_steering_ind, df411),
        (external_clock_ind, df412),
        (gnss_smoothing_type_ind, df417),
        (gnss_smoothing_interval_index, df_u3),
        (data_segment, msg1093_data)
    ],
);
