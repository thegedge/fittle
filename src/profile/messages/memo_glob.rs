// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};
use serde::Serialize;

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct MemoGlob {
    #[serde(skip_serializing_if = "Option::is_none")]
    part_index: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    memo: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_number: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<enums::MessageIndex>,

}

impl MemoGlob {
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
                250 => msg.part_index = content.one().map(<u32>::from),
                0 => msg.memo = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                1 => msg.message_number = content.one().map(<u16>::from),
                2 => msg.message_index = content.one().map(<enums::MessageIndex>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

