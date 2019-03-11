use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GoalRecurrence {
    Custom,
    Daily,
    Monthly,
    Off,
    Weekly,
    Yearly,
    UnknownValue(u64),
}

impl From<FieldContent> for GoalRecurrence {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => GoalRecurrence::Off,
                1 => GoalRecurrence::Daily,
                2 => GoalRecurrence::Weekly,
                3 => GoalRecurrence::Monthly,
                4 => GoalRecurrence::Yearly,
                5 => GoalRecurrence::Custom,
                n => GoalRecurrence::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert GoalRecurrence to {:?}", field);
        }
    }
}
