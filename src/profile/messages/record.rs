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
pub struct Record {
    #[serde(skip_serializing_if = "Option::is_none")]
    absolute_pressure: Option<crate::fields::Pressure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    accumulated_power: Option<crate::fields::Power>,

    #[serde(skip_serializing_if = "Option::is_none")]
    activity_type: Option<crate::profile::enums::ActivityType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    altitude: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ball_speed: Option<crate::fields::Velocity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    battery_soc: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cadence: Option<crate::fields::Frequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cadence256: Option<crate::fields::Frequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calories: Option<crate::fields::Energy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cns_load: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    combined_pedal_smoothness: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    compressed_accumulated_power: Option<crate::fields::Power>,

    #[serde(skip_serializing_if = "Option::is_none")]
    compressed_speed_distance: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cycle_length: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cycles: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    depth: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    device_index: Option<crate::profile::enums::DeviceIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    distance: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    enhanced_altitude: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    enhanced_speed: Option<crate::fields::Velocity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fractional_cadence: Option<crate::fields::Frequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    gps_accuracy: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    grade: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    heart_rate: Option<crate::fields::Frequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left_pco: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left_pedal_smoothness: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left_power_phase: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left_power_phase_peak: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left_right_balance: Option<crate::profile::enums::LeftRightBalance>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left_torque_effectiveness: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    motor_power: Option<crate::fields::Power>,

    #[serde(skip_serializing_if = "Option::is_none")]
    n2_load: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ndl_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    next_stop_depth: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    next_stop_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    position_lat: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    position_long: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    power: Option<crate::fields::Power>,

    #[serde(skip_serializing_if = "Option::is_none")]
    resistance: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    right_pco: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    right_pedal_smoothness: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    right_power_phase: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    right_power_phase_peak: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    right_torque_effectiveness: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    saturated_hemoglobin_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    saturated_hemoglobin_percent_max: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    saturated_hemoglobin_percent_min: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    speed: Option<crate::fields::Velocity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    speed_1s: Option<Vec<crate::fields::Velocity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stance_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stance_time_balance: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stance_time_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    step_length: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stroke_type: Option<crate::profile::enums::StrokeType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<crate::fields::ThermodynamicTemperature>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time128: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_from_course: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_to_surface: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_cycles: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_hemoglobin_conc: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_hemoglobin_conc_max: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_hemoglobin_conc_min: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    vertical_oscillation: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    vertical_ratio: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    vertical_speed: Option<crate::fields::Velocity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<u8>,
}

impl Record {
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
                self.position_lat =field.one().map(|v| {
                    let value = i32::from(v);
                    value
                })
            },

            1 => {
                self.position_long =field.one().map(|v| {
                    let value = i32::from(v);
                    value
                })
            },

            2 => {
                self.altitude =field.one().map(|v| {
                    let value = u16::from(v);
                    let bits = value.to_le_bytes();
                    let mut bit_reader = BitReader::new(&bits);
                    {
                        bit_reader.read::<u32>(16).map(|bits_value| {
                            self.from_content(78, Field::One(FieldContent::UnsignedInt32(bits_value)));
                        });
                    }
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 5.0 - 500.0 })(v)))(value)
                })
            },

            3 => {
                self.heart_rate =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>)(value)
                })
            },

            4 => {
                self.cadence =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>)(value)
                })
            },

            5 => {
                self.distance =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 100.0 - 0.0 })(v)))(value)
                })
            },

            6 => {
                self.speed =field.one().map(|v| {
                    let value = u16::from(v);
                    let bits = value.to_le_bytes();
                    let mut bit_reader = BitReader::new(&bits);
                    {
                        bit_reader.read::<u32>(16).map(|bits_value| {
                            self.from_content(73, Field::One(FieldContent::UnsignedInt32(bits_value)));
                        });
                    }
                    (|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            7 => {
                self.power =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Power::new::<uom::si::power::watt, u16>)(value)
                })
            },

            8 => {
                self.compressed_speed_distance =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    let bits = value.as_slice();
                    let mut bit_reader = BitReader::new(&bits);
                    {
                        bit_reader.read::<u16>(12).map(|bits_value| {
                            self.from_content(6, Field::One(FieldContent::UnsignedInt16(bits_value)));
                        });
                    }
                    value
                })
            },

            9 => {
                self.grade =field.one().map(|v| {
                    let value = i16::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            10 => {
                self.resistance =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            11 => {
                self.time_from_course =field.one().map(|v| {
                    let value = i32::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            12 => {
                self.cycle_length =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 100.0 - 0.0 })(v)))(value)
                })
            },

            13 => {
                self.temperature =field.one().map(|v| {
                    let value = i8::from(v);
                    (crate::fields::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::degree_celsius, i8>)(value)
                })
            },

            17 => {
                self.speed_1s =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { f64::from(v) / 16.0 - 0.0 })(v))).collect()
                })
            },

            18 => {
                self.cycles =field.one().map(|v| {
                    let value = u8::from(v);
                    let bits = value.to_le_bytes();
                    let mut bit_reader = BitReader::new(&bits);
                    {
                        bit_reader.read::<u32>(8).map(|bits_value| {
                            self.from_content(19, Field::One(FieldContent::UnsignedInt32(bits_value)));
                        });
                    }
                    value
                })
            },

            19 => {
                self.total_cycles =field.one().map(|v| {
                    let value = u32::from(v);
                    value
                })
            },

            28 => {
                self.compressed_accumulated_power =field.one().map(|v| {
                    let value = u16::from(v);
                    let bits = value.to_le_bytes();
                    let mut bit_reader = BitReader::new(&bits);
                    {
                        bit_reader.read::<u32>(16).map(|bits_value| {
                            self.from_content(29, Field::One(FieldContent::UnsignedInt32(bits_value)));
                        });
                    }
                    (crate::fields::Power::new::<uom::si::power::watt, u16>)(value)
                })
            },

            29 => {
                self.accumulated_power =field.one().map(|v| {
                    let value = u32::from(v);
                    (crate::fields::Power::new::<uom::si::power::watt, u32>)(value)
                })
            },

            30 => {
                self.left_right_balance =field.one().map(|v| {
                    let value = crate::profile::enums::LeftRightBalance::from(v);
                    value
                })
            },

            31 => {
                self.gps_accuracy =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Length::new::<uom::si::length::meter, u8>)(value)
                })
            },

            32 => {
                self.vertical_speed =field.one().map(|v| {
                    let value = i16::from(v);
                    (|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            33 => {
                self.calories =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Energy::new::<uom::si::energy::kilocalorie, u16>)(value)
                })
            },

            39 => {
                self.vertical_oscillation =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::millimeter, f64>((|v| { f64::from(v) / 10.0 - 0.0 })(v)))(value)
                })
            },

            40 => {
                self.stance_time_percent =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            41 => {
                self.stance_time =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::millisecond, f64>((|v| { f64::from(v) / 10.0 - 0.0 })(v)))(value)
                })
            },

            42 => {
                self.activity_type =field.one().map(|v| {
                    let value = crate::profile::enums::ActivityType::from(v);
                    value
                })
            },

            43 => {
                self.left_torque_effectiveness =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 2.0 - 0.0 })(value)
                })
            },

            44 => {
                self.right_torque_effectiveness =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 2.0 - 0.0 })(value)
                })
            },

            45 => {
                self.left_pedal_smoothness =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 2.0 - 0.0 })(value)
                })
            },

            46 => {
                self.right_pedal_smoothness =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 2.0 - 0.0 })(value)
                })
            },

            47 => {
                self.combined_pedal_smoothness =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 2.0 - 0.0 })(value)
                })
            },

            48 => {
                self.time128 =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 128.0 - 0.0 })(v)))(value)
                })
            },

            49 => {
                self.stroke_type =field.one().map(|v| {
                    let value = crate::profile::enums::StrokeType::from(v);
                    value
                })
            },

            50 => {
                self.zone =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            51 => {
                self.ball_speed =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { f64::from(v) / 100.0 - 0.0 })(v)))(value)
                })
            },

            52 => {
                self.cadence256 =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, f64>((|v| { f64::from(v) / 256.0 - 0.0 })(v)))(value)
                })
            },

            53 => {
                self.fractional_cadence =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, f64>((|v| { f64::from(v) / 128.0 - 0.0 })(v)))(value)
                })
            },

            54 => {
                self.total_hemoglobin_conc =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            55 => {
                self.total_hemoglobin_conc_min =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            56 => {
                self.total_hemoglobin_conc_max =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            57 => {
                self.saturated_hemoglobin_percent =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 10.0 - 0.0 })(value)
                })
            },

            58 => {
                self.saturated_hemoglobin_percent_min =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 10.0 - 0.0 })(value)
                })
            },

            59 => {
                self.saturated_hemoglobin_percent_max =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 10.0 - 0.0 })(value)
                })
            },

            62 => {
                self.device_index =field.one().map(|v| {
                    let value = crate::profile::enums::DeviceIndex::from(v);
                    value
                })
            },

            67 => {
                self.left_pco =field.one().map(|v| {
                    let value = i8::from(v);
                    (crate::fields::Length::new::<uom::si::length::millimeter, i8>)(value)
                })
            },

            68 => {
                self.right_pco =field.one().map(|v| {
                    let value = i8::from(v);
                    (crate::fields::Length::new::<uom::si::length::millimeter, i8>)(value)
                })
            },

            69 => {
                self.left_power_phase =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| { f64::from(v) / 0.0 - 0.0 }).collect()
                })
            },

            70 => {
                self.left_power_phase_peak =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| { f64::from(v) / 0.0 - 0.0 }).collect()
                })
            },

            71 => {
                self.right_power_phase =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| { f64::from(v) / 0.0 - 0.0 }).collect()
                })
            },

            72 => {
                self.right_power_phase_peak =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| { f64::from(v) / 0.0 - 0.0 }).collect()
                })
            },

            73 => {
                self.enhanced_speed =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            78 => {
                self.enhanced_altitude =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 5.0 - 500.0 })(v)))(value)
                })
            },

            81 => {
                self.battery_soc =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 2.0 - 0.0 })(value)
                })
            },

            82 => {
                self.motor_power =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Power::new::<uom::si::power::watt, u16>)(value)
                })
            },

            83 => {
                self.vertical_ratio =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            84 => {
                self.stance_time_balance =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            85 => {
                self.step_length =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::millimeter, f64>((|v| { f64::from(v) / 10.0 - 0.0 })(v)))(value)
                })
            },

            91 => {
                self.absolute_pressure =field.one().map(|v| {
                    let value = u32::from(v);
                    (crate::fields::Pressure::new::<uom::si::pressure::pascal, u32>)(value)
                })
            },

            92 => {
                self.depth =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            93 => {
                self.next_stop_depth =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            94 => {
                self.next_stop_time =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1.0 - 0.0 })(v)))(value)
                })
            },

            95 => {
                self.time_to_surface =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1.0 - 0.0 })(v)))(value)
                })
            },

            96 => {
                self.ndl_time =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1.0 - 0.0 })(v)))(value)
                })
            },

            97 => {
                self.cns_load =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            98 => {
                self.n2_load =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 1.0 - 0.0 })(value)
                })
            },

            253 => {
                self.timestamp =field.one().map(|v| {
                    let value = crate::fields::DateTime::from(v);
                    value
                })
            },

            _ => (),
        }
    }
}
