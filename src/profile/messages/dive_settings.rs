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
    po2_critical: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    po2_deco: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    po2_warn: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    repeat_dive_interval: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    safety_stop_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    safety_stop_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    water_density: Option<crate::fields::Density>,

    #[serde(skip_serializing_if = "Option::is_none")]
    water_type: Option<crate::profile::enums::WaterType>,
}

impl DiveSettings {
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
                self.name =field.one().map(|v| {
                    let value = String::from(v);
                    value
                })
            },

            1 => {
                self.model =field.one().map(|v| {
                    let value = crate::profile::enums::TissueModelType::from(v);
                    value
                })
            },

            2 => {
                self.gf_low =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            3 => {
                self.gf_high =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            4 => {
                self.water_type =field.one().map(|v| {
                    let value = crate::profile::enums::WaterType::from(v);
                    value
                })
            },

            5 => {
                self.water_density =field.one().map(|v| {
                    let value = f32::from(v);
                    (crate::fields::Density::new::<uom::si::density::kilogram_per_cubic_meter, f32>)(value)
                })
            },

            6 => {
                self.po2_warn =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            7 => {
                self.po2_critical =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            8 => {
                self.po2_deco =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 100.0 - 0.0 })(value)
                })
            },

            9 => {
                self.safety_stop_enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            10 => {
                self.bottom_depth =field.one().map(|v| {
                    let value = f32::from(v);
                    value
                })
            },

            11 => {
                self.bottom_time =field.one().map(|v| {
                    let value = u32::from(v);
                    value
                })
            },

            12 => {
                self.apnea_countdown_enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            13 => {
                self.apnea_countdown_time =field.one().map(|v| {
                    let value = u32::from(v);
                    value
                })
            },

            14 => {
                self.backlight_mode =field.one().map(|v| {
                    let value = crate::profile::enums::DiveBacklightMode::from(v);
                    value
                })
            },

            15 => {
                self.backlight_brightness =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            16 => {
                self.backlight_timeout =field.one().map(|v| {
                    let value = crate::profile::enums::BacklightTimeout::from(v);
                    value
                })
            },

            17 => {
                self.repeat_dive_interval =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1.0 - 0.0 })(v)))(value)
                })
            },

            18 => {
                self.safety_stop_time =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1.0 - 0.0 })(v)))(value)
                })
            },

            19 => {
                self.heart_rate_source_type =field.one().map(|v| {
                    let value = crate::profile::enums::SourceType::from(v);
                    value
                })
            },

            20 => {
                self.heart_rate_source =field.one().map(|v| {
                    let value = u8::from(v);
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
