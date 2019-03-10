// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct CoursePoint {
    message_index: Option<enums::MessageIndex>,
    timestamp: Option<enums::DateTime>,
    position_lat: Option<i32>,
    position_long: Option<i32>,
    distance: Option<u32>,
    type_: Option<enums::CoursePoint>,
    name: Option<String>,
    favorite: Option<bool>,
}

impl CoursePoint {
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
                1 => msg.timestamp = content.one().map(<enums::DateTime>::from),
                2 => msg.position_lat = content.one().map(<i32>::from),
                3 => msg.position_long = content.one().map(<i32>::from),
                4 => msg.distance = content.one().map(<u32>::from),
                5 => msg.type_ = content.one().map(<enums::CoursePoint>::from),
                6 => msg.name = content.one().map(<String>::from),
                8 => msg.favorite = content.one().map(<bool>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

