// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct Course {
    sport: Option<enums::Sport>,
    name: Option<String>,
    capabilities: Option<enums::CourseCapabilities>,
    sub_sport: Option<enums::SubSport>,
}

impl Course {
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
                4 => msg.sport = content.one().map(<enums::Sport>::from),
                5 => msg.name = content.one().map(<String>::from),
                6 => msg.capabilities = content.one().map(<enums::CourseCapabilities>::from),
                7 => msg.sub_sport = content.one().map(<enums::SubSport>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

