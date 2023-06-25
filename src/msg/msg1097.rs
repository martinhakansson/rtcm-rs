use super::msm57_sat::*;

msm_sig_frag!(
    id: msg1097_sig,
    type_name: Msg1097Sig,
    gnss: gal,
    fields: [
        (gnss_signal_fine_pseudorange_ext_ms,df405),
        (gnss_signal_fine_phaserange_ext_ms,df406),
        (gnss_phaserange_lock_time_ext_ind,df407),
        (half_cycle_ambiguity_ind,df420),
        (gnss_signal_cnr_ext_dbhz,df408),
        (gnss_signal_fine_phaserange_rate_m_s,df404)
    ],
);

msm_data_seg_frag!(
    id: msg1097_data,
    type_name: Msg1097Data,
    gnss: gal,
    sat_id: msm57_sat,
    sig_id: msg1097_sig,
);

msg!(
    id: msg1097,
    type_name: Msg1097T,
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
        (data_segment, msg1097_data)
    ],
);
