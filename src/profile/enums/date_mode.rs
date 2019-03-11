use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DateMode {
    DayMonth,
    MonthDay,
    UnknownValue(u64),
}

impl From<FieldContent> for DateMode {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => DateMode::DayMonth,
                1 => DateMode::MonthDay,
                n => DateMode::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert DateMode to {:?}", field);
        }
    }
}
