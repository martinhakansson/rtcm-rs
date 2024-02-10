use super::msm123_sat::*;

msm_sig_frag!(
    id: msg1111_sig,
    type_name: Msg1111Sig,
    gnss: qzss,
    fields: [(gnss_signal_fine_pseudorange_ms, df400)],
);

msm_data_seg_frag!(
    id: msg1111_data,
    type_name: Msg1111Data,
    gnss: qzss,
    sat_id: msm123_sat,
    sig_id: msg1111_sig,
);

msg!(
    id: msg1111,
    type_name: Msg1111T,
    fields: [
        (reference_station_id, df003),
        (qzss_epoch_time_ms, df428),
        (msm_multiple_message_flag, df393),
        (issue_of_data_station, df409),
        (reserved_58_7, df001_7bits),
        (clock_steering_ind, df411),
        (external_clock_ind, df412),
        (gnss_smoothing_type_ind, df417),
        (gnss_smoothing_interval_index, df_u3),
        (data_segment, msg1111_data)
    ],
);
