use super::msm46_sat::*;

msm_sig_frag!(
    id: msg1114_sig,
    type_name: Msg1114Sig,
    gnss: qzss,
    fields: [
        (gnss_signal_fine_pseudorange_ms,df400),
        (gnss_signal_fine_phaserange_ms,df401),
        (gnss_phaserange_lock_time_ind,df402),
        (half_cycle_ambiguity_ind,df420),
        (gnss_signal_cnr_dbhz,df403)
    ],
);

msm_data_seg_frag!(
    id: msg1114_data,
    type_name: Msg1114Data,
    gnss: qzss,
    sat_id: msm46_sat,
    sig_id: msg1114_sig,
);

msg!(
    id: msg1114,
    type_name: Msg1114T,
    fields: [
        (reference_station_id, df003),
        (qzss_epoch_time_ms, df428),
        (msm_multiple_message_flag, df393),
        (issue_of_data_station, df409),
        (reserved_58_7, df001_7bits),
        (clock_steering_ind, df411),
        (external_clock_ind, df412),
        (gnss_smoothing_type_ind, df417),
        (gnss_smoothing_interval_bitval, df418),
        (data_segment, msg1114_data)
    ],
);
