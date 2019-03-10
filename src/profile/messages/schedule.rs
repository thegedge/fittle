// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};
use serde::Serialize;

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct Schedule {
    #[serde(skip_serializing_if = "Option::is_none")]
    manufacturer: Option<enums::Manufacturer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    product: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    serial_number: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_created: Option<enums::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    completed: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<enums::Schedule>,

    #[serde(skip_serializing_if = "Option::is_none")]
    scheduled_time: Option<enums::LocalDateTime>,

}

impl Schedule {
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
                0 => msg.manufacturer = content.one().map(<enums::Manufacturer>::from),
                1 => msg.product = content.one().map(<u16>::from),
                2 => msg.serial_number = content.one().map(<u32>::from),
                3 => msg.time_created = content.one().map(<enums::DateTime>::from),
                4 => msg.completed = content.one().map(<bool>::from),
                5 => msg.type_ = content.one().map(<enums::Schedule>::from),
                6 => msg.scheduled_time = content.one().map(<enums::LocalDateTime>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

