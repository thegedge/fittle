use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Goal {
    ActiveMinutes,
    Ascent,
    Calories,
    Distance,
    Frequency,
    Steps,
    Time,
    UnknownValue(u64),
}

impl From<FieldContent> for Goal {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => Goal::Time,
                1 => Goal::Distance,
                2 => Goal::Calories,
                3 => Goal::Frequency,
                4 => Goal::Steps,
                5 => Goal::Ascent,
                6 => Goal::ActiveMinutes,
                n => Goal::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert Goal to {:?}", field);
        }
    }
}
