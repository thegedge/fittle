// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct ExdDataFieldConfiguration {
    screen_index: Option<u8>,
    concept_field: Option<u8>,
    field_id: Option<u8>,
    concept_count: Option<u8>,
    display_type: Option<enums::ExdDisplayType>,
    title: Option<Vec<String>>,
}

impl ExdDataFieldConfiguration {
    pub fn from_fields<'i, Order, Reader>(reader: &mut Reader, fields: &Vec<FieldDefinition>)
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

