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
pub struct Event {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    data16: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    device_index: Option<crate::profile::enums::DeviceIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event: Option<crate::profile::enums::Event>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event_group: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event_type: Option<crate::profile::enums::EventType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    front_gear: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    front_gear_num: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    opponent_score: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rear_gear: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    rear_gear_num: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    score: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,
}

impl Event {
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
                self.event =field.one().map(|v| {
                    let value = crate::profile::enums::Event::from(v);
                    value
                })
            },

            1 => {
                self.event_type =field.one().map(|v| {
                    let value = crate::profile::enums::EventType::from(v);
                    value
                })
            },

            2 => {
                self.data16 =field.one().map(|v| {
                    let value = u16::from(v);
                    let bits = value.to_le_bytes();
                    let mut bit_reader = BitReader::new(&bits);
                    {
                        bit_reader.read::<u32>(16).map(|bits_value| {
                            self.from_content(3, Field::One(FieldContent::UnsignedInt32(bits_value)));
                        });
                    }
                    value
                })
            },

            3 => {
                self.data =field.one().map(|v| {
                    let value = u32::from(v);
                    value
                })
            },

            4 => {
                self.event_group =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            7 => {
                self.score =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            8 => {
                self.opponent_score =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            9 => {
                self.front_gear_num =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            10 => {
                self.front_gear =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            11 => {
                self.rear_gear_num =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            12 => {
                self.rear_gear =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            13 => {
                self.device_index =field.one().map(|v| {
                    let value = crate::profile::enums::DeviceIndex::from(v);
                    value
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
