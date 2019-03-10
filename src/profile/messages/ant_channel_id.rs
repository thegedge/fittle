// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct AntChannelId {
    channel_number: Option<u8>,
    device_type: Option<u8>,
    device_number: Option<u16>,
    transmission_type: Option<u8>,
    device_index: Option<enums::DeviceIndex>,
}

impl AntChannelId {
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
                0 => msg.channel_number = content.one().map(<u8>::from),
                1 => msg.device_type = content.one().map(<u8>::from),
                2 => msg.device_number = content.one().map(<u16>::from),
                3 => msg.transmission_type = content.one().map(<u8>::from),
                4 => msg.device_index = content.one().map(<enums::DeviceIndex>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

