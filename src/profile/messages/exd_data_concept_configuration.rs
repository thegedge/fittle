// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct ExdDataConceptConfiguration {
    screen_index: Option<u8>,
    concept_field: Option<u8>,
    field_id: Option<u8>,
    concept_index: Option<u8>,
    data_page: Option<u8>,
    concept_key: Option<u8>,
    scaling: Option<u8>,
    data_units: Option<enums::ExdDataUnits>,
    qualifier: Option<enums::ExdQualifiers>,
    descriptor: Option<enums::ExdDescriptors>,
    is_signed: Option<bool>,
}

impl ExdDataConceptConfiguration {
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
                3 => msg.concept_index = content.one().map(<u8>::from),
                4 => msg.data_page = content.one().map(<u8>::from),
                5 => msg.concept_key = content.one().map(<u8>::from),
                6 => msg.scaling = content.one().map(<u8>::from),
                8 => msg.data_units = content.one().map(<enums::ExdDataUnits>::from),
                9 => msg.qualifier = content.one().map(<enums::ExdQualifiers>::from),
                10 => msg.descriptor = content.one().map(<enums::ExdDescriptors>::from),
                11 => msg.is_signed = content.one().map(<bool>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

