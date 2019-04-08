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
pub struct SdmProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    odometer: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    odometer_rollover: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sdm_ant_id: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sdm_ant_id_trans_type: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sdm_cal_factor: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    speed_source: Option<bool>,
}

impl SdmProfile {
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
                self.enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            1 => {
                self.sdm_ant_id =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            2 => {
                self.sdm_cal_factor =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 10.0 - 0.0 })(value)
                })
            },

            3 => {
                self.odometer =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 100.0 - 0.0 })(v)))(value)
                })
            },

            4 => {
                self.speed_source =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            5 => {
                self.sdm_ant_id_trans_type =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            7 => {
                self.odometer_rollover =field.one().map(|v| {
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
