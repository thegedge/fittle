// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

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
                0 => msg.device_index = content.one().map(<crate::profile::enums::DeviceIndex>::from),
                1 => msg.calories = content.one().map(|v| crate::fields::Energy::new::<uom::si::energy::kilocalorie, u16>((<u16>::from)(v))),
                2 => msg.distance = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u32>::from(v)) / 100.0 - 0.0 })(v))),
                3 => msg.cycles = content.one().map(|v| { <f64>::from(<u32>::from(v)) / 2.0 - 0.0 }),
                4 => msg.active_time = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                5 => msg.activity_type = content.one().map(<crate::profile::enums::ActivityType>::from),
                6 => msg.activity_subtype = content.one().map(<crate::profile::enums::ActivitySubtype>::from),
                7 => msg.activity_level = content.one().map(<crate::profile::enums::ActivityLevel>::from),
                8 => msg.distance_16 = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, u16>((<u16>::from)(v))),
                9 => msg.cycles_16 = content.one().map(<u16>::from),
                10 => msg.active_time_16 = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, u16>((<u16>::from)(v))),
                11 => msg.local_timestamp = content.one().map(<crate::fields::LocalDateTime>::from),
                12 => msg.temperature = content.one().map(|v| crate::fields::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::degree_celsius, f64>((|v| { <f64>::from(<i16>::from(v)) / 100.0 - 0.0 })(v))),
                14 => msg.temperature_min = content.one().map(|v| crate::fields::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::degree_celsius, f64>((|v| { <f64>::from(<i16>::from(v)) / 100.0 - 0.0 })(v))),
                15 => msg.temperature_max = content.one().map(|v| crate::fields::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::degree_celsius, f64>((|v| { <f64>::from(<i16>::from(v)) / 100.0 - 0.0 })(v))),
                16 => msg.activity_time = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::minute, u16>((<u16>::from)(v))).collect()),
                19 => msg.active_calories = content.one().map(|v| crate::fields::Energy::new::<uom::si::energy::kilocalorie, u16>((<u16>::from)(v))),
                24 => msg.current_activity_type_intensity = content.one().map(<u8>::from),
                25 => msg.timestamp_min_8 = content.one().map(|v| crate::fields::Time::new::<uom::si::time::minute, u8>((<u8>::from)(v))),
                26 => msg.timestamp_16 = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, u16>((<u16>::from)(v))),
                27 => msg.heart_rate = content.one().map(|v| crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>((<u8>::from)(v))),
                28 => msg.intensity = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 10.0 - 0.0 }),
                29 => msg.duration_min = content.one().map(|v| crate::fields::Time::new::<uom::si::time::minute, u16>((<u16>::from)(v))),
                30 => msg.duration = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, u32>((<u32>::from)(v))),
                31 => msg.ascent = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                32 => msg.descent = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                33 => msg.moderate_activity_minutes = content.one().map(|v| crate::fields::Time::new::<uom::si::time::minute, u16>((<u16>::from)(v))),
                34 => msg.vigorous_activity_minutes = content.one().map(|v| crate::fields::Time::new::<uom::si::time::minute, u16>((<u16>::from)(v))),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
