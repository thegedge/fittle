// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};
use serde::Serialize;

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct DiveGas {
    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    helium_content: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    oxygen_content: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
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

