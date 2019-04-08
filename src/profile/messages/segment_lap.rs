// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

#[allow(unused_imports)]
use crate::bits::BitReader;

#[allow(unused_imports)]
use crate::fields::{
    Field,
    FieldContent,
    FieldDefinition,
};

#[derive(Debug, Default, Serialize)]
pub struct SegmentLap {
    #[serde(skip_serializing_if = "Option::is_none")]
    active_time: Option<crate::fields::Time>,

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
    avg_speed: Option<crate::fields::Velocity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_temperature: Option<crate::fields::ThermodynamicTemperature>,

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
    max_cadence: Option<crate::fields::Frequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_cadence_position: Option<Vec<crate::fields::Frequency>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_fractional_cadence: Option<crate::fields::Frequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_heart_rate: Option<crate::fields::Frequency>,

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
    max_speed: Option<crate::fields::Velocity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_temperature: Option<crate::fields::ThermodynamicTemperature>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_altitude: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_heart_rate: Option<crate::fields::Frequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    nec_lat: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    nec_long: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    normalized_power: Option<crate::fields::Power>,

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
    uuid: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    wkt_step_index: Option<crate::profile::enums::MessageIndex>,
}

impl SegmentLap {
    pub fn from_fields<Order, Reader>(reader: &mut Reader, field_defs: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut msg: Self = Default::default();
        for field_def in field_defs {
            let (number, field) = field_def.content_from::<Order, Reader>(reader)?;
            msg.from_content(number, field);
        }

        Ok(msg)
    }

    fn from_content(&mut self, number: u8, field: Field) {
        match number {
            0 => {
                self.event =field.one().map(|v| {
                    let value = crate::profile::enums::Event::from(v);
                    value
                })
            },

            1 => {
                self.event_type =field.one().map(|v| {
                    let value = crate::profile::enums::EventType::from(v);
                    value
                })
            },

            2 => {
                self.start_time =field.one().map(|v| {
                    let value = crate::fields::DateTime::from(v);
                    value
                })
            },

            3 => {
                self.start_position_lat =field.one().map(|v| {
                    let value = i32::from(v);
                    value
                })
            },

            4 => {
                self.start_position_long =field.one().map(|v| {
                    let value = i32::from(v);
                    value
                })
            },

            5 => {
                self.end_position_lat =field.one().map(|v| {
                    let value = i32::from(v);
                    value
                })
            },

            6 => {
                self.end_position_long =field.one().map(|v| {
                    let value = i32::from(v);
                    value
                })
            },

            7 => {
                self.total_elapsed_time =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            8 => {
                self.total_timer_time =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            9 => {
                self.total_distance =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 100.0 - 0.0 })(v)))(value)
                })
            },

            10 => {
                self.total_cycles =field.one().map(|v| {
                    let value = u32::from(v);
                    value
                })
            },

            11 => {
                self.total_calories =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Energy::new::<uom::si::energy::kilocalorie, u16>)(value)
                })
            },

            12 => {
                self.total_fat_calories =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Energy::new::<uom::si::energy::kilocalorie, u16>)(value)
                })
            },

            13 => {
                self.avg_speed =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            14 => {
                self.max_speed =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            15 => {
                self.avg_heart_rate =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>)(value)
                })
            },

            16 => {
                self.max_heart_rate =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>)(value)
                })
            },

            17 => {
                self.avg_cadence =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>)(value)
                })
            },

            18 => {
                self.max_cadence =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>)(value)
                })
            },

            19 => {
                self.avg_power =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Power::new::<uom::si::power::watt, u16>)(value)
                })
            },

            20 => {
                self.max_power =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Power::new::<uom::si::power::watt, u16>)(value)
                })
            },

            21 => {
                self.total_ascent =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Length::new::<uom::si::length::meter, u16>)(value)
                })
            },

            22 => {
                self.total_descent =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Length::new::<uom::si::length::meter, u16>)(value)
                })
            },

            23 => {
                self.sport =field.one().map(|v| {
                    let value = crate::profile::enums::Sport::from(v);
                    value
                })
            },

            24 => {
                self.event_group =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            25 => {
                self.nec_lat =field.one().map(|v| {
                    let value = i32::from(v);
                    value
                })
            },

            26 => {
                self.nec_long =field.one().map(|v| {
                    let value = i32::from(v);
                    value
                })
            },

            27 => {
                self.swc_lat =field.one().map(|v| {
                    let value = i32::from(v);
                    value
                })
            },

            28 => {
                self.swc_long =field.one().map(|v| {
                    let value = i32::from(v);
                    value
                })
            },

            29 => {
                self.name =field.one().map(|v| {
                    let value = String::from(v);
                    value
                })
            },

            30 => {
                self.normalized_power =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Power::new::<uom::si::power::watt, u16>)(value)
                })
            },

            31 => {
                self.left_right_balance =field.one().map(|v| {
                    let value = crate::profile::enums::LeftRightBalance100::from(v);
                    value
                })
            },

            32 => {
                self.sub_sport =field.one().map(|v| {
                    let value = crate::profile::enums::SubSport::from(v);
                    value
                })
            },

            33 => {
                self.total_work =field.one().map(|v| {
                    let value = u32::from(v);
                    (crate::fields::Energy::new::<uom::si::energy::joule, u32>)(value)
                })
            },

            34 => {
                self.avg_altitude =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 5.0 - 500.0 })(v)))(value)
                })
            },

            35 => {
                self.max_altitude =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 5.0 - 500.0 })(v)))(value)
                })
            },

            36 => {
                self.gps_accuracy =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Length::new::<uom::si::length::meter, u8>)(value)
                })
            },

            37 => {
                self.avg_grade =field.one().map(|v| {
                    let value = i16::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            38 => {
                self.avg_pos_grade =field.one().map(|v| {
                    let value = i16::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            39 => {
                self.avg_neg_grade =field.one().map(|v| {
                    let value = i16::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            40 => {
                self.max_pos_grade =field.one().map(|v| {
                    let value = i16::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            41 => {
                self.max_neg_grade =field.one().map(|v| {
                    let value = i16::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            42 => {
                self.avg_temperature =field.one().map(|v| {
                    let value = i8::from(v);
                    (crate::fields::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::degree_celsius, i8>)(value)
                })
            },

            43 => {
                self.max_temperature =field.one().map(|v| {
                    let value = i8::from(v);
                    (crate::fields::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::degree_celsius, i8>)(value)
                })
            },

            44 => {
                self.total_moving_time =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            45 => {
                self.avg_pos_vertical_speed =field.one().map(|v| {
                    let value = i16::from(v);
                    (|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            46 => {
                self.avg_neg_vertical_speed =field.one().map(|v| {
                    let value = i16::from(v);
                    (|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            47 => {
                self.max_pos_vertical_speed =field.one().map(|v| {
                    let value = i16::from(v);
                    (|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            48 => {
                self.max_neg_vertical_speed =field.one().map(|v| {
                    let value = i16::from(v);
                    (|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            49 => {
                self.time_in_hr_zone =field.many().map(|v| {
                    let value = v.into_iter().map(u32::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v))).collect()
                })
            },

            50 => {
                self.time_in_speed_zone =field.many().map(|v| {
                    let value = v.into_iter().map(u32::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v))).collect()
                })
            },

            51 => {
                self.time_in_cadence_zone =field.many().map(|v| {
                    let value = v.into_iter().map(u32::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v))).collect()
                })
            },

            52 => {
                self.time_in_power_zone =field.many().map(|v| {
                    let value = v.into_iter().map(u32::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v))).collect()
                })
            },

            53 => {
                self.repetition_num =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            54 => {
                self.min_altitude =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 5.0 - 500.0 })(v)))(value)
                })
            },

            55 => {
                self.min_heart_rate =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>)(value)
                })
            },

            56 => {
                self.active_time =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            57 => {
                self.wkt_step_index =field.one().map(|v| {
                    let value = crate::profile::enums::MessageIndex::from(v);
                    value
                })
            },

            58 => {
                self.sport_event =field.one().map(|v| {
                    let value = crate::profile::enums::SportEvent::from(v);
                    value
                })
            },

            59 => {
                self.avg_left_torque_effectiveness =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 2.0 - 0.0 })(value)
                })
            },

            60 => {
                self.avg_right_torque_effectiveness =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 2.0 - 0.0 })(value)
                })
            },

            61 => {
                self.avg_left_pedal_smoothness =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 2.0 - 0.0 })(value)
                })
            },

            62 => {
                self.avg_right_pedal_smoothness =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 2.0 - 0.0 })(value)
                })
            },

            63 => {
                self.avg_combined_pedal_smoothness =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 2.0 - 0.0 })(value)
                })
            },

            64 => {
                self.status =field.one().map(|v| {
                    let value = crate::profile::enums::SegmentLapStatus::from(v);
                    value
                })
            },

            65 => {
                self.uuid =field.one().map(|v| {
                    let value = String::from(v);
                    value
                })
            },

            66 => {
                self.avg_fractional_cadence =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, f64>((|v| { f64::from(v) / 128.0 - 0.0 })(v)))(value)
                })
            },

            67 => {
                self.max_fractional_cadence =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, f64>((|v| { f64::from(v) / 128.0 - 0.0 })(v)))(value)
                })
            },

            68 => {
                self.total_fractional_cycles =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 128.0 - 0.0 })(value)
                })
            },

            69 => {
                self.front_gear_shift_count =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            70 => {
                self.rear_gear_shift_count =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            71 => {
                self.time_standing =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            72 => {
                self.stand_count =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            73 => {
                self.avg_left_pco =field.one().map(|v| {
                    let value = i8::from(v);
                    (crate::fields::Length::new::<uom::si::length::millimeter, i8>)(value)
                })
            },

            74 => {
                self.avg_right_pco =field.one().map(|v| {
                    let value = i8::from(v);
                    (crate::fields::Length::new::<uom::si::length::millimeter, i8>)(value)
                })
            },

            75 => {
                self.avg_left_power_phase =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| { f64::from(v) / 0.0 - 0.0 }).collect()
                })
            },

            76 => {
                self.avg_left_power_phase_peak =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| { f64::from(v) / 0.0 - 0.0 }).collect()
                })
            },

            77 => {
                self.avg_right_power_phase =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| { f64::from(v) / 0.0 - 0.0 }).collect()
                })
            },

            78 => {
                self.avg_right_power_phase_peak =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| { f64::from(v) / 0.0 - 0.0 }).collect()
                })
            },

            79 => {
                self.avg_power_position =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value.into_iter().map(crate::fields::Power::new::<uom::si::power::watt, u16>).collect()
                })
            },

            80 => {
                self.max_power_position =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value.into_iter().map(crate::fields::Power::new::<uom::si::power::watt, u16>).collect()
                })
            },

            81 => {
                self.avg_cadence_position =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value.into_iter().map(crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>).collect()
                })
            },

            82 => {
                self.max_cadence_position =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value.into_iter().map(crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>).collect()
                })
            },

            83 => {
                self.manufacturer =field.one().map(|v| {
                    let value = crate::profile::enums::Manufacturer::from(v);
                    value
                })
            },

            253 => {
                self.timestamp =field.one().map(|v| {
                    let value = crate::fields::DateTime::from(v);
                    value
                })
            },

            254 => {
                self.message_index =field.one().map(|v| {
                    let value = crate::profile::enums::MessageIndex::from(v);
                    value
                })
            },

            _ => (),
        }
    }
}
