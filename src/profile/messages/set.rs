// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct Set {
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<Vec<crate::profile::enums::ExerciseCategory>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    category_subtype: Option<Vec<u16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    repetitions: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    set_type: Option<crate::profile::enums::SetType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<crate::fields::Mass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    weight_display_unit: Option<crate::profile::enums::FitBaseUnit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    wkt_step_index: Option<crate::profile::enums::MessageIndex>,
}

impl Set {
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
                0 => msg.duration = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                3 => msg.repetitions = content.one().map(<u16>::from),
                4 => msg.weight = content.one().map(|v| crate::fields::Mass::new::<uom::si::mass::kilogram, f64>((|v| { <f64>::from(<u16>::from(v)) / 16.0 - 0.0 })(v))),
                5 => msg.set_type = content.one().map(<crate::profile::enums::SetType>::from),
                6 => msg.start_time = content.one().map(<crate::fields::DateTime>::from),
                7 => msg.category = content.many().map(|vec| vec.into_iter().map(<crate::profile::enums::ExerciseCategory>::from).collect()),
                8 => msg.category_subtype = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                9 => msg.weight_display_unit = content.one().map(<crate::profile::enums::FitBaseUnit>::from),
                10 => msg.message_index = content.one().map(<crate::profile::enums::MessageIndex>::from),
                11 => msg.wkt_step_index = content.one().map(<crate::profile::enums::MessageIndex>::from),
                254 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
