
use super::msm123_sat::*;

msm_sig_frag!(
    id: msg1071_sig,
    type_name: Msm1071Sig,
    gnss: gps,
    fields: [
        (signal_fine_pseudorange_ms,df400)
    ],
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
        //(message_number, df002),
        (reference_station_id, df003),
        (epoch_time, df004),
        (multiple_message_bit, df393),
        (issue_of_data_station, df409),
        (reserved, df001_7bits),
        (clock_steering_indicator, df411),
        (external_clock_indicator, df412),
        (divergence_free_smoothing_indicator, df417),
        (smoothing_interval, df418),
        (data_segment, msg1071_data)
    ],
);

