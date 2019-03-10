// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};
use serde::Serialize;

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct FileCreator {
    #[serde(skip_serializing_if = "Option::is_none")]
    software_version: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    hardware_version: Option<u8>,

}

impl FileCreator {
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
                0 => msg.software_version = content.one().map(<u16>::from),
                1 => msg.hardware_version = content.one().map(<u8>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

