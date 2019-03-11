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
pub struct FieldCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    field_num: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<enums::File>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mesg_num: Option<enums::MesgNum>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<enums::MessageIndex>,
}

impl FieldCapabilities {
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
                0 => msg.file = content.one().map(<enums::File>::from),
                1 => msg.mesg_num = content.one().map(<enums::MesgNum>::from),
                2 => msg.field_num = content.one().map(<u8>::from),
                3 => msg.count = content.one().map(<u16>::from),
                254 => msg.message_index = content.one().map(<enums::MessageIndex>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
