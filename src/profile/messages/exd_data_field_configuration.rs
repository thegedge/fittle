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
pub struct ExdDataFieldConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    concept_count: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    concept_field: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    display_type: Option<crate::profile::enums::ExdDisplayType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    field_id: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    screen_index: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<Vec<String>>,
}

impl ExdDataFieldConfiguration {
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
                self.screen_index =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            1 => {
                self.concept_field =field.one().map(|v| {
                    let value = u8::from(v);
                    let bits = value.to_le_bytes();
                    let mut bit_reader = BitReader::new(&bits);
                    {
                        bit_reader.read::<u8>(4).map(|bits_value| {
                            self.from_content(2, Field::One(FieldContent::UnsignedInt8(bits_value)));
                        });
                    }
                    {
                        bit_reader.read::<u8>(4).map(|bits_value| {
                            self.from_content(3, Field::One(FieldContent::UnsignedInt8(bits_value)));
                        });
                    }
                    value
                })
            },

            2 => {
                self.field_id =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            3 => {
                self.concept_count =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            4 => {
                self.display_type =field.one().map(|v| {
                    let value = crate::profile::enums::ExdDisplayType::from(v);
                    value
                })
            },

            5 => {
                self.title =field.many().map(|v| {
                    let value = v.into_iter().map(String::from).collect::<Vec<_>>();
                    value
                })
            },

            _ => (),
        }
    }
}
