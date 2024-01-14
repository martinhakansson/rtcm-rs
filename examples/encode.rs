use rtcm_rs::msg::{Msg1001Sat, Msg1001T};
use rtcm_rs::prelude::*;
use rtcm_rs::util::DataVec;

fn main() {
    let mut message_builder = MessageBuilder::new();
    let result = message_builder.build_message(&Message::Msg1001(Msg1001T {
        reference_station_id: 100,
        gps_epoch_time_ms: 0,
        synchronous_gnss_msg_flag: 0,
        divergence_free_smoothing_flag: 0,
        smoothing_interval_index: 0,
        satellites: {
            let mut satellites = DataVec::new();
            satellites.push(Msg1001Sat {
                gps_satellite_id: 20,
                gps_l1_code_ind: 0,
                l1_pseudorange_m: Some(20000000.0),
                l1_phase_pseudorange_diff_m: Some(0.2),
                l1_lock_time_index: 0,
            });
            satellites.push(Msg1001Sat {
                gps_satellite_id: 21,
                gps_l1_code_ind: 0,
                l1_pseudorange_m: Some(26000000.0),
                l1_phase_pseudorange_diff_m: Some(0.4),
                l1_lock_time_index: 0,
            });
            satellites
        },
    }));
    if let Ok(bytes) = result {
        println!("Encoded message: {:?}", bytes);
    }
}
