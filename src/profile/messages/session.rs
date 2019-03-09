// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct Session {
    message_index: Option<enums::MessageIndex>,
    timestamp: Option<enums::DateTime>,
    event: Option<enums::Event>,
    event_type: Option<enums::EventType>,
    start_time: Option<enums::DateTime>,
    start_position_lat: Option<i32>,
    start_position_long: Option<i32>,
    sport: Option<enums::Sport>,
    sub_sport: Option<enums::SubSport>,
    total_elapsed_time: Option<u32>,
    total_timer_time: Option<u32>,
    total_distance: Option<u32>,
    total_cycles: Option<u32>,
    total_calories: Option<u16>,
    total_fat_calories: Option<u16>,
    avg_speed: Option<u16>,
    max_speed: Option<u16>,
    avg_heart_rate: Option<u8>,
    max_heart_rate: Option<u8>,
    avg_cadence: Option<u8>,
    max_cadence: Option<u8>,
    avg_power: Option<u16>,
    max_power: Option<u16>,
    total_ascent: Option<u16>,
    total_descent: Option<u16>,
    total_training_effect: Option<u8>,
    first_lap_index: Option<u16>,
    num_laps: Option<u16>,
    event_group: Option<u8>,
    trigger: Option<enums::SessionTrigger>,
    nec_lat: Option<i32>,
    nec_long: Option<i32>,
    swc_lat: Option<i32>,
    swc_long: Option<i32>,
    normalized_power: Option<u16>,
    training_stress_score: Option<u16>,
    intensity_factor: Option<u16>,
    left_right_balance: Option<enums::LeftRightBalance100>,
    avg_stroke_count: Option<u32>,
    avg_stroke_distance: Option<u16>,
    swim_stroke: Option<enums::SwimStroke>,
    pool_length: Option<u16>,
    threshold_power: Option<u16>,
    pool_length_unit: Option<enums::DisplayMeasure>,
    num_active_lengths: Option<u16>,
    total_work: Option<u32>,
    avg_altitude: Option<u16>,
    max_altitude: Option<u16>,
    gps_accuracy: Option<u8>,
    avg_grade: Option<i16>,
    avg_pos_grade: Option<i16>,
    avg_neg_grade: Option<i16>,
    max_pos_grade: Option<i16>,
    max_neg_grade: Option<i16>,
    avg_temperature: Option<i8>,
    max_temperature: Option<i8>,
    total_moving_time: Option<u32>,
    avg_pos_vertical_speed: Option<i16>,
    avg_neg_vertical_speed: Option<i16>,
    max_pos_vertical_speed: Option<i16>,
    max_neg_vertical_speed: Option<i16>,
    min_heart_rate: Option<u8>,
    time_in_hr_zone: Option<Vec<u32>>,
    time_in_speed_zone: Option<Vec<u32>>,
    time_in_cadence_zone: Option<Vec<u32>>,
    time_in_power_zone: Option<Vec<u32>>,
    avg_lap_time: Option<u32>,
    best_lap_index: Option<u16>,
    min_altitude: Option<u16>,
    player_score: Option<u16>,
    opponent_score: Option<u16>,
    opponent_name: Option<String>,
    stroke_count: Option<Vec<u16>>,
    zone_count: Option<Vec<u16>>,
    max_ball_speed: Option<u16>,
    avg_ball_speed: Option<u16>,
    avg_vertical_oscillation: Option<u16>,
    avg_stance_time_percent: Option<u16>,
    avg_stance_time: Option<u16>,
    avg_fractional_cadence: Option<u8>,
    max_fractional_cadence: Option<u8>,
    total_fractional_cycles: Option<u8>,
    avg_total_hemoglobin_conc: Option<Vec<u16>>,
    min_total_hemoglobin_conc: Option<Vec<u16>>,
    max_total_hemoglobin_conc: Option<Vec<u16>>,
    avg_saturated_hemoglobin_percent: Option<Vec<u16>>,
    min_saturated_hemoglobin_percent: Option<Vec<u16>>,
    max_saturated_hemoglobin_percent: Option<Vec<u16>>,
    avg_left_torque_effectiveness: Option<u8>,
    avg_right_torque_effectiveness: Option<u8>,
    avg_left_pedal_smoothness: Option<u8>,
    avg_right_pedal_smoothness: Option<u8>,
    avg_combined_pedal_smoothness: Option<u8>,
    sport_index: Option<u8>,
    time_standing: Option<u32>,
    stand_count: Option<u16>,
    avg_left_pco: Option<i8>,
    avg_right_pco: Option<i8>,
    avg_left_power_phase: Option<Vec<u8>>,
    avg_left_power_phase_peak: Option<Vec<u8>>,
    avg_right_power_phase: Option<Vec<u8>>,
    avg_right_power_phase_peak: Option<Vec<u8>>,
    avg_power_position: Option<Vec<u16>>,
    max_power_position: Option<Vec<u16>>,
    avg_cadence_position: Option<Vec<u8>>,
    max_cadence_position: Option<Vec<u8>>,
    enhanced_avg_speed: Option<u32>,
    enhanced_max_speed: Option<u32>,
    enhanced_avg_altitude: Option<u32>,
    enhanced_min_altitude: Option<u32>,
    enhanced_max_altitude: Option<u32>,
    avg_lev_motor_power: Option<u16>,
    max_lev_motor_power: Option<u16>,
    lev_battery_consumption: Option<u8>,
    avg_vertical_ratio: Option<u16>,
    avg_stance_time_balance: Option<u16>,
    avg_step_length: Option<u16>,
    total_anaerobic_training_effect: Option<u8>,
    avg_vam: Option<u16>,
}

impl From<Vec<(u8, Field)>> for Session {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.event = field.one().map(<enums::Event>::from),
                1 => msg.event_type = field.one().map(<enums::EventType>::from),
                2 => msg.start_time = field.one().map(<enums::DateTime>::from),
                3 => msg.start_position_lat = field.one().map(<i32>::from),
                4 => msg.start_position_long = field.one().map(<i32>::from),
                5 => msg.sport = field.one().map(<enums::Sport>::from),
                6 => msg.sub_sport = field.one().map(<enums::SubSport>::from),
                7 => msg.total_elapsed_time = field.one().map(<u32>::from),
                8 => msg.total_timer_time = field.one().map(<u32>::from),
                9 => msg.total_distance = field.one().map(<u32>::from),
                10 => msg.total_cycles = field.one().map(<u32>::from),
                11 => msg.total_calories = field.one().map(<u16>::from),
                13 => msg.total_fat_calories = field.one().map(<u16>::from),
                14 => msg.avg_speed = field.one().map(<u16>::from),
                15 => msg.max_speed = field.one().map(<u16>::from),
                16 => msg.avg_heart_rate = field.one().map(<u8>::from),
                17 => msg.max_heart_rate = field.one().map(<u8>::from),
                18 => msg.avg_cadence = field.one().map(<u8>::from),
                19 => msg.max_cadence = field.one().map(<u8>::from),
                20 => msg.avg_power = field.one().map(<u16>::from),
                21 => msg.max_power = field.one().map(<u16>::from),
                22 => msg.total_ascent = field.one().map(<u16>::from),
                23 => msg.total_descent = field.one().map(<u16>::from),
                24 => msg.total_training_effect = field.one().map(<u8>::from),
                25 => msg.first_lap_index = field.one().map(<u16>::from),
                26 => msg.num_laps = field.one().map(<u16>::from),
                27 => msg.event_group = field.one().map(<u8>::from),
                28 => msg.trigger = field.one().map(<enums::SessionTrigger>::from),
                29 => msg.nec_lat = field.one().map(<i32>::from),
                30 => msg.nec_long = field.one().map(<i32>::from),
                31 => msg.swc_lat = field.one().map(<i32>::from),
                32 => msg.swc_long = field.one().map(<i32>::from),
                34 => msg.normalized_power = field.one().map(<u16>::from),
                35 => msg.training_stress_score = field.one().map(<u16>::from),
                36 => msg.intensity_factor = field.one().map(<u16>::from),
                37 => msg.left_right_balance = field.one().map(<enums::LeftRightBalance100>::from),
                41 => msg.avg_stroke_count = field.one().map(<u32>::from),
                42 => msg.avg_stroke_distance = field.one().map(<u16>::from),
                43 => msg.swim_stroke = field.one().map(<enums::SwimStroke>::from),
                44 => msg.pool_length = field.one().map(<u16>::from),
                45 => msg.threshold_power = field.one().map(<u16>::from),
                46 => msg.pool_length_unit = field.one().map(<enums::DisplayMeasure>::from),
                47 => msg.num_active_lengths = field.one().map(<u16>::from),
                48 => msg.total_work = field.one().map(<u32>::from),
                49 => msg.avg_altitude = field.one().map(<u16>::from),
                50 => msg.max_altitude = field.one().map(<u16>::from),
                51 => msg.gps_accuracy = field.one().map(<u8>::from),
                52 => msg.avg_grade = field.one().map(<i16>::from),
                53 => msg.avg_pos_grade = field.one().map(<i16>::from),
                54 => msg.avg_neg_grade = field.one().map(<i16>::from),
                55 => msg.max_pos_grade = field.one().map(<i16>::from),
                56 => msg.max_neg_grade = field.one().map(<i16>::from),
                57 => msg.avg_temperature = field.one().map(<i8>::from),
                58 => msg.max_temperature = field.one().map(<i8>::from),
                59 => msg.total_moving_time = field.one().map(<u32>::from),
                60 => msg.avg_pos_vertical_speed = field.one().map(<i16>::from),
                61 => msg.avg_neg_vertical_speed = field.one().map(<i16>::from),
                62 => msg.max_pos_vertical_speed = field.one().map(<i16>::from),
                63 => msg.max_neg_vertical_speed = field.one().map(<i16>::from),
                64 => msg.min_heart_rate = field.one().map(<u8>::from),
                65 => msg.time_in_hr_zone = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                66 => msg.time_in_speed_zone = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                67 => msg.time_in_cadence_zone = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                68 => msg.time_in_power_zone = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                69 => msg.avg_lap_time = field.one().map(<u32>::from),
                70 => msg.best_lap_index = field.one().map(<u16>::from),
                71 => msg.min_altitude = field.one().map(<u16>::from),
                82 => msg.player_score = field.one().map(<u16>::from),
                83 => msg.opponent_score = field.one().map(<u16>::from),
                84 => msg.opponent_name = field.one().map(<String>::from),
                85 => msg.stroke_count = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                86 => msg.zone_count = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                87 => msg.max_ball_speed = field.one().map(<u16>::from),
                88 => msg.avg_ball_speed = field.one().map(<u16>::from),
                89 => msg.avg_vertical_oscillation = field.one().map(<u16>::from),
                90 => msg.avg_stance_time_percent = field.one().map(<u16>::from),
                91 => msg.avg_stance_time = field.one().map(<u16>::from),
                92 => msg.avg_fractional_cadence = field.one().map(<u8>::from),
                93 => msg.max_fractional_cadence = field.one().map(<u8>::from),
                94 => msg.total_fractional_cycles = field.one().map(<u8>::from),
                95 => msg.avg_total_hemoglobin_conc = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                96 => msg.min_total_hemoglobin_conc = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                97 => msg.max_total_hemoglobin_conc = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                98 => msg.avg_saturated_hemoglobin_percent = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                99 => msg.min_saturated_hemoglobin_percent = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                100 => msg.max_saturated_hemoglobin_percent = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                101 => msg.avg_left_torque_effectiveness = field.one().map(<u8>::from),
                102 => msg.avg_right_torque_effectiveness = field.one().map(<u8>::from),
                103 => msg.avg_left_pedal_smoothness = field.one().map(<u8>::from),
                104 => msg.avg_right_pedal_smoothness = field.one().map(<u8>::from),
                105 => msg.avg_combined_pedal_smoothness = field.one().map(<u8>::from),
                111 => msg.sport_index = field.one().map(<u8>::from),
                112 => msg.time_standing = field.one().map(<u32>::from),
                113 => msg.stand_count = field.one().map(<u16>::from),
                114 => msg.avg_left_pco = field.one().map(<i8>::from),
                115 => msg.avg_right_pco = field.one().map(<i8>::from),
                116 => msg.avg_left_power_phase = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                117 => msg.avg_left_power_phase_peak = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                118 => msg.avg_right_power_phase = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                119 => msg.avg_right_power_phase_peak = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                120 => msg.avg_power_position = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                121 => msg.max_power_position = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                122 => msg.avg_cadence_position = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                123 => msg.max_cadence_position = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                124 => msg.enhanced_avg_speed = field.one().map(<u32>::from),
                125 => msg.enhanced_max_speed = field.one().map(<u32>::from),
                126 => msg.enhanced_avg_altitude = field.one().map(<u32>::from),
                127 => msg.enhanced_min_altitude = field.one().map(<u32>::from),
                128 => msg.enhanced_max_altitude = field.one().map(<u32>::from),
                129 => msg.avg_lev_motor_power = field.one().map(<u16>::from),
                130 => msg.max_lev_motor_power = field.one().map(<u16>::from),
                131 => msg.lev_battery_consumption = field.one().map(<u8>::from),
                132 => msg.avg_vertical_ratio = field.one().map(<u16>::from),
                133 => msg.avg_stance_time_balance = field.one().map(<u16>::from),
                134 => msg.avg_step_length = field.one().map(<u16>::from),
                137 => msg.total_anaerobic_training_effect = field.one().map(<u8>::from),
                139 => msg.avg_vam = field.one().map(<u16>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}
