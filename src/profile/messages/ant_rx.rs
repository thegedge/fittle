// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct AntRx {
    timestamp: Option<enums::DateTime>,
    fractional_timestamp: Option<u16>,
    mesg_id: Option<u8>,
    mesg_data: Option<Vec<u8>>,
    channel_number: Option<u8>,
    data: Option<Vec<u8>>,
}

impl AntRx {
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
                253 => msg.timestamp = content.one().map(<enums::DateTime>::from),
                0 => msg.fractional_timestamp = content.one().map(<u16>::from),
                1 => msg.mesg_id = content.one().map(<u8>::from),
                2 => msg.mesg_data = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                3 => msg.channel_number = content.one().map(<u8>::from),
                4 => msg.data = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                _ => (),
            };
        }
        Ok(msg)
    }
}

