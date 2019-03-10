// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct DeveloperDataId {
    developer_id: Option<Vec<u8>>,
    application_id: Option<Vec<u8>>,
    manufacturer_id: Option<enums::Manufacturer>,
    developer_data_index: Option<u8>,
    application_version: Option<u32>,
}

impl DeveloperDataId {
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
                0 => msg.developer_id = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                1 => msg.application_id = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                2 => msg.manufacturer_id = content.one().map(<enums::Manufacturer>::from),
                3 => msg.developer_data_index = content.one().map(<u8>::from),
                4 => msg.application_version = content.one().map(<u32>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

