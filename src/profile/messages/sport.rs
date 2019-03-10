// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct Sport {
    sport: Option<enums::Sport>,
    sub_sport: Option<enums::SubSport>,
    name: Option<String>,
}

impl Sport {
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
                0 => msg.sport = content.one().map(<enums::Sport>::from),
                1 => msg.sub_sport = content.one().map(<enums::SubSport>::from),
                3 => msg.name = content.one().map(<String>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

