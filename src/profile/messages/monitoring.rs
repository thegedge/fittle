// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct Monitoring {
    timestamp: Option<enums::DateTime>,
    device_index: Option<enums::DeviceIndex>,
    calories: Option<u16>,
    distance: Option<u32>,
    cycles: Option<u32>,
    active_time: Option<u32>,
    activity_type: Option<enums::ActivityType>,
    activity_subtype: Option<enums::ActivitySubtype>,
    activity_level: Option<enums::ActivityLevel>,
    distance_16: Option<u16>,
    cycles_16: Option<u16>,
    active_time_16: Option<u16>,
    local_timestamp: Option<enums::LocalDateTime>,
    temperature: Option<i16>,
    temperature_min: Option<i16>,
    temperature_max: Option<i16>,
    activity_time: Option<Vec<u16>>,
    active_calories: Option<u16>,
    current_activity_type_intensity: Option<u8>,
    timestamp_min_8: Option<u8>,
    timestamp_16: Option<u16>,
    heart_rate: Option<u8>,
    intensity: Option<u8>,
    duration_min: Option<u16>,
    duration: Option<u32>,
    ascent: Option<u32>,
    descent: Option<u32>,
    moderate_activity_minutes: Option<u16>,
    vigorous_activity_minutes: Option<u16>,
}

impl From<Vec<(u8, Field)>> for Monitoring {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.device_index = field.one().map(<enums::DeviceIndex>::from),
                1 => msg.calories = field.one().map(<u16>::from),
                2 => msg.distance = field.one().map(<u32>::from),
                3 => msg.cycles = field.one().map(<u32>::from),
                4 => msg.active_time = field.one().map(<u32>::from),
                5 => msg.activity_type = field.one().map(<enums::ActivityType>::from),
                6 => msg.activity_subtype = field.one().map(<enums::ActivitySubtype>::from),
                7 => msg.activity_level = field.one().map(<enums::ActivityLevel>::from),
                8 => msg.distance_16 = field.one().map(<u16>::from),
                9 => msg.cycles_16 = field.one().map(<u16>::from),
                10 => msg.active_time_16 = field.one().map(<u16>::from),
                11 => msg.local_timestamp = field.one().map(<enums::LocalDateTime>::from),
                12 => msg.temperature = field.one().map(<i16>::from),
                14 => msg.temperature_min = field.one().map(<i16>::from),
                15 => msg.temperature_max = field.one().map(<i16>::from),
                16 => msg.activity_time = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                19 => msg.active_calories = field.one().map(<u16>::from),
                24 => msg.current_activity_type_intensity = field.one().map(<u8>::from),
                25 => msg.timestamp_min_8 = field.one().map(<u8>::from),
                26 => msg.timestamp_16 = field.one().map(<u16>::from),
                27 => msg.heart_rate = field.one().map(<u8>::from),
                28 => msg.intensity = field.one().map(<u8>::from),
                29 => msg.duration_min = field.one().map(<u16>::from),
                30 => msg.duration = field.one().map(<u32>::from),
                31 => msg.ascent = field.one().map(<u32>::from),
                32 => msg.descent = field.one().map(<u32>::from),
                33 => msg.moderate_activity_minutes = field.one().map(<u16>::from),
                34 => msg.vigorous_activity_minutes = field.one().map(<u16>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

