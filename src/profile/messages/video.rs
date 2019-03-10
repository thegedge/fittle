// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};
use serde::Serialize;

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct Video {
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    hosting_provider: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<u32>,

}

impl Video {
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
                0 => msg.url = content.one().map(<String>::from),
                1 => msg.hosting_provider = content.one().map(<String>::from),
                2 => msg.duration = content.one().map(<u32>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

