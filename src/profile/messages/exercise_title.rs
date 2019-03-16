// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct ExerciseTitle {
    #[serde(skip_serializing_if = "Option::is_none")]
    exercise_category: Option<crate::profile::enums::ExerciseCategory>,

    #[serde(skip_serializing_if = "Option::is_none")]
    exercise_name: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    wkt_step_name: Option<Vec<String>>,
}

impl ExerciseTitle {
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
                0 => msg.exercise_category = content.one().map(<crate::profile::enums::ExerciseCategory>::from),
                1 => msg.exercise_name = content.one().map(<u16>::from),
                2 => msg.wkt_step_name = content.many().map(|vec| vec.into_iter().map(<String>::from).collect()),
                254 => msg.message_index = content.one().map(<crate::profile::enums::MessageIndex>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
