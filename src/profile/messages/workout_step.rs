// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

#[allow(unused_imports)]
use crate::bits::BitReader;

#[allow(unused_imports)]
use crate::fields::{
    Field,
    FieldContent,
    FieldDefinition,
};

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
    pub fn from_fields<Order, Reader>(reader: &mut Reader, field_defs: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut msg: Self = Default::default();
        for field_def in field_defs {
            let (number, field) = field_def.content_from::<Order, Reader>(reader)?;
            msg.from_content(number, field);
        }

        Ok(msg)
    }

    fn from_content(&mut self, number: u8, field: Field) {
        match number {
            0 => {
                self.wkt_step_name =field.one().map(|v| {
                    let value = String::from(v);
                    value
                })
            },

            1 => {
                self.duration_type =field.one().map(|v| {
                    let value = crate::profile::enums::WktStepDuration::from(v);
                    value
                })
            },

            2 => {
                self.duration_value =field.one().map(|v| {
                    let value = u32::from(v);
                    value
                })
            },

            3 => {
                self.target_type =field.one().map(|v| {
                    let value = crate::profile::enums::WktStepTarget::from(v);
                    value
                })
            },

            4 => {
                self.target_value =field.one().map(|v| {
                    let value = u32::from(v);
                    value
                })
            },

            5 => {
                self.custom_target_value_low =field.one().map(|v| {
                    let value = u32::from(v);
                    value
                })
            },

            6 => {
                self.custom_target_value_high =field.one().map(|v| {
                    let value = u32::from(v);
                    value
                })
            },

            7 => {
                self.intensity =field.one().map(|v| {
                    let value = crate::profile::enums::Intensity::from(v);
                    value
                })
            },

            8 => {
                self.notes =field.one().map(|v| {
                    let value = String::from(v);
                    value
                })
            },

            9 => {
                self.equipment =field.one().map(|v| {
                    let value = crate::profile::enums::WorkoutEquipment::from(v);
                    value
                })
            },

            10 => {
                self.exercise_category =field.one().map(|v| {
                    let value = crate::profile::enums::ExerciseCategory::from(v);
                    value
                })
            },

            11 => {
                self.exercise_name =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            12 => {
                self.exercise_weight =field.one().map(|v| {
                    let value = u16::from(v);
                    (|v| crate::fields::Mass::new::<uom::si::mass::kilogram, f64>((|v| { f64::from(v) / 100.0 - 0.0 })(v)))(value)
                })
            },

            13 => {
                self.weight_display_unit =field.one().map(|v| {
                    let value = crate::profile::enums::FitBaseUnit::from(v);
                    value
                })
            },

            254 => {
                self.message_index =field.one().map(|v| {
                    let value = crate::profile::enums::MessageIndex::from(v);
                    value
                })
            },

            _ => (),
        }
    }
}
