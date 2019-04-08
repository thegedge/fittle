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
pub struct Session {
    #[serde(skip_serializing_if = "Option::is_none")]
    avg_altitude: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    avg_ball_speed: Option<crate::fields::Velocity>,

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
    avg_lap_time: Option<crate::fields::Time>,

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
    avg_stroke_count: Option<f64>,

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
    best_lap_index: Option<u16>,

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
    max_ball_speed: Option<crate::fields::Velocity>,

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
    nec_lat: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    nec_long: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    normalized_power: Option<crate::fields::Power>,

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
    threshold_power: Option<crate::fields::Power>,

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
    total_training_effect: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_work: Option<crate::fields::Energy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    training_stress_score: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    trigger: Option<crate::profile::enums::SessionTrigger>,

    #[serde(skip_serializing_if = "Option::is_none")]
    zone_count: Option<Vec<u16>>,
}

impl Session {
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
                self.sport =field.one().map(|v| {
                    let value = crate::profile::enums::Sport::from(v);
                    value
                })
            },

            6 => {
                self.sub_sport =field.one().map(|v| {
                    let value = crate::profile::enums::SubSport::from(v);
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

            13 => {
                self.total_fat_calories =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Energy::new::<uom::si::energy::kilocalorie, u16>)(value)
                })
            },

            14 => {
                self.avg_speed =field.one().map(|v| {
                    let value = u16::from(v);
                    let bits = value.to_le_bytes();
                    let mut bit_reader = BitReader::new(&bits);
                    {
                        bit_reader.read::<u32>(16).map(|bits_value| {
                            self.from_content(124, Field::One(FieldContent::UnsignedInt32(bits_value)));
                        });
                    }
                    (|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            15 => {
                self.max_speed =field.one().map(|v| {
                    let value = u16::from(v);
                    let bits = value.to_le_bytes();
                    let mut bit_reader = BitReader::new(&bits);
                    {
                        bit_reader.read::<u32>(16).map(|bits_value| {
                            self.from_content(125, Field::One(FieldContent::UnsignedInt32(bits_value)));
                        });
                    }
                    (|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            16 => {
                self.avg_heart_rate =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>)(value)
                })
            },

            17 => {
                self.max_heart_rate =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>)(value)
                })
            },

            18 => {
                self.avg_cadence =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>)(value)
                })
            },

            19 => {
                self.max_cadence =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>)(value)
                })
            },

            20 => {
                self.avg_power =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Power::new::<uom::si::power::watt, u16>)(value)
                })
            },

            21 => {
                self.max_power =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Power::new::<uom::si::power::watt, u16>)(value)
                })
            },

            22 => {
                self.total_ascent =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Length::new::<uom::si::length::meter, u16>)(value)
                })
            },

            23 => {
                self.total_descent =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Length::new::<uom::si::length::meter, u16>)(value)
                })
            },

            24 => {
                self.total_training_effect =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 10.0 - 0.0 })(value)
                })
            },

            25 => {
                self.first_lap_index =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            26 => {
                self.num_laps =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            27 => {
                self.event_group =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            28 => {
                self.trigger =field.one().map(|v| {
                    let value = crate::profile::enums::SessionTrigger::from(v);
                    value
                })
            },

            29 => {
                self.nec_lat =field.one().map(|v| {
                    let value = i32::from(v);
                    value
                })
            },

            30 => {
                self.nec_long =field.one().map(|v| {
                    let value = i32::from(v);
                    value
                })
            },

            31 => {
                self.swc_lat =field.one().map(|v| {
                    let value = i32::from(v);
                    value
                })
            },

            32 => {
                self.swc_long =field.one().map(|v| {
                    let value = i32::from(v);
                    value
                })
            },

            34 => {
                self.normalized_power =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Power::new::<uom::si::power::watt, u16>)(value)
                })
            },

            35 => {
                self.training_stress_score =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 10.0 - 0.0 })(value)
                })
            },

            36 => {
                self.intensity_factor =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 1000.0 - 0.0 })(value)
                })
            },

            37 => {
                self.left_right_balance =field.one().map(|v| {
                    let value = crate::profile::enums::LeftRightBalance100::from(v);
                    value
                })
            },

            41 => {
                self.avg_stroke_count =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| { f64::from(v) / 10.0 - 0.0 })(value)
                })
            },

            42 => {
                self.avg_stroke_distance =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 100.0 - 0.0 })(v)))(value)
                })
            },

            43 => {
                self.swim_stroke =field.one().map(|v| {
                    let value = crate::profile::enums::SwimStroke::from(v);
                    value
                })
            },

            44 => {
                self.pool_length =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 100.0 - 0.0 })(v)))(value)
                })
            },

            45 => {
                self.threshold_power =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Power::new::<uom::si::power::watt, u16>)(value)
                })
            },

            46 => {
                self.pool_length_unit =field.one().map(|v| {
                    let value = crate::profile::enums::DisplayMeasure::from(v);
                    value
                })
            },

            47 => {
                self.num_active_lengths =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            48 => {
                self.total_work =field.one().map(|v| {
                    let value = u32::from(v);
                    (crate::fields::Energy::new::<uom::si::energy::joule, u32>)(value)
                })
            },

            49 => {
                self.avg_altitude =field.one().map(|v| {
                    let value = u16::from(v);
                    let bits = value.to_le_bytes();
                    let mut bit_reader = BitReader::new(&bits);
                    {
                        bit_reader.read::<u32>(16).map(|bits_value| {
                            self.from_content(126, Field::One(FieldContent::UnsignedInt32(bits_value)));
                        });
                    }
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 5.0 - 500.0 })(v)))(value)
                })
            },

            50 => {
                self.max_altitude =field.one().map(|v| {
                    let value = u16::from(v);
                    let bits = value.to_le_bytes();
                    let mut bit_reader = BitReader::new(&bits);
                    {
                        bit_reader.read::<u32>(16).map(|bits_value| {
                            self.from_content(128, Field::One(FieldContent::UnsignedInt32(bits_value)));
                        });
                    }
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 5.0 - 500.0 })(v)))(value)
                })
            },

            51 => {
                self.gps_accuracy =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Length::new::<uom::si::length::meter, u8>)(value)
                })
            },

            52 => {
                self.avg_grade =field.one().map(|v| {
                    let value = i16::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            53 => {
                self.avg_pos_grade =field.one().map(|v| {
                    let value = i16::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            54 => {
                self.avg_neg_grade =field.one().map(|v| {
                    let value = i16::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            55 => {
                self.max_pos_grade =field.one().map(|v| {
                    let value = i16::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            56 => {
                self.max_neg_grade =field.one().map(|v| {
                    let value = i16::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            57 => {
                self.avg_temperature =field.one().map(|v| {
                    let value = i8::from(v);
                    (crate::fields::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::degree_celsius, i8>)(value)
                })
            },

            58 => {
                self.max_temperature =field.one().map(|v| {
                    let value = i8::from(v);
                    (crate::fields::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::degree_celsius, i8>)(value)
                })
            },

            59 => {
                self.total_moving_time =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            60 => {
                self.avg_pos_vertical_speed =field.one().map(|v| {
                    let value = i16::from(v);
                    (|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            61 => {
                self.avg_neg_vertical_speed =field.one().map(|v| {
                    let value = i16::from(v);
                    (|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            62 => {
                self.max_pos_vertical_speed =field.one().map(|v| {
                    let value = i16::from(v);
                    (|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            63 => {
                self.max_neg_vertical_speed =field.one().map(|v| {
                    let value = i16::from(v);
                    (|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            64 => {
                self.min_heart_rate =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>)(value)
                })
            },

            65 => {
                self.time_in_hr_zone =field.many().map(|v| {
                    let value = v.into_iter().map(u32::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v))).collect()
                })
            },

            66 => {
                self.time_in_speed_zone =field.many().map(|v| {
                    let value = v.into_iter().map(u32::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v))).collect()
                })
            },

            67 => {
                self.time_in_cadence_zone =field.many().map(|v| {
                    let value = v.into_iter().map(u32::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v))).collect()
                })
            },

            68 => {
                self.time_in_power_zone =field.many().map(|v| {
                    let value = v.into_iter().map(u32::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v))).collect()
                })
            },

            69 => {
                self.avg_lap_time =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            70 => {
                self.best_lap_index =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            71 => {
                self.min_altitude =field.one().map(|v| {
                    let value = u16::from(v);
                    let bits = value.to_le_bytes();
                    let mut bit_reader = BitReader::new(&bits);
                    {
                        bit_reader.read::<u32>(16).map(|bits_value| {
                            self.from_content(127, Field::One(FieldContent::UnsignedInt32(bits_value)));
                        });
                    }
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 5.0 - 500.0 })(v)))(value)
                })
            },

            82 => {
                self.player_score =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            83 => {
                self.opponent_score =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            84 => {
                self.opponent_name =field.one().map(|v| {
                    let value = String::from(v);
                    value
                })
            },

            85 => {
                self.stroke_count =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value
                })
            },

            86 => {
                self.zone_count =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value
                })
            },

            87 => {
                self.max_ball_speed =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { f64::from(v) / 100.0 - 0.0 })(v)))(value)
                })
            },

            88 => {
                self.avg_ball_speed =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { f64::from(v) / 100.0 - 0.0 })(v)))(value)
                })
            },

            89 => {
                self.avg_vertical_oscillation =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::millimeter, f64>((|v| { f64::from(v) / 10.0 - 0.0 })(v)))(value)
                })
            },

            90 => {
                self.avg_stance_time_percent =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            91 => {
                self.avg_stance_time =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::millisecond, f64>((|v| { f64::from(v) / 10.0 - 0.0 })(v)))(value)
                })
            },

            92 => {
                self.avg_fractional_cadence =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, f64>((|v| { f64::from(v) / 128.0 - 0.0 })(v)))(value)
                })
            },

            93 => {
                self.max_fractional_cadence =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, f64>((|v| { f64::from(v) / 128.0 - 0.0 })(v)))(value)
                })
            },

            94 => {
                self.total_fractional_cycles =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 128.0 - 0.0 })(value)
                })
            },

            95 => {
                self.avg_total_hemoglobin_conc =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| { f64::from(v) / 100.0 - 0.0 }).collect()
                })
            },

            96 => {
                self.min_total_hemoglobin_conc =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| { f64::from(v) / 100.0 - 0.0 }).collect()
                })
            },

            97 => {
                self.max_total_hemoglobin_conc =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| { f64::from(v) / 100.0 - 0.0 }).collect()
                })
            },

            98 => {
                self.avg_saturated_hemoglobin_percent =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| { f64::from(v) / 10.0 - 0.0 }).collect()
                })
            },

            99 => {
                self.min_saturated_hemoglobin_percent =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| { f64::from(v) / 10.0 - 0.0 }).collect()
                })
            },

            100 => {
                self.max_saturated_hemoglobin_percent =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| { f64::from(v) / 10.0 - 0.0 }).collect()
                })
            },

            101 => {
                self.avg_left_torque_effectiveness =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 2.0 - 0.0 })(value)
                })
            },

            102 => {
                self.avg_right_torque_effectiveness =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 2.0 - 0.0 })(value)
                })
            },

            103 => {
                self.avg_left_pedal_smoothness =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 2.0 - 0.0 })(value)
                })
            },

            104 => {
                self.avg_right_pedal_smoothness =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 2.0 - 0.0 })(value)
                })
            },

            105 => {
                self.avg_combined_pedal_smoothness =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 2.0 - 0.0 })(value)
                })
            },

            111 => {
                self.sport_index =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            112 => {
                self.time_standing =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            113 => {
                self.stand_count =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            114 => {
                self.avg_left_pco =field.one().map(|v| {
                    let value = i8::from(v);
                    (crate::fields::Length::new::<uom::si::length::millimeter, i8>)(value)
                })
            },

            115 => {
                self.avg_right_pco =field.one().map(|v| {
                    let value = i8::from(v);
                    (crate::fields::Length::new::<uom::si::length::millimeter, i8>)(value)
                })
            },

            116 => {
                self.avg_left_power_phase =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| { f64::from(v) / 0.0 - 0.0 }).collect()
                })
            },

            117 => {
                self.avg_left_power_phase_peak =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| { f64::from(v) / 0.0 - 0.0 }).collect()
                })
            },

            118 => {
                self.avg_right_power_phase =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| { f64::from(v) / 0.0 - 0.0 }).collect()
                })
            },

            119 => {
                self.avg_right_power_phase_peak =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| { f64::from(v) / 0.0 - 0.0 }).collect()
                })
            },

            120 => {
                self.avg_power_position =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value.into_iter().map(crate::fields::Power::new::<uom::si::power::watt, u16>).collect()
                })
            },

            121 => {
                self.max_power_position =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value.into_iter().map(crate::fields::Power::new::<uom::si::power::watt, u16>).collect()
                })
            },

            122 => {
                self.avg_cadence_position =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value.into_iter().map(crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>).collect()
                })
            },

            123 => {
                self.max_cadence_position =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value.into_iter().map(crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>).collect()
                })
            },

            124 => {
                self.enhanced_avg_speed =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            125 => {
                self.enhanced_max_speed =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            126 => {
                self.enhanced_avg_altitude =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 5.0 - 500.0 })(v)))(value)
                })
            },

            127 => {
                self.enhanced_min_altitude =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 5.0 - 500.0 })(v)))(value)
                })
            },

            128 => {
                self.enhanced_max_altitude =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 5.0 - 500.0 })(v)))(value)
                })
            },

            129 => {
                self.avg_lev_motor_power =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Power::new::<uom::si::power::watt, u16>)(value)
                })
            },

            130 => {
                self.max_lev_motor_power =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Power::new::<uom::si::power::watt, u16>)(value)
                })
            },

            131 => {
                self.lev_battery_consumption =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 2.0 - 0.0 })(value)
                })
            },

            132 => {
                self.avg_vertical_ratio =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            133 => {
                self.avg_stance_time_balance =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            134 => {
                self.avg_step_length =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::millimeter, f64>((|v| { f64::from(v) / 10.0 - 0.0 })(v)))(value)
                })
            },

            137 => {
                self.total_anaerobic_training_effect =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 10.0 - 0.0 })(value)
                })
            },

            139 => {
                self.avg_vam =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
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
