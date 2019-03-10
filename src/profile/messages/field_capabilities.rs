// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct FieldCapabilities {
    message_index: Option<enums::MessageIndex>,
    file: Option<enums::File>,
    mesg_num: Option<enums::MesgNum>,
    field_num: Option<u8>,
    count: Option<u16>,
}

impl FieldCapabilities {
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
                2 => msg.field_num = content.one().map(<u8>::from),
                3 => msg.count = content.one().map(<u16>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

