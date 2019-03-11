// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct ExdDataFieldConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    concept_count: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    concept_field: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    display_type: Option<enums::ExdDisplayType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    field_id: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    screen_index: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<Vec<String>>,
}

impl ExdDataFieldConfiguration {
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
                0 => msg.screen_index = content.one().map(<u8>::from),
                1 => msg.concept_field = content.one().map(<u8>::from),
                2 => msg.field_id = content.one().map(<u8>::from),
                3 => msg.concept_count = content.one().map(<u8>::from),
                4 => msg.display_type = content.one().map(<enums::ExdDisplayType>::from),
                5 => msg.title = content.many().map(|vec| vec.into_iter().map(<String>::from).collect()),
                _ => (),
            };
        }

        Ok(msg)
    }
}
