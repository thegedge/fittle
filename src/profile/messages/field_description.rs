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
pub struct FieldDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    accumulate: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    array: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bits: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    developer_data_index: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    field_definition_number: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    field_name: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fit_base_type_id: Option<crate::profile::enums::FitBaseType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fit_base_unit_id: Option<crate::profile::enums::FitBaseUnit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    native_field_num: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    native_mesg_num: Option<crate::profile::enums::MesgNum>,

    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<i8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    units: Option<Vec<String>>,
}

impl FieldDescription {
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
                self.developer_data_index =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            1 => {
                self.field_definition_number =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            2 => {
                self.fit_base_type_id =field.one().map(|v| {
                    let value = crate::profile::enums::FitBaseType::from(v);
                    value
                })
            },

            3 => {
                self.field_name =field.many().map(|v| {
                    let value = v.into_iter().map(String::from).collect::<Vec<_>>();
                    value
                })
            },

            4 => {
                self.array =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            5 => {
                self.components =field.one().map(|v| {
                    let value = String::from(v);
                    value
                })
            },

            6 => {
                self.scale =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            7 => {
                self.offset =field.one().map(|v| {
                    let value = i8::from(v);
                    value
                })
            },

            8 => {
                self.units =field.many().map(|v| {
                    let value = v.into_iter().map(String::from).collect::<Vec<_>>();
                    value
                })
            },

            9 => {
                self.bits =field.one().map(|v| {
                    let value = String::from(v);
                    value
                })
            },

            10 => {
                self.accumulate =field.one().map(|v| {
                    let value = String::from(v);
                    value
                })
            },

            13 => {
                self.fit_base_unit_id =field.one().map(|v| {
                    let value = crate::profile::enums::FitBaseUnit::from(v);
                    value
                })
            },

            14 => {
                self.native_mesg_num =field.one().map(|v| {
                    let value = crate::profile::enums::MesgNum::from(v);
                    value
                })
            },

            15 => {
                self.native_field_num =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            _ => (),
        }
    }
}
