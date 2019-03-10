// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};
use serde::Serialize;

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct SegmentLap {
    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<enums::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event: Option<enums::Event>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event_type: Option<enums::EventType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<enums::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_position_lat: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_position_long: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end_position_lat: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end_position_long: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_elapsed_time: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_timer_time: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_distance: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_cycles: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_calories: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_fat_calories: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_speed: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_speed: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_heart_rate: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_heart_rate: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_cadence: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_cadence: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_power: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_power: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_ascent: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_descent: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sport: Option<enums::Sport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event_group: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    nec_lat: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    nec_long: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    swc_lat: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    swc_long: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    normalized_power: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left_right_balance: Option<enums::LeftRightBalance100>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sub_sport: Option<enums::SubSport>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_work: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_altitude: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_altitude: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    gps_accuracy: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_grade: Option<i16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_pos_grade: Option<i16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_neg_grade: Option<i16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_pos_grade: Option<i16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_neg_grade: Option<i16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_temperature: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_temperature: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_moving_time: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_pos_vertical_speed: Option<i16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_neg_vertical_speed: Option<i16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_pos_vertical_speed: Option<i16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_neg_vertical_speed: Option<i16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_in_hr_zone: Option<Vec<u32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_in_speed_zone: Option<Vec<u32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_in_cadence_zone: Option<Vec<u32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_in_power_zone: Option<Vec<u32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    repetition_num: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_altitude: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_heart_rate: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    active_time: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    wkt_step_index: Option<enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sport_event: Option<enums::SportEvent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_left_torque_effectiveness: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_right_torque_effectiveness: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_left_pedal_smoothness: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_right_pedal_smoothness: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_combined_pedal_smoothness: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<enums::SegmentLapStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    uuid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_fractional_cadence: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_fractional_cadence: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_fractional_cycles: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    front_gear_shift_count: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rear_gear_shift_count: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_standing: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stand_count: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_left_pco: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_right_pco: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_left_power_phase: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_left_power_phase_peak: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_right_power_phase: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_right_power_phase_peak: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_power_position: Option<Vec<u16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_power_position: Option<Vec<u16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_cadence_position: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_cadence_position: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    manufacturer: Option<enums::Manufacturer>,

}

impl SegmentLap {
    pub fn from_fields<'i, Order, Reader>(reader: &mut Reader, fields: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut msg: Self = Default::default();
        for field in fields {
            let (number, content) = field.content_from::<Order, Reader>(reader)?;
            match number {
                254 => msg.message_index = content.one().map(<enums::MessageIndex>::from),
                253 => msg.timestamp = content.one().map(<enums::DateTime>::from),
                0 => msg.event = content.one().map(<enums::Event>::from),
                1 => msg.event_type = content.one().map(<enums::EventType>::from),
                2 => msg.start_time = content.one().map(<enums::DateTime>::from),
                3 => msg.start_position_lat = content.one().map(<i32>::from),
                4 => msg.start_position_long = content.one().map(<i32>::from),
                5 => msg.end_position_lat = content.one().map(<i32>::from),
                6 => msg.end_position_long = content.one().map(<i32>::from),
                7 => msg.total_elapsed_time = content.one().map(<u32>::from),
                8 => msg.total_timer_time = content.one().map(<u32>::from),
                9 => msg.total_distance = content.one().map(<u32>::from),
                10 => msg.total_cycles = content.one().map(<u32>::from),
                11 => msg.total_calories = content.one().map(<u16>::from),
                12 => msg.total_fat_calories = content.one().map(<u16>::from),
                13 => msg.avg_speed = content.one().map(<u16>::from),
                14 => msg.max_speed = content.one().map(<u16>::from),
                15 => msg.avg_heart_rate = content.one().map(<u8>::from),
                16 => msg.max_heart_rate = content.one().map(<u8>::from),
                17 => msg.avg_cadence = content.one().map(<u8>::from),
                18 => msg.max_cadence = content.one().map(<u8>::from),
                19 => msg.avg_power = content.one().map(<u16>::from),
                20 => msg.max_power = content.one().map(<u16>::from),
                21 => msg.total_ascent = content.one().map(<u16>::from),
                22 => msg.total_descent = content.one().map(<u16>::from),
                23 => msg.sport = content.one().map(<enums::Sport>::from),
                24 => msg.event_group = content.one().map(<u8>::from),
                25 => msg.nec_lat = content.one().map(<i32>::from),
                26 => msg.nec_long = content.one().map(<i32>::from),
                27 => msg.swc_lat = content.one().map(<i32>::from),
                28 => msg.swc_long = content.one().map(<i32>::from),
                29 => msg.name = content.one().map(<String>::from),
                30 => msg.normalized_power = content.one().map(<u16>::from),
                31 => msg.left_right_balance = content.one().map(<enums::LeftRightBalance100>::from),
                32 => msg.sub_sport = content.one().map(<enums::SubSport>::from),
                33 => msg.total_work = content.one().map(<u32>::from),
                34 => msg.avg_altitude = content.one().map(<u16>::from),
                35 => msg.max_altitude = content.one().map(<u16>::from),
                36 => msg.gps_accuracy = content.one().map(<u8>::from),
                37 => msg.avg_grade = content.one().map(<i16>::from),
                38 => msg.avg_pos_grade = content.one().map(<i16>::from),
                39 => msg.avg_neg_grade = content.one().map(<i16>::from),
                40 => msg.max_pos_grade = content.one().map(<i16>::from),
                41 => msg.max_neg_grade = content.one().map(<i16>::from),
                42 => msg.avg_temperature = content.one().map(<i8>::from),
                43 => msg.max_temperature = content.one().map(<i8>::from),
                44 => msg.total_moving_time = content.one().map(<u32>::from),
                45 => msg.avg_pos_vertical_speed = content.one().map(<i16>::from),
                46 => msg.avg_neg_vertical_speed = content.one().map(<i16>::from),
                47 => msg.max_pos_vertical_speed = content.one().map(<i16>::from),
                48 => msg.max_neg_vertical_speed = content.one().map(<i16>::from),
                49 => msg.time_in_hr_zone = content.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                50 => msg.time_in_speed_zone = content.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                51 => msg.time_in_cadence_zone = content.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                52 => msg.time_in_power_zone = content.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                53 => msg.repetition_num = content.one().map(<u16>::from),
                54 => msg.min_altitude = content.one().map(<u16>::from),
                55 => msg.min_heart_rate = content.one().map(<u8>::from),
                56 => msg.active_time = content.one().map(<u32>::from),
                57 => msg.wkt_step_index = content.one().map(<enums::MessageIndex>::from),
                58 => msg.sport_event = content.one().map(<enums::SportEvent>::from),
                59 => msg.avg_left_torque_effectiveness = content.one().map(<u8>::from),
                60 => msg.avg_right_torque_effectiveness = content.one().map(<u8>::from),
                61 => msg.avg_left_pedal_smoothness = content.one().map(<u8>::from),
                62 => msg.avg_right_pedal_smoothness = content.one().map(<u8>::from),
                63 => msg.avg_combined_pedal_smoothness = content.one().map(<u8>::from),
                64 => msg.status = content.one().map(<enums::SegmentLapStatus>::from),
                65 => msg.uuid = content.one().map(<String>::from),
                66 => msg.avg_fractional_cadence = content.one().map(<u8>::from),
                67 => msg.max_fractional_cadence = content.one().map(<u8>::from),
                68 => msg.total_fractional_cycles = content.one().map(<u8>::from),
                69 => msg.front_gear_shift_count = content.one().map(<u16>::from),
                70 => msg.rear_gear_shift_count = content.one().map(<u16>::from),
                71 => msg.time_standing = content.one().map(<u32>::from),
                72 => msg.stand_count = content.one().map(<u16>::from),
                73 => msg.avg_left_pco = content.one().map(<i8>::from),
                74 => msg.avg_right_pco = content.one().map(<i8>::from),
                75 => msg.avg_left_power_phase = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                76 => msg.avg_left_power_phase_peak = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                77 => msg.avg_right_power_phase = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                78 => msg.avg_right_power_phase_peak = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                79 => msg.avg_power_position = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                80 => msg.max_power_position = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                81 => msg.avg_cadence_position = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                82 => msg.max_cadence_position = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                83 => msg.manufacturer = content.one().map(<enums::Manufacturer>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

