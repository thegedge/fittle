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
pub struct AntRx {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_number: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fractional_timestamp: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mesg_data: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mesg_id: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<enums::DateTime>,
}

impl AntRx {
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
                0 => msg.fractional_timestamp = content.one().map(<u16>::from),
                1 => msg.mesg_id = content.one().map(<u8>::from),
                2 => msg.mesg_data = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                3 => msg.channel_number = content.one().map(<u8>::from),
                4 => msg.data = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                253 => msg.timestamp = content.one().map(<enums::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
