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
pub struct DiveSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    avg_depth: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bottom_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dive_number: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end_cns: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end_n2: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_depth: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    o2_toxicity: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    reference_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    reference_mesg: Option<crate::profile::enums::MesgNum>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_cns: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_n2: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    surface_interval: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,
}

impl DiveSummary {
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
                self.reference_mesg =field.one().map(|v| {
                    let value = crate::profile::enums::MesgNum::from(v);
                    value
                })
            },

            1 => {
                self.reference_index =field.one().map(|v| {
                    let value = crate::profile::enums::MessageIndex::from(v);
                    value
                })
            },

            2 => {
                self.avg_depth =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            3 => {
                self.max_depth =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
                })
            },

            4 => {
                self.surface_interval =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1.0 - 0.0 })(v)))(value)
                })
            },

            5 => {
                self.start_cns =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 1.0 - 0.0 })(value)
                })
            },

            6 => {
                self.end_cns =field.one().map(|v| {
                    let value = u8::from(v);
                    (|v| { f64::from(v) / 1.0 - 0.0 })(value)
                })
            },

            7 => {
                self.start_n2 =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 1.0 - 0.0 })(value)
                })
            },

            8 => {
                self.end_n2 =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| { f64::from(v) / 1.0 - 0.0 })(value)
                })
            },

            9 => {
                self.o2_toxicity =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            10 => {
                self.dive_number =field.one().map(|v| {
                    let value = u32::from(v);
                    value
                })
            },

            11 => {
                self.bottom_time =field.one().map(|v| {
                    let value = u32::from(v);
                    (|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { f64::from(v) / 1000.0 - 0.0 })(v)))(value)
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
