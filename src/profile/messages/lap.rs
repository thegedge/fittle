// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct Lap {
    #[serde(skip_serializing_if = "Option::is_none")]
    avg_altitude: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_cadence: Option<crate::fields::Frequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_cadence_position: Option<Vec<crate::fields::Frequency>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_combined_pedal_smoothness: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_fractional_cadence: Option<crate::fields::Frequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_grade: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_heart_rate: Option<crate::fields::Frequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_left_pco: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_left_pedal_smoothness: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_left_power_phase: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_left_power_phase_peak: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_left_torque_effectiveness: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_lev_motor_power: Option<crate::fields::Power>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_neg_grade: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_neg_vertical_speed: Option<crate::fields::Velocity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_pos_grade: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_pos_vertical_speed: Option<crate::fields::Velocity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_power: Option<crate::fields::Power>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_power_position: Option<Vec<crate::fields::Power>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_right_pco: Option<crate::fields::Length>,

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
    avg_speed: Option<crate::fields::Velocity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_stance_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_stance_time_balance: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_stance_time_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_step_length: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_stroke_distance: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_temperature: Option<crate::fields::ThermodynamicTemperature>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_total_hemoglobin_conc: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_vam: Option<crate::fields::Velocity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_vertical_oscillation: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_vertical_ratio: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end_position_lat: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end_position_long: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    enhanced_avg_altitude: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    enhanced_avg_speed: Option<crate::fields::Velocity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    enhanced_max_altitude: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    enhanced_max_speed: Option<crate::fields::Velocity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    enhanced_min_altitude: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event: Option<crate::profile::enums::Event>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event_group: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event_type: Option<crate::profile::enums::EventType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    first_length_index: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    gps_accuracy: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    intensity: Option<crate::profile::enums::Intensity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    lap_trigger: Option<crate::profile::enums::LapTrigger>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left_right_balance: Option<crate::profile::enums::LeftRightBalance100>,

    #[serde(skip_serializing_if = "Option::is_none")]
    lev_battery_consumption: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_altitude: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_cadence: Option<crate::fields::Frequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_cadence_position: Option<Vec<crate::fields::Frequency>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_fractional_cadence: Option<crate::fields::Frequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_heart_rate: Option<crate::fields::Frequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_lev_motor_power: Option<crate::fields::Power>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_neg_grade: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_neg_vertical_speed: Option<crate::fields::Velocity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_pos_grade: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_pos_vertical_speed: Option<crate::fields::Velocity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_power: Option<crate::fields::Power>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_power_position: Option<Vec<crate::fields::Power>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_saturated_hemoglobin_percent: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_speed: Option<crate::fields::Velocity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_temperature: Option<crate::fields::ThermodynamicTemperature>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_total_hemoglobin_conc: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_altitude: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_heart_rate: Option<crate::fields::Frequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_saturated_hemoglobin_percent: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_total_hemoglobin_conc: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    normalized_power: Option<crate::fields::Power>,

    #[serde(skip_serializing_if = "Option::is_none")]
    num_active_lengths: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    num_lengths: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    opponent_score: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    player_score: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    repetition_num: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sport: Option<crate::profile::enums::Sport>,

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
    swim_stroke: Option<crate::profile::enums::SwimStroke>,

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
    total_ascent: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_calories: Option<crate::fields::Energy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_cycles: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_descent: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_distance: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_elapsed_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_fat_calories: Option<crate::fields::Energy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_fractional_cycles: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_moving_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_timer_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_work: Option<crate::fields::Energy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    wkt_step_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    zone_count: Option<Vec<u16>>,
}

impl Lap {
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
                5 => msg.end_position_lat = content.one().map(<i32>::from),
                6 => msg.end_position_long = content.one().map(<i32>::from),
                7 => msg.total_elapsed_time = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                8 => msg.total_timer_time = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                9 => msg.total_distance = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u32>::from(v)) / 100.0 - 0.0 })(v))),
                10 => msg.total_cycles = content.one().map(<u32>::from),
                11 => msg.total_calories = content.one().map(|v| crate::fields::Energy::new::<uom::si::energy::kilocalorie, u16>((<u16>::from)(v))),
                12 => msg.total_fat_calories = content.one().map(|v| crate::fields::Energy::new::<uom::si::energy::kilocalorie, u16>((<u16>::from)(v))),
                13 => msg.avg_speed = content.one().map(|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { <f64>::from(<u16>::from(v)) / 1000.0 - 0.0 })(v))),
                14 => msg.max_speed = content.one().map(|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { <f64>::from(<u16>::from(v)) / 1000.0 - 0.0 })(v))),
                15 => msg.avg_heart_rate = content.one().map(|v| crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>((<u8>::from)(v))),
                16 => msg.max_heart_rate = content.one().map(|v| crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>((<u8>::from)(v))),
                17 => msg.avg_cadence = content.one().map(|v| crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>((<u8>::from)(v))),
                18 => msg.max_cadence = content.one().map(|v| crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>((<u8>::from)(v))),
                19 => msg.avg_power = content.one().map(|v| crate::fields::Power::new::<uom::si::power::watt, u16>((<u16>::from)(v))),
                20 => msg.max_power = content.one().map(|v| crate::fields::Power::new::<uom::si::power::watt, u16>((<u16>::from)(v))),
                21 => msg.total_ascent = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, u16>((<u16>::from)(v))),
                22 => msg.total_descent = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, u16>((<u16>::from)(v))),
                23 => msg.intensity = content.one().map(<crate::profile::enums::Intensity>::from),
                24 => msg.lap_trigger = content.one().map(<crate::profile::enums::LapTrigger>::from),
                25 => msg.sport = content.one().map(<crate::profile::enums::Sport>::from),
                26 => msg.event_group = content.one().map(<u8>::from),
                32 => msg.num_lengths = content.one().map(<u16>::from),
                33 => msg.normalized_power = content.one().map(|v| crate::fields::Power::new::<uom::si::power::watt, u16>((<u16>::from)(v))),
                34 => msg.left_right_balance = content.one().map(<crate::profile::enums::LeftRightBalance100>::from),
                35 => msg.first_length_index = content.one().map(<u16>::from),
                37 => msg.avg_stroke_distance = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 })(v))),
                38 => msg.swim_stroke = content.one().map(<crate::profile::enums::SwimStroke>::from),
                39 => msg.sub_sport = content.one().map(<crate::profile::enums::SubSport>::from),
                40 => msg.num_active_lengths = content.one().map(<u16>::from),
                41 => msg.total_work = content.one().map(|v| crate::fields::Energy::new::<uom::si::energy::joule, u32>((<u32>::from)(v))),
                42 => msg.avg_altitude = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u16>::from(v)) / 5.0 - 500.0 })(v))),
                43 => msg.max_altitude = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u16>::from(v)) / 5.0 - 500.0 })(v))),
                44 => msg.gps_accuracy = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, u8>((<u8>::from)(v))),
                45 => msg.avg_grade = content.one().map(|v| { <f64>::from(<i16>::from(v)) / 100.0 - 0.0 }),
                46 => msg.avg_pos_grade = content.one().map(|v| { <f64>::from(<i16>::from(v)) / 100.0 - 0.0 }),
                47 => msg.avg_neg_grade = content.one().map(|v| { <f64>::from(<i16>::from(v)) / 100.0 - 0.0 }),
                48 => msg.max_pos_grade = content.one().map(|v| { <f64>::from(<i16>::from(v)) / 100.0 - 0.0 }),
                49 => msg.max_neg_grade = content.one().map(|v| { <f64>::from(<i16>::from(v)) / 100.0 - 0.0 }),
                50 => msg.avg_temperature = content.one().map(|v| crate::fields::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::degree_celsius, i8>((<i8>::from)(v))),
                51 => msg.max_temperature = content.one().map(|v| crate::fields::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::degree_celsius, i8>((<i8>::from)(v))),
                52 => msg.total_moving_time = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                53 => msg.avg_pos_vertical_speed = content.one().map(|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { <f64>::from(<i16>::from(v)) / 1000.0 - 0.0 })(v))),
                54 => msg.avg_neg_vertical_speed = content.one().map(|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { <f64>::from(<i16>::from(v)) / 1000.0 - 0.0 })(v))),
                55 => msg.max_pos_vertical_speed = content.one().map(|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { <f64>::from(<i16>::from(v)) / 1000.0 - 0.0 })(v))),
                56 => msg.max_neg_vertical_speed = content.one().map(|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { <f64>::from(<i16>::from(v)) / 1000.0 - 0.0 })(v))),
                57 => msg.time_in_hr_zone = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))).collect()),
                58 => msg.time_in_speed_zone = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))).collect()),
                59 => msg.time_in_cadence_zone = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))).collect()),
                60 => msg.time_in_power_zone = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))).collect()),
                61 => msg.repetition_num = content.one().map(<u16>::from),
                62 => msg.min_altitude = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u16>::from(v)) / 5.0 - 500.0 })(v))),
                63 => msg.min_heart_rate = content.one().map(|v| crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>((<u8>::from)(v))),
                71 => msg.wkt_step_index = content.one().map(<crate::profile::enums::MessageIndex>::from),
                74 => msg.opponent_score = content.one().map(<u16>::from),
                75 => msg.stroke_count = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                76 => msg.zone_count = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                77 => msg.avg_vertical_oscillation = content.one().map(|v| crate::fields::Length::new::<uom::si::length::millimeter, f64>((|v| { <f64>::from(<u16>::from(v)) / 10.0 - 0.0 })(v))),
                78 => msg.avg_stance_time_percent = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 }),
                79 => msg.avg_stance_time = content.one().map(|v| crate::fields::Time::new::<uom::si::time::millisecond, f64>((|v| { <f64>::from(<u16>::from(v)) / 10.0 - 0.0 })(v))),
                80 => msg.avg_fractional_cadence = content.one().map(|v| crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, f64>((|v| { <f64>::from(<u8>::from(v)) / 128.0 - 0.0 })(v))),
                81 => msg.max_fractional_cadence = content.one().map(|v| crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, f64>((|v| { <f64>::from(<u8>::from(v)) / 128.0 - 0.0 })(v))),
                82 => msg.total_fractional_cycles = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 128.0 - 0.0 }),
                83 => msg.player_score = content.one().map(<u16>::from),
                84 => msg.avg_total_hemoglobin_conc = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 }).collect()),
                85 => msg.min_total_hemoglobin_conc = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 }).collect()),
                86 => msg.max_total_hemoglobin_conc = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 }).collect()),
                87 => msg.avg_saturated_hemoglobin_percent = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u16>::from(v)) / 10.0 - 0.0 }).collect()),
                88 => msg.min_saturated_hemoglobin_percent = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u16>::from(v)) / 10.0 - 0.0 }).collect()),
                89 => msg.max_saturated_hemoglobin_percent = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u16>::from(v)) / 10.0 - 0.0 }).collect()),
                91 => msg.avg_left_torque_effectiveness = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 2.0 - 0.0 }),
                92 => msg.avg_right_torque_effectiveness = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 2.0 - 0.0 }),
                93 => msg.avg_left_pedal_smoothness = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 2.0 - 0.0 }),
                94 => msg.avg_right_pedal_smoothness = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 2.0 - 0.0 }),
                95 => msg.avg_combined_pedal_smoothness = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 2.0 - 0.0 }),
                98 => msg.time_standing = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                99 => msg.stand_count = content.one().map(<u16>::from),
                100 => msg.avg_left_pco = content.one().map(|v| crate::fields::Length::new::<uom::si::length::millimeter, i8>((<i8>::from)(v))),
                101 => msg.avg_right_pco = content.one().map(|v| crate::fields::Length::new::<uom::si::length::millimeter, i8>((<i8>::from)(v))),
                102 => msg.avg_left_power_phase = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u8>::from(v)) / 0.0 - 0.0 }).collect()),
                103 => msg.avg_left_power_phase_peak = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u8>::from(v)) / 0.0 - 0.0 }).collect()),
                104 => msg.avg_right_power_phase = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u8>::from(v)) / 0.0 - 0.0 }).collect()),
                105 => msg.avg_right_power_phase_peak = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u8>::from(v)) / 0.0 - 0.0 }).collect()),
                106 => msg.avg_power_position = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Power::new::<uom::si::power::watt, u16>((<u16>::from)(v))).collect()),
                107 => msg.max_power_position = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Power::new::<uom::si::power::watt, u16>((<u16>::from)(v))).collect()),
                108 => msg.avg_cadence_position = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>((<u8>::from)(v))).collect()),
                109 => msg.max_cadence_position = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>((<u8>::from)(v))).collect()),
                110 => msg.enhanced_avg_speed = content.one().map(|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                111 => msg.enhanced_max_speed = content.one().map(|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                112 => msg.enhanced_avg_altitude = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u32>::from(v)) / 5.0 - 500.0 })(v))),
                113 => msg.enhanced_min_altitude = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u32>::from(v)) / 5.0 - 500.0 })(v))),
                114 => msg.enhanced_max_altitude = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u32>::from(v)) / 5.0 - 500.0 })(v))),
                115 => msg.avg_lev_motor_power = content.one().map(|v| crate::fields::Power::new::<uom::si::power::watt, u16>((<u16>::from)(v))),
                116 => msg.max_lev_motor_power = content.one().map(|v| crate::fields::Power::new::<uom::si::power::watt, u16>((<u16>::from)(v))),
                117 => msg.lev_battery_consumption = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 2.0 - 0.0 }),
                118 => msg.avg_vertical_ratio = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 }),
                119 => msg.avg_stance_time_balance = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 }),
                120 => msg.avg_step_length = content.one().map(|v| crate::fields::Length::new::<uom::si::length::millimeter, f64>((|v| { <f64>::from(<u16>::from(v)) / 10.0 - 0.0 })(v))),
                121 => msg.avg_vam = content.one().map(|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { <f64>::from(<u16>::from(v)) / 1000.0 - 0.0 })(v))),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                254 => msg.message_index = content.one().map(<crate::profile::enums::MessageIndex>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
