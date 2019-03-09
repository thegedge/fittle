// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

#[derive(Debug, Default)]
pub struct DiveSettings {
    message_index: Option<enums::MessageIndex>,
    name: Option<String>,
    model: Option<enums::TissueModelType>,
    gf_low: Option<u8>,
    gf_high: Option<u8>,
    water_type: Option<enums::WaterType>,
    water_density: Option<f32>,
    po2_warn: Option<u8>,
    po2_critical: Option<u8>,
    po2_deco: Option<u8>,
    safety_stop_enabled: Option<bool>,
    bottom_depth: Option<f32>,
    bottom_time: Option<u32>,
    apnea_countdown_enabled: Option<bool>,
    apnea_countdown_time: Option<u32>,
    backlight_mode: Option<enums::DiveBacklightMode>,
    backlight_brightness: Option<u8>,
    backlight_timeout: Option<enums::BacklightTimeout>,
    repeat_dive_interval: Option<u16>,
    safety_stop_time: Option<u16>,
    heart_rate_source_type: Option<enums::SourceType>,
    heart_rate_source: Option<u8>,
}

impl From<Vec<(u8, Field)>> for DiveSettings {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.name = field.one().map(<String>::from),
                1 => msg.model = field.one().map(<enums::TissueModelType>::from),
                2 => msg.gf_low = field.one().map(<u8>::from),
                3 => msg.gf_high = field.one().map(<u8>::from),
                4 => msg.water_type = field.one().map(<enums::WaterType>::from),
                5 => msg.water_density = field.one().map(<f32>::from),
                6 => msg.po2_warn = field.one().map(<u8>::from),
                7 => msg.po2_critical = field.one().map(<u8>::from),
                8 => msg.po2_deco = field.one().map(<u8>::from),
                9 => msg.safety_stop_enabled = field.one().map(<bool>::from),
                10 => msg.bottom_depth = field.one().map(<f32>::from),
                11 => msg.bottom_time = field.one().map(<u32>::from),
                12 => msg.apnea_countdown_enabled = field.one().map(<bool>::from),
                13 => msg.apnea_countdown_time = field.one().map(<u32>::from),
                14 => msg.backlight_mode = field.one().map(<enums::DiveBacklightMode>::from),
                15 => msg.backlight_brightness = field.one().map(<u8>::from),
                16 => msg.backlight_timeout = field.one().map(<enums::BacklightTimeout>::from),
                17 => msg.repeat_dive_interval = field.one().map(<u16>::from),
                18 => msg.safety_stop_time = field.one().map(<u16>::from),
                19 => msg.heart_rate_source_type = field.one().map(<enums::SourceType>::from),
                20 => msg.heart_rate_source = field.one().map(<u8>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

