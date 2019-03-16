// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct TrainingFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    manufacturer: Option<crate::profile::enums::Manufacturer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    product: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    serial_number: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_created: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<crate::profile::enums::File>,
}

impl TrainingFile {
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
                0 => msg.type_ = content.one().map(<crate::profile::enums::File>::from),
                1 => msg.manufacturer = content.one().map(<crate::profile::enums::Manufacturer>::from),
                2 => msg.product = content.one().map(<u16>::from),
                3 => msg.serial_number = content.one().map(<u32>::from),
                4 => msg.time_created = content.one().map(<crate::fields::DateTime>::from),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
