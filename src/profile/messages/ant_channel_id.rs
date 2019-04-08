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
pub struct AntChannelId {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_number: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    device_index: Option<crate::profile::enums::DeviceIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    device_number: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    device_type: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    transmission_type: Option<u8>,
}

impl AntChannelId {
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
                self.channel_number =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            1 => {
                self.device_type =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            2 => {
                self.device_number =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            3 => {
                self.transmission_type =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            4 => {
                self.device_index =field.one().map(|v| {
                    let value = crate::profile::enums::DeviceIndex::from(v);
                    value
                })
            },

            _ => (),
        }
    }
}
