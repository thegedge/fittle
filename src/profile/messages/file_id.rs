// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct FileId {
    type_: Option<enums::File>,
    manufacturer: Option<enums::Manufacturer>,
    product: Option<u16>,
    serial_number: Option<u32>,
    time_created: Option<enums::DateTime>,
    number: Option<u16>,
    product_name: Option<String>,
}

impl FileId {
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
                0 => msg.type_ = content.one().map(<enums::File>::from),
                1 => msg.manufacturer = content.one().map(<enums::Manufacturer>::from),
                2 => msg.product = content.one().map(<u16>::from),
                3 => msg.serial_number = content.one().map(<u32>::from),
                4 => msg.time_created = content.one().map(<enums::DateTime>::from),
                5 => msg.number = content.one().map(<u16>::from),
                8 => msg.product_name = content.one().map(<String>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

