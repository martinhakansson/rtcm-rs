use super::msm46_sat::*;

msm_sig_frag!(
    id: msg1086_sig,
    type_name: Msg1086Sig,
    gnss: glo,
    fields: [
        (gnss_signal_fine_pseudorange_ext_ms,df405),
        (gnss_signal_fine_phaserange_ext_ms,df406),
        (gnss_phaserange_lock_time_ext_ind,df407),
        (half_cycle_ambiguity_ind,df420),
        (gnss_signal_cnr_ext_dbhz,df408)
    ],
);

msm_data_seg_frag!(
    id: msg1086_data,
    type_name: Msg1086Data,
    gnss: glo,
    sat_id: msm46_sat,
    sig_id: msg1086_sig,
);

msg!(
    id: msg1086,
    type_name: Msg1086T,
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
        (data_segment, msg1086_data)
    ],
);
