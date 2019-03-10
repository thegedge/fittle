// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};
use serde::Serialize;

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct FileCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<enums::File>,

    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<enums::FileFlags>,

    #[serde(skip_serializing_if = "Option::is_none")]
    directory: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_count: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_size: Option<u32>,

}

impl FileCapabilities {
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
                254 => msg.message_index = content.one().map(<enums::MessageIndex>::from),
                0 => msg.type_ = content.one().map(<enums::File>::from),
                1 => msg.flags = content.one().map(<enums::FileFlags>::from),
                2 => msg.directory = content.one().map(<String>::from),
                3 => msg.max_count = content.one().map(<u16>::from),
                4 => msg.max_size = content.one().map(<u32>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

