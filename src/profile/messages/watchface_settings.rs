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
pub struct WatchfaceSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    layout: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<enums::WatchfaceMode>,
}

impl WatchfaceSettings {
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
                0 => msg.mode = content.one().map(<enums::WatchfaceMode>::from),
                1 => msg.layout = content.one().map(<u8>::from),
                254 => msg.message_index = content.one().map(<enums::MessageIndex>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
