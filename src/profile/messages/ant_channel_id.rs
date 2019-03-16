// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct AntChannelId {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_number: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    device_index: Option<crate::profile::enums::DeviceIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    device_number: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    device_type: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    transmission_type: Option<u8>,
}

impl AntChannelId {
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
                0 => msg.channel_number = content.one().map(<u8>::from),
                1 => msg.device_type = content.one().map(<u8>::from),
                2 => msg.device_number = content.one().map(<u16>::from),
                3 => msg.transmission_type = content.one().map(<u8>::from),
                4 => msg.device_index = content.one().map(<crate::profile::enums::DeviceIndex>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
