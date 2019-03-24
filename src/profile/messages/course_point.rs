// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct CoursePoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    distance: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    favorite: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    position_lat: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    position_long: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<crate::profile::enums::CoursePoint>,
}

impl CoursePoint {
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
                1 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                2 => msg.position_lat = content.one().map(<i32>::from),
                3 => msg.position_long = content.one().map(<i32>::from),
                4 => msg.distance = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u32>::from(v)) / 100.0 - 0.0 })(v))),
                5 => msg.type_ = content.one().map(<crate::profile::enums::CoursePoint>::from),
                6 => msg.name = content.one().map(<String>::from),
                8 => msg.favorite = content.one().map(<bool>::from),
                254 => msg.message_index = content.one().map(<crate::profile::enums::MessageIndex>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
