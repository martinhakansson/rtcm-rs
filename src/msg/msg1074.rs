
use super::msm46_sat::*;

msm_sig_frag!(
    id: msg1074_sig,
    type_name: Msg1074Sig,
    gnss: gps,
    fields: [
        (signal_fine_pseudorange_ms,df400),
        (signal_fine_phaserange_ms,df401),
        (phaserange_lock_time_indicator,df402),
        (half_cycle_ambiguity_indicator,df420),
        (signal_cnr,df403)
    ],
);

msm_data_seg_frag!(
    id: msg1074_data,
    type_name: Msg1074Data,
    gnss: gps,
    sat_id: msm46_sat,
    sig_id: msg1074_sig,
);

msg!(
    id: msg1074,
    type_name: Msg1074T,
    fields: [
        (reference_station_id, df003),
        (epoch_time, df004),
        (multiple_message_bit, df393),
        (issue_of_data_station, df409),
        (reserved, df001_7bits),
        (clock_steering_indicator, df411),
        (external_clock_indicator, df412),
        (divergence_free_smoothing_indicator, df417),
        (smoothing_interval, df418),
        (data_segment, msg1074_data)
    ],
);

