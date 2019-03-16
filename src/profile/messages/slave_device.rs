// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct SlaveDevice {
    #[serde(skip_serializing_if = "Option::is_none")]
    manufacturer: Option<crate::profile::enums::Manufacturer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    product: Option<u16>,
}

impl SlaveDevice {
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
                0 => msg.manufacturer = content.one().map(<crate::profile::enums::Manufacturer>::from),
                1 => msg.product = content.one().map(<u16>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
