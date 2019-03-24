// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct SegmentLap {
    #[serde(skip_serializing_if = "Option::is_none")]
    active_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_altitude: Option<crate::fields::Length>,

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
    avg_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_temperature: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end_position_lat: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end_position_long: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event: Option<crate::profile::enums::Event>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event_group: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event_type: Option<crate::profile::enums::EventType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    front_gear_shift_count: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    gps_accuracy: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left_right_balance: Option<crate::profile::enums::LeftRightBalance100>,

    #[serde(skip_serializing_if = "Option::is_none")]
    manufacturer: Option<crate::profile::enums::Manufacturer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_altitude: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_cadence: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_cadence_position: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_fractional_cadence: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_heart_rate: Option<u8>,

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
    max_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_temperature: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_altitude: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_heart_rate: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    nec_lat: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    nec_long: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    normalized_power: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rear_gear_shift_count: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    repetition_num: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sport: Option<crate::profile::enums::Sport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sport_event: Option<crate::profile::enums::SportEvent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stand_count: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_position_lat: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_position_long: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<crate::profile::enums::SegmentLapStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sub_sport: Option<crate::profile::enums::SubSport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    swc_lat: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    swc_long: Option<i32>,

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
    total_work: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    uuid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    wkt_step_index: Option<crate::profile::enums::MessageIndex>,
}

impl SegmentLap {
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
                11 => msg.total_calories = content.one().map(<u16>::from),
                12 => msg.total_fat_calories = content.one().map(<u16>::from),
                13 => msg.avg_speed = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 1000.0 - 0.0 }),
                14 => msg.max_speed = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 1000.0 - 0.0 }),
                15 => msg.avg_heart_rate = content.one().map(<u8>::from),
                16 => msg.max_heart_rate = content.one().map(<u8>::from),
                17 => msg.avg_cadence = content.one().map(<u8>::from),
                18 => msg.max_cadence = content.one().map(<u8>::from),
                19 => msg.avg_power = content.one().map(<u16>::from),
                20 => msg.max_power = content.one().map(<u16>::from),
                21 => msg.total_ascent = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, u16>((<u16>::from)(v))),
                22 => msg.total_descent = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, u16>((<u16>::from)(v))),
                23 => msg.sport = content.one().map(<crate::profile::enums::Sport>::from),
                24 => msg.event_group = content.one().map(<u8>::from),
                25 => msg.nec_lat = content.one().map(<i32>::from),
                26 => msg.nec_long = content.one().map(<i32>::from),
                27 => msg.swc_lat = content.one().map(<i32>::from),
                28 => msg.swc_long = content.one().map(<i32>::from),
                29 => msg.name = content.one().map(<String>::from),
                30 => msg.normalized_power = content.one().map(<u16>::from),
                31 => msg.left_right_balance = content.one().map(<crate::profile::enums::LeftRightBalance100>::from),
                32 => msg.sub_sport = content.one().map(<crate::profile::enums::SubSport>::from),
                33 => msg.total_work = content.one().map(<u32>::from),
                34 => msg.avg_altitude = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u16>::from(v)) / 5.0 - 500.0 })(v))),
                35 => msg.max_altitude = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u16>::from(v)) / 5.0 - 500.0 })(v))),
                36 => msg.gps_accuracy = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, u8>((<u8>::from)(v))),
                37 => msg.avg_grade = content.one().map(|v| { <f64>::from(<i16>::from(v)) / 100.0 - 0.0 }),
                38 => msg.avg_pos_grade = content.one().map(|v| { <f64>::from(<i16>::from(v)) / 100.0 - 0.0 }),
                39 => msg.avg_neg_grade = content.one().map(|v| { <f64>::from(<i16>::from(v)) / 100.0 - 0.0 }),
                40 => msg.max_pos_grade = content.one().map(|v| { <f64>::from(<i16>::from(v)) / 100.0 - 0.0 }),
                41 => msg.max_neg_grade = content.one().map(|v| { <f64>::from(<i16>::from(v)) / 100.0 - 0.0 }),
                42 => msg.avg_temperature = content.one().map(<i8>::from),
                43 => msg.max_temperature = content.one().map(<i8>::from),
                44 => msg.total_moving_time = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                45 => msg.avg_pos_vertical_speed = content.one().map(|v| { <f64>::from(<i16>::from(v)) / 1000.0 - 0.0 }),
                46 => msg.avg_neg_vertical_speed = content.one().map(|v| { <f64>::from(<i16>::from(v)) / 1000.0 - 0.0 }),
                47 => msg.max_pos_vertical_speed = content.one().map(|v| { <f64>::from(<i16>::from(v)) / 1000.0 - 0.0 }),
                48 => msg.max_neg_vertical_speed = content.one().map(|v| { <f64>::from(<i16>::from(v)) / 1000.0 - 0.0 }),
                49 => msg.time_in_hr_zone = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))).collect()),
                50 => msg.time_in_speed_zone = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))).collect()),
                51 => msg.time_in_cadence_zone = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))).collect()),
                52 => msg.time_in_power_zone = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))).collect()),
                53 => msg.repetition_num = content.one().map(<u16>::from),
                54 => msg.min_altitude = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u16>::from(v)) / 5.0 - 500.0 })(v))),
                55 => msg.min_heart_rate = content.one().map(<u8>::from),
                56 => msg.active_time = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                57 => msg.wkt_step_index = content.one().map(<crate::profile::enums::MessageIndex>::from),
                58 => msg.sport_event = content.one().map(<crate::profile::enums::SportEvent>::from),
                59 => msg.avg_left_torque_effectiveness = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 2.0 - 0.0 }),
                60 => msg.avg_right_torque_effectiveness = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 2.0 - 0.0 }),
                61 => msg.avg_left_pedal_smoothness = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 2.0 - 0.0 }),
                62 => msg.avg_right_pedal_smoothness = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 2.0 - 0.0 }),
                63 => msg.avg_combined_pedal_smoothness = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 2.0 - 0.0 }),
                64 => msg.status = content.one().map(<crate::profile::enums::SegmentLapStatus>::from),
                65 => msg.uuid = content.one().map(<String>::from),
                66 => msg.avg_fractional_cadence = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 128.0 - 0.0 }),
                67 => msg.max_fractional_cadence = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 128.0 - 0.0 }),
                68 => msg.total_fractional_cycles = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 128.0 - 0.0 }),
                69 => msg.front_gear_shift_count = content.one().map(<u16>::from),
                70 => msg.rear_gear_shift_count = content.one().map(<u16>::from),
                71 => msg.time_standing = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                72 => msg.stand_count = content.one().map(<u16>::from),
                73 => msg.avg_left_pco = content.one().map(<i8>::from),
                74 => msg.avg_right_pco = content.one().map(<i8>::from),
                75 => msg.avg_left_power_phase = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u8>::from(v)) / 0.0 - 0.0 }).collect()),
                76 => msg.avg_left_power_phase_peak = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u8>::from(v)) / 0.0 - 0.0 }).collect()),
                77 => msg.avg_right_power_phase = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u8>::from(v)) / 0.0 - 0.0 }).collect()),
                78 => msg.avg_right_power_phase_peak = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u8>::from(v)) / 0.0 - 0.0 }).collect()),
                79 => msg.avg_power_position = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                80 => msg.max_power_position = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                81 => msg.avg_cadence_position = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                82 => msg.max_cadence_position = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                83 => msg.manufacturer = content.one().map(<crate::profile::enums::Manufacturer>::from),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                254 => msg.message_index = content.one().map(<crate::profile::enums::MessageIndex>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
