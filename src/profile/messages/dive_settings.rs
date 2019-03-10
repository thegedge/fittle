// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

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

impl DiveSettings {
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
                0 => msg.name = content.one().map(<String>::from),
                1 => msg.model = content.one().map(<enums::TissueModelType>::from),
                2 => msg.gf_low = content.one().map(<u8>::from),
                3 => msg.gf_high = content.one().map(<u8>::from),
                4 => msg.water_type = content.one().map(<enums::WaterType>::from),
                5 => msg.water_density = content.one().map(<f32>::from),
                6 => msg.po2_warn = content.one().map(<u8>::from),
                7 => msg.po2_critical = content.one().map(<u8>::from),
                8 => msg.po2_deco = content.one().map(<u8>::from),
                9 => msg.safety_stop_enabled = content.one().map(<bool>::from),
                10 => msg.bottom_depth = content.one().map(<f32>::from),
                11 => msg.bottom_time = content.one().map(<u32>::from),
                12 => msg.apnea_countdown_enabled = content.one().map(<bool>::from),
                13 => msg.apnea_countdown_time = content.one().map(<u32>::from),
                14 => msg.backlight_mode = content.one().map(<enums::DiveBacklightMode>::from),
                15 => msg.backlight_brightness = content.one().map(<u8>::from),
                16 => msg.backlight_timeout = content.one().map(<enums::BacklightTimeout>::from),
                17 => msg.repeat_dive_interval = content.one().map(<u16>::from),
                18 => msg.safety_stop_time = content.one().map(<u16>::from),
                19 => msg.heart_rate_source_type = content.one().map(<enums::SourceType>::from),
                20 => msg.heart_rate_source = content.one().map(<u8>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

