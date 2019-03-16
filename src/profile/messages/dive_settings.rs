// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct DiveSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    apnea_countdown_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    apnea_countdown_time: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    backlight_brightness: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    backlight_mode: Option<crate::profile::enums::DiveBacklightMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    backlight_timeout: Option<crate::profile::enums::BacklightTimeout>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bottom_depth: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bottom_time: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    gf_high: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    gf_low: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    heart_rate_source: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    heart_rate_source_type: Option<crate::profile::enums::SourceType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<crate::profile::enums::TissueModelType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    po2_critical: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    po2_deco: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    po2_warn: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    repeat_dive_interval: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    safety_stop_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    safety_stop_time: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    water_density: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    water_type: Option<crate::profile::enums::WaterType>,
}

impl DiveSettings {
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
                0 => msg.name = content.one().map(<String>::from),
                1 => msg.model = content.one().map(<crate::profile::enums::TissueModelType>::from),
                2 => msg.gf_low = content.one().map(<u8>::from),
                3 => msg.gf_high = content.one().map(<u8>::from),
                4 => msg.water_type = content.one().map(<crate::profile::enums::WaterType>::from),
                5 => msg.water_density = content.one().map(<f32>::from),
                6 => msg.po2_warn = content.one().map(<u8>::from),
                7 => msg.po2_critical = content.one().map(<u8>::from),
                8 => msg.po2_deco = content.one().map(<u8>::from),
                9 => msg.safety_stop_enabled = content.one().map(<bool>::from),
                10 => msg.bottom_depth = content.one().map(<f32>::from),
                11 => msg.bottom_time = content.one().map(<u32>::from),
                12 => msg.apnea_countdown_enabled = content.one().map(<bool>::from),
                13 => msg.apnea_countdown_time = content.one().map(<u32>::from),
                14 => msg.backlight_mode = content.one().map(<crate::profile::enums::DiveBacklightMode>::from),
                15 => msg.backlight_brightness = content.one().map(<u8>::from),
                16 => msg.backlight_timeout = content.one().map(<crate::profile::enums::BacklightTimeout>::from),
                17 => msg.repeat_dive_interval = content.one().map(<u16>::from),
                18 => msg.safety_stop_time = content.one().map(<u16>::from),
                19 => msg.heart_rate_source_type = content.one().map(<crate::profile::enums::SourceType>::from),
                20 => msg.heart_rate_source = content.one().map(<u8>::from),
                254 => msg.message_index = content.one().map(<crate::profile::enums::MessageIndex>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
