// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct WorkoutStep {
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_target_value_high: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    custom_target_value_low: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    duration_type: Option<crate::profile::enums::WktStepDuration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    duration_value: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    equipment: Option<crate::profile::enums::WorkoutEquipment>,

    #[serde(skip_serializing_if = "Option::is_none")]
    exercise_category: Option<crate::profile::enums::ExerciseCategory>,

    #[serde(skip_serializing_if = "Option::is_none")]
    exercise_name: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    exercise_weight: Option<crate::fields::Mass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    intensity: Option<crate::profile::enums::Intensity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    message_index: Option<crate::profile::enums::MessageIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    notes: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    target_type: Option<crate::profile::enums::WktStepTarget>,

    #[serde(skip_serializing_if = "Option::is_none")]
    target_value: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    weight_display_unit: Option<crate::profile::enums::FitBaseUnit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    wkt_step_name: Option<String>,
}

impl WorkoutStep {
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
                0 => msg.wkt_step_name = content.one().map(<String>::from),
                1 => msg.duration_type = content.one().map(<crate::profile::enums::WktStepDuration>::from),
                2 => msg.duration_value = content.one().map(<u32>::from),
                3 => msg.target_type = content.one().map(<crate::profile::enums::WktStepTarget>::from),
                4 => msg.target_value = content.one().map(<u32>::from),
                5 => msg.custom_target_value_low = content.one().map(<u32>::from),
                6 => msg.custom_target_value_high = content.one().map(<u32>::from),
                7 => msg.intensity = content.one().map(<crate::profile::enums::Intensity>::from),
                8 => msg.notes = content.one().map(<String>::from),
                9 => msg.equipment = content.one().map(<crate::profile::enums::WorkoutEquipment>::from),
                10 => msg.exercise_category = content.one().map(<crate::profile::enums::ExerciseCategory>::from),
                11 => msg.exercise_name = content.one().map(<u16>::from),
                12 => msg.exercise_weight = content.one().map(|v| crate::fields::Mass::new::<uom::si::mass::kilogram, f64>((|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 })(v))),
                13 => msg.weight_display_unit = content.one().map(<crate::profile::enums::FitBaseUnit>::from),
                254 => msg.message_index = content.one().map(<crate::profile::enums::MessageIndex>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
