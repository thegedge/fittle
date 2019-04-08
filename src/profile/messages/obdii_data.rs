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
pub struct ObdiiData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pid: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pid_data_size: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    raw_data: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_timestamp_ms: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    system_time: Option<Vec<u32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_offset: Option<Vec<crate::fields::Time>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp_ms: Option<crate::fields::Time>,
}

impl ObdiiData {
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
                self.timestamp_ms =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Time::new::<uom::si::time::millisecond, u16>)(value)
                })
            },

            1 => {
                self.time_offset =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value.into_iter().map(crate::fields::Time::new::<uom::si::time::millisecond, u16>).collect()
                })
            },

            2 => {
                self.pid =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            3 => {
                self.raw_data =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value
                })
            },

            4 => {
                self.pid_data_size =field.many().map(|v| {
                    let value = v.into_iter().map(u8::from).collect::<Vec<_>>();
                    value
                })
            },

            5 => {
                self.system_time =field.many().map(|v| {
                    let value = v.into_iter().map(u32::from).collect::<Vec<_>>();
                    value
                })
            },

            6 => {
                self.start_timestamp =field.one().map(|v| {
                    let value = crate::fields::DateTime::from(v);
                    value
                })
            },

            7 => {
                self.start_timestamp_ms =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Time::new::<uom::si::time::millisecond, u16>)(value)
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
