// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct DeveloperDataId {
    #[serde(skip_serializing_if = "Option::is_none")]
    application_id: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    application_version: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    developer_data_index: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    developer_id: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    manufacturer_id: Option<crate::profile::enums::Manufacturer>,
}

impl DeveloperDataId {
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
                0 => msg.developer_id = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                1 => msg.application_id = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                2 => msg.manufacturer_id = content.one().map(<crate::profile::enums::Manufacturer>::from),
                3 => msg.developer_data_index = content.one().map(<u8>::from),
                4 => msg.application_version = content.one().map(<u32>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
