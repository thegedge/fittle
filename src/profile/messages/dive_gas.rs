// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct DiveGas {
    message_index: Option<enums::MessageIndex>,
    helium_content: Option<u8>,
    oxygen_content: Option<u8>,
    status: Option<enums::DiveGasStatus>,
}

impl DiveGas {
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
                0 => msg.helium_content = content.one().map(<u8>::from),
                1 => msg.oxygen_content = content.one().map(<u8>::from),
                2 => msg.status = content.one().map(<enums::DiveGasStatus>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

