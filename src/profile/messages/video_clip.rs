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
pub struct VideoClip {
    #[serde(skip_serializing_if = "Option::is_none")]
    clip_end: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    clip_number: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    clip_start: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end_timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    end_timestamp_ms: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_timestamp_ms: Option<u16>,
}

impl VideoClip {
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
                self.clip_number =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            1 => {
                self.start_timestamp =field.one().map(|v| {
                    let value = crate::fields::DateTime::from(v);
                    value
                })
            },

            2 => {
                self.start_timestamp_ms =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            3 => {
                self.end_timestamp =field.one().map(|v| {
                    let value = crate::fields::DateTime::from(v);
                    value
                })
            },

            4 => {
                self.end_timestamp_ms =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            6 => {
                self.clip_start =field.one().map(|v| {
                    let value = u32::from(v);
                    (crate::fields::Time::new::<uom::si::time::millisecond, u32>)(value)
                })
            },

            7 => {
                self.clip_end =field.one().map(|v| {
                    let value = u32::from(v);
                    (crate::fields::Time::new::<uom::si::time::millisecond, u32>)(value)
                })
            },

            _ => (),
        }
    }
}
