// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};
use serde::Serialize;

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct MesgCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<enums::File>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mesg_num: Option<enums::MesgNum>,

    #[serde(skip_serializing_if = "Option::is_none")]
    count_type: Option<enums::MesgCount>,

    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<u16>,

}

impl MesgCapabilities {
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
                0 => msg.file = content.one().map(<enums::File>::from),
                1 => msg.mesg_num = content.one().map(<enums::MesgNum>::from),
                2 => msg.count_type = content.one().map(<enums::MesgCount>::from),
                3 => msg.count = content.one().map(<u16>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

