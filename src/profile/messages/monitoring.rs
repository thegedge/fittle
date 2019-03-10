// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};
use serde::Serialize;

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct Monitoring {
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<enums::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    device_index: Option<enums::DeviceIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calories: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    distance: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cycles: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    active_time: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    activity_type: Option<enums::ActivityType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    activity_subtype: Option<enums::ActivitySubtype>,

    #[serde(skip_serializing_if = "Option::is_none")]
    activity_level: Option<enums::ActivityLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    distance_16: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cycles_16: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    active_time_16: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    local_timestamp: Option<enums::LocalDateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<i16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature_min: Option<i16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature_max: Option<i16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    activity_time: Option<Vec<u16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    active_calories: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    current_activity_type_intensity: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_min_8: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_16: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    heart_rate: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    intensity: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    duration_min: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ascent: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    descent: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    moderate_activity_minutes: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    vigorous_activity_minutes: Option<u16>,

}

impl Monitoring {
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
                253 => msg.timestamp = content.one().map(<enums::DateTime>::from),
                0 => msg.device_index = content.one().map(<enums::DeviceIndex>::from),
                1 => msg.calories = content.one().map(<u16>::from),
                2 => msg.distance = content.one().map(<u32>::from),
                3 => msg.cycles = content.one().map(<u32>::from),
                4 => msg.active_time = content.one().map(<u32>::from),
                5 => msg.activity_type = content.one().map(<enums::ActivityType>::from),
                6 => msg.activity_subtype = content.one().map(<enums::ActivitySubtype>::from),
                7 => msg.activity_level = content.one().map(<enums::ActivityLevel>::from),
                8 => msg.distance_16 = content.one().map(<u16>::from),
                9 => msg.cycles_16 = content.one().map(<u16>::from),
                10 => msg.active_time_16 = content.one().map(<u16>::from),
                11 => msg.local_timestamp = content.one().map(<enums::LocalDateTime>::from),
                12 => msg.temperature = content.one().map(<i16>::from),
                14 => msg.temperature_min = content.one().map(<i16>::from),
                15 => msg.temperature_max = content.one().map(<i16>::from),
                16 => msg.activity_time = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                19 => msg.active_calories = content.one().map(<u16>::from),
                24 => msg.current_activity_type_intensity = content.one().map(<u8>::from),
                25 => msg.timestamp_min_8 = content.one().map(<u8>::from),
                26 => msg.timestamp_16 = content.one().map(<u16>::from),
                27 => msg.heart_rate = content.one().map(<u8>::from),
                28 => msg.intensity = content.one().map(<u8>::from),
                29 => msg.duration_min = content.one().map(<u16>::from),
                30 => msg.duration = content.one().map(<u32>::from),
                31 => msg.ascent = content.one().map(<u32>::from),
                32 => msg.descent = content.one().map(<u32>::from),
                33 => msg.moderate_activity_minutes = content.one().map(<u16>::from),
                34 => msg.vigorous_activity_minutes = content.one().map(<u16>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

