// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct Session {
    #[serde(skip_serializing_if = "Option::is_none")]
    avg_altitude: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_ball_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_cadence: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_cadence_position: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_combined_pedal_smoothness: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_fractional_cadence: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_grade: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_heart_rate: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_lap_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_left_pco: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_left_pedal_smoothness: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_left_power_phase: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_left_power_phase_peak: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_left_torque_effectiveness: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_lev_motor_power: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_neg_grade: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_neg_vertical_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_pos_grade: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_pos_vertical_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_power: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_power_position: Option<Vec<u16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_right_pco: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_right_pedal_smoothness: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_right_power_phase: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_right_power_phase_peak: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_right_torque_effectiveness: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_saturated_hemoglobin_percent: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_stance_time: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_stance_time_balance: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_stance_time_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_step_length: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_stroke_count: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_stroke_distance: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_temperature: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_total_hemoglobin_conc: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_vam: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_vertical_oscillation: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_vertical_ratio: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    best_lap_index: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    enhanced_avg_altitude: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    enhanced_avg_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    enhanced_max_altitude: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    enhanced_max_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    enhanced_min_altitude: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event: Option<crate::profile::enums::Event>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event_group: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event_type: Option<crate::profile::enums::EventType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    first_lap_index: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    gps_accuracy: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    intensity_factor: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left_right_balance: Option<crate::profile::enums::LeftRightBalance100>,

    #[serde(skip_serializing_if = "Option::is_none")]
    lev_battery_consumption: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_altitude: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_ball_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_cadence: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_cadence_position: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_fractional_cadence: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_heart_rate: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_lev_motor_power: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_neg_grade: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_neg_vertical_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_pos_grade: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_pos_vertical_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_power: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_power_position: Option<Vec<u16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_saturated_hemoglobin_percent: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_temperature: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_total_hemoglobin_conc: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_altitude: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_heart_rate: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_saturated_hemoglobin_percent: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_total_hemoglobin_conc: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    nec_lat: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    nec_long: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    normalized_power: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    num_active_lengths: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    num_laps: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    opponent_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    opponent_score: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    player_score: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pool_length: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pool_length_unit: Option<crate::profile::enums::DisplayMeasure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sport: Option<crate::profile::enums::Sport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sport_index: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stand_count: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_position_lat: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_position_long: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stroke_count: Option<Vec<u16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sub_sport: Option<crate::profile::enums::SubSport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    swc_lat: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    swc_long: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    swim_stroke: Option<crate::profile::enums::SwimStroke>,

    #[serde(skip_serializing_if = "Option::is_none")]
    threshold_power: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_in_cadence_zone: Option<Vec<crate::fields::Time>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_in_hr_zone: Option<Vec<crate::fields::Time>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_in_power_zone: Option<Vec<crate::fields::Time>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_in_speed_zone: Option<Vec<crate::fields::Time>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_standing: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_anaerobic_training_effect: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_ascent: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_calories: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_cycles: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_descent: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_distance: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_elapsed_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_fat_calories: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_fractional_cycles: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_moving_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_timer_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_training_effect: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_work: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    training_stress_score: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    trigger: Option<crate::profile::enums::SessionTrigger>,

    #[serde(skip_serializing_if = "Option::is_none")]
    zone_count: Option<Vec<u16>>,
}

impl Session {
    pub fn from_fields<Order, Reader>(reader: &mut Reader, fields: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut msg: Self = Default::default();
        for field in fields {
            let (number, content) = field.content_from::<Order, Reader>(reader)?;
            match number {
                0 => msg.event = content.one().map(<crate::profile::enums::Event>::from),
                1 => msg.event_type = content.one().map(<crate::profile::enums::EventType>::from),
                2 => msg.start_time = content.one().map(<crate::fields::DateTime>::from),
                3 => msg.start_position_lat = content.one().map(<i32>::from),
                4 => msg.start_position_long = content.one().map(<i32>::from),
                5 => msg.sport = content.one().map(<crate::profile::enums::Sport>::from),
                6 => msg.sub_sport = content.one().map(<crate::profile::enums::SubSport>::from),
                7 => msg.total_elapsed_time = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                8 => msg.total_timer_time = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                9 => msg.total_distance = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u32>::from(v)) / 100.0 - 0.0 })(v))),
                10 => msg.total_cycles = content.one().map(<u32>::from),
                11 => msg.total_calories = content.one().map(<u16>::from),
                13 => msg.total_fat_calories = content.one().map(<u16>::from),
                14 => msg.avg_speed = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 1000.0 - 0.0 }),
                15 => msg.max_speed = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 1000.0 - 0.0 }),
                16 => msg.avg_heart_rate = content.one().map(<u8>::from),
                17 => msg.max_heart_rate = content.one().map(<u8>::from),
                18 => msg.avg_cadence = content.one().map(<u8>::from),
                19 => msg.max_cadence = content.one().map(<u8>::from),
                20 => msg.avg_power = content.one().map(<u16>::from),
                21 => msg.max_power = content.one().map(<u16>::from),
                22 => msg.total_ascent = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, u16>((<u16>::from)(v))),
                23 => msg.total_descent = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, u16>((<u16>::from)(v))),
                24 => msg.total_training_effect = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 10.0 - 0.0 }),
                25 => msg.first_lap_index = content.one().map(<u16>::from),
                26 => msg.num_laps = content.one().map(<u16>::from),
                27 => msg.event_group = content.one().map(<u8>::from),
                28 => msg.trigger = content.one().map(<crate::profile::enums::SessionTrigger>::from),
                29 => msg.nec_lat = content.one().map(<i32>::from),
                30 => msg.nec_long = content.one().map(<i32>::from),
                31 => msg.swc_lat = content.one().map(<i32>::from),
                32 => msg.swc_long = content.one().map(<i32>::from),
                34 => msg.normalized_power = content.one().map(<u16>::from),
                35 => msg.training_stress_score = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 10.0 - 0.0 }),
                36 => msg.intensity_factor = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 1000.0 - 0.0 }),
                37 => msg.left_right_balance = content.one().map(<crate::profile::enums::LeftRightBalance100>::from),
                41 => msg.avg_stroke_count = content.one().map(|v| { <f64>::from(<u32>::from(v)) / 10.0 - 0.0 }),
                42 => msg.avg_stroke_distance = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 })(v))),
                43 => msg.swim_stroke = content.one().map(<crate::profile::enums::SwimStroke>::from),
                44 => msg.pool_length = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 })(v))),
                45 => msg.threshold_power = content.one().map(<u16>::from),
                46 => msg.pool_length_unit = content.one().map(<crate::profile::enums::DisplayMeasure>::from),
                47 => msg.num_active_lengths = content.one().map(<u16>::from),
                48 => msg.total_work = content.one().map(<u32>::from),
                49 => msg.avg_altitude = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u16>::from(v)) / 5.0 - 500.0 })(v))),
                50 => msg.max_altitude = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u16>::from(v)) / 5.0 - 500.0 })(v))),
                51 => msg.gps_accuracy = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, u8>((<u8>::from)(v))),
                52 => msg.avg_grade = content.one().map(|v| { <f64>::from(<i16>::from(v)) / 100.0 - 0.0 }),
                53 => msg.avg_pos_grade = content.one().map(|v| { <f64>::from(<i16>::from(v)) / 100.0 - 0.0 }),
                54 => msg.avg_neg_grade = content.one().map(|v| { <f64>::from(<i16>::from(v)) / 100.0 - 0.0 }),
                55 => msg.max_pos_grade = content.one().map(|v| { <f64>::from(<i16>::from(v)) / 100.0 - 0.0 }),
                56 => msg.max_neg_grade = content.one().map(|v| { <f64>::from(<i16>::from(v)) / 100.0 - 0.0 }),
                57 => msg.avg_temperature = content.one().map(<i8>::from),
                58 => msg.max_temperature = content.one().map(<i8>::from),
                59 => msg.total_moving_time = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                60 => msg.avg_pos_vertical_speed = content.one().map(|v| { <f64>::from(<i16>::from(v)) / 1000.0 - 0.0 }),
                61 => msg.avg_neg_vertical_speed = content.one().map(|v| { <f64>::from(<i16>::from(v)) / 1000.0 - 0.0 }),
                62 => msg.max_pos_vertical_speed = content.one().map(|v| { <f64>::from(<i16>::from(v)) / 1000.0 - 0.0 }),
                63 => msg.max_neg_vertical_speed = content.one().map(|v| { <f64>::from(<i16>::from(v)) / 1000.0 - 0.0 }),
                64 => msg.min_heart_rate = content.one().map(<u8>::from),
                65 => msg.time_in_hr_zone = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))).collect()),
                66 => msg.time_in_speed_zone = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))).collect()),
                67 => msg.time_in_cadence_zone = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))).collect()),
                68 => msg.time_in_power_zone = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))).collect()),
                69 => msg.avg_lap_time = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                70 => msg.best_lap_index = content.one().map(<u16>::from),
                71 => msg.min_altitude = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u16>::from(v)) / 5.0 - 500.0 })(v))),
                82 => msg.player_score = content.one().map(<u16>::from),
                83 => msg.opponent_score = content.one().map(<u16>::from),
                84 => msg.opponent_name = content.one().map(<String>::from),
                85 => msg.stroke_count = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                86 => msg.zone_count = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                87 => msg.max_ball_speed = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 }),
                88 => msg.avg_ball_speed = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 }),
                89 => msg.avg_vertical_oscillation = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 10.0 - 0.0 }),
                90 => msg.avg_stance_time_percent = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 }),
                91 => msg.avg_stance_time = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 10.0 - 0.0 }),
                92 => msg.avg_fractional_cadence = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 128.0 - 0.0 }),
                93 => msg.max_fractional_cadence = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 128.0 - 0.0 }),
                94 => msg.total_fractional_cycles = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 128.0 - 0.0 }),
                95 => msg.avg_total_hemoglobin_conc = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 }).collect()),
                96 => msg.min_total_hemoglobin_conc = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 }).collect()),
                97 => msg.max_total_hemoglobin_conc = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 }).collect()),
                98 => msg.avg_saturated_hemoglobin_percent = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u16>::from(v)) / 10.0 - 0.0 }).collect()),
                99 => msg.min_saturated_hemoglobin_percent = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u16>::from(v)) / 10.0 - 0.0 }).collect()),
                100 => msg.max_saturated_hemoglobin_percent = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u16>::from(v)) / 10.0 - 0.0 }).collect()),
                101 => msg.avg_left_torque_effectiveness = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 2.0 - 0.0 }),
                102 => msg.avg_right_torque_effectiveness = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 2.0 - 0.0 }),
                103 => msg.avg_left_pedal_smoothness = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 2.0 - 0.0 }),
                104 => msg.avg_right_pedal_smoothness = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 2.0 - 0.0 }),
                105 => msg.avg_combined_pedal_smoothness = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 2.0 - 0.0 }),
                111 => msg.sport_index = content.one().map(<u8>::from),
                112 => msg.time_standing = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                113 => msg.stand_count = content.one().map(<u16>::from),
                114 => msg.avg_left_pco = content.one().map(<i8>::from),
                115 => msg.avg_right_pco = content.one().map(<i8>::from),
                116 => msg.avg_left_power_phase = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u8>::from(v)) / 0.0 - 0.0 }).collect()),
                117 => msg.avg_left_power_phase_peak = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u8>::from(v)) / 0.0 - 0.0 }).collect()),
                118 => msg.avg_right_power_phase = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u8>::from(v)) / 0.0 - 0.0 }).collect()),
                119 => msg.avg_right_power_phase_peak = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u8>::from(v)) / 0.0 - 0.0 }).collect()),
                120 => msg.avg_power_position = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                121 => msg.max_power_position = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                122 => msg.avg_cadence_position = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                123 => msg.max_cadence_position = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                124 => msg.enhanced_avg_speed = content.one().map(|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 }),
                125 => msg.enhanced_max_speed = content.one().map(|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 }),
                126 => msg.enhanced_avg_altitude = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u32>::from(v)) / 5.0 - 500.0 })(v))),
                127 => msg.enhanced_min_altitude = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u32>::from(v)) / 5.0 - 500.0 })(v))),
                128 => msg.enhanced_max_altitude = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u32>::from(v)) / 5.0 - 500.0 })(v))),
                129 => msg.avg_lev_motor_power = content.one().map(<u16>::from),
                130 => msg.max_lev_motor_power = content.one().map(<u16>::from),
                131 => msg.lev_battery_consumption = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 2.0 - 0.0 }),
                132 => msg.avg_vertical_ratio = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 }),
                133 => msg.avg_stance_time_balance = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 }),
                134 => msg.avg_step_length = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 10.0 - 0.0 }),
                137 => msg.total_anaerobic_training_effect = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 10.0 - 0.0 }),
                139 => msg.avg_vam = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 1000.0 - 0.0 }),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                254 => msg.message_index = content.one().map(<crate::profile::enums::MessageIndex>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
