// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

#[derive(Debug, Default)]
pub struct WorkoutStep {
    message_index: Option<enums::MessageIndex>,
    wkt_step_name: Option<String>,
    duration_type: Option<enums::WktStepDuration>,
    duration_value: Option<u32>,
    target_type: Option<enums::WktStepTarget>,
    target_value: Option<u32>,
    custom_target_value_low: Option<u32>,
    custom_target_value_high: Option<u32>,
    intensity: Option<enums::Intensity>,
    notes: Option<String>,
    equipment: Option<enums::WorkoutEquipment>,
    exercise_category: Option<enums::ExerciseCategory>,
    exercise_name: Option<u16>,
    exercise_weight: Option<u16>,
    weight_display_unit: Option<enums::FitBaseUnit>,
}

impl WorkoutStep {
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
                0 => msg.wkt_step_name = content.one().map(<String>::from),
                1 => msg.duration_type = content.one().map(<enums::WktStepDuration>::from),
                2 => msg.duration_value = content.one().map(<u32>::from),
                3 => msg.target_type = content.one().map(<enums::WktStepTarget>::from),
                4 => msg.target_value = content.one().map(<u32>::from),
                5 => msg.custom_target_value_low = content.one().map(<u32>::from),
                6 => msg.custom_target_value_high = content.one().map(<u32>::from),
                7 => msg.intensity = content.one().map(<enums::Intensity>::from),
                8 => msg.notes = content.one().map(<String>::from),
                9 => msg.equipment = content.one().map(<enums::WorkoutEquipment>::from),
                10 => msg.exercise_category = content.one().map(<enums::ExerciseCategory>::from),
                11 => msg.exercise_name = content.one().map(<u16>::from),
                12 => msg.exercise_weight = content.one().map(<u16>::from),
                13 => msg.weight_display_unit = content.one().map(<enums::FitBaseUnit>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

