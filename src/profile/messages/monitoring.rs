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
pub struct Monitoring {
    #[serde(skip_serializing_if = "Option::is_none")]
    active_calories: Option<crate::fields::Energy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    active_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    active_time_16: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    activity_level: Option<crate::profile::enums::ActivityLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    activity_subtype: Option<crate::profile::enums::ActivitySubtype>,

    #[serde(skip_serializing_if = "Option::is_none")]
    activity_time: Option<Vec<crate::fields::Time>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    activity_type: Option<crate::profile::enums::ActivityType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ascent: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calories: Option<crate::fields::Energy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    current_activity_type_intensity: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cycles: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cycles_16: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    descent: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    device_index: Option<crate::profile::enums::DeviceIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    distance: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    distance_16: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    duration_min: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    heart_rate: Option<crate::fields::Frequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    intensity: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    local_timestamp: Option<crate::fields::LocalDateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    moderate_activity_minutes: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<crate::fields::ThermodynamicTemperature>,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature_max: Option<crate::fields::ThermodynamicTemperature>,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature_min: Option<crate::fields::ThermodynamicTemperature>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_16: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_min_8: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    vigorous_activity_minutes: Option<crate::fields::Time>,
}

impl Monitoring {
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
                self.device_index =field.one().map(|v| {
                    let value = crate::profile::enums::DeviceIndex::from(v);
                    value
                })
            },

            1 => {
                self.calories =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Energy::new::<uom::si::energy::kilocalorie, u16>)(value)
                })
            },

            2 => {
                self.distance =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 100.0 - 0.0 })(v)))(value)
                })
            },

            3 => {
                self.cycles =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| { f64::from(v) / 2.0 - 0.0 })(value)
                })
            },

            4 => {
                self.active_time =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            5 => {
                self.activity_type =field.one().map(|v| {
                    let value = crate::profile::enums::ActivityType::from(v);
                    value
                })
            },

            6 => {
                self.activity_subtype =field.one().map(|v| {
                    let value = crate::profile::enums::ActivitySubtype::from(v);
                    value
                })
            },

            7 => {
                self.activity_level =field.one().map(|v| {
                    let value = crate::profile::enums::ActivityLevel::from(v);
                    value
                })
            },

            8 => {
                self.distance_16 =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Length::new::<uom::si::length::meter, u16>)(value)
                })
            },

            9 => {
                self.cycles_16 =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            10 => {
                self.active_time_16 =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Time::new::<uom::si::time::second, u16>)(value)
                })
            },

            11 => {
                self.local_timestamp =field.one().map(|v| {
                    let value = crate::fields::LocalDateTime::from(v);
                    value
                })
            },

            12 => {
                self.temperature =field.one().map(|v| {
                    let value = i16::from(v);
                    (|v| crate::fields::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::degree_celsius, f64>((|v| { f64::from(v) / 100.0 - 0.0 })(v)))(value)
                })
            },

            14 => {
                self.temperature_min =field.one().map(|v| {
                    let value = i16::from(v);
                    (|v| crate::fields::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::degree_celsius, f64>((|v| { f64::from(v) / 100.0 - 0.0 })(v)))(value)
                })
            },

            15 => {
                self.temperature_max =field.one().map(|v| {
                    let value = i16::from(v);
                    (|v| crate::fields::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::degree_celsius, f64>((|v| { f64::from(v) / 100.0 - 0.0 })(v)))(value)
                })
            },

            16 => {
                self.activity_time =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value.into_iter().map(crate::fields::Time::new::<uom::si::time::minute, u16>).collect()
                })
            },

            19 => {
                self.active_calories =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Energy::new::<uom::si::energy::kilocalorie, u16>)(value)
                })
            },

            24 => {
                self.current_activity_type_intensity =field.one().map(|v| {
                    let value = u8::from(v);
                    let bits = value.to_le_bytes();
                    let mut bit_reader = BitReader::new(&bits);
                    {
                        bit_reader.read::<u8>(5).map(|bits_value| {
                            self.from_content(5, Field::One(FieldContent::Enum(bits_value)));
                        });
                    }
                    {
                        bit_reader.read::<u8>(3).map(|bits_value| {
                            self.from_content(28, Field::One(FieldContent::UnsignedInt8(bits_value)));
                        });
                    }
                    value
                })
            },

            25 => {
                self.timestamp_min_8 =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Time::new::<uom::si::time::minute, u8>)(value)
                })
            },

            26 => {
                self.timestamp_16 =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Time::new::<uom::si::time::second, u16>)(value)
                })
            },

            27 => {
                self.heart_rate =field.one().map(|v| {
                    let value = u8::from(v);
                    (crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>)(value)
                })
            },

            28 => {
                self.intensity =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 10.0 - 0.0 })(value)
                })
            },

            29 => {
                self.duration_min =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Time::new::<uom::si::time::minute, u16>)(value)
                })
            },

            30 => {
                self.duration =field.one().map(|v| {
                    let value = u32::from(v);
                    (crate::fields::Time::new::<uom::si::time::second, u32>)(value)
                })
            },

            31 => {
                self.ascent =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            32 => {
                self.descent =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            33 => {
                self.moderate_activity_minutes =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Time::new::<uom::si::time::minute, u16>)(value)
                })
            },

            34 => {
                self.vigorous_activity_minutes =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Time::new::<uom::si::time::minute, u16>)(value)
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
