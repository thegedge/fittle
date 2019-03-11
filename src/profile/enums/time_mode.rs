use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeMode {
    Hour12,
    Hour12WithSeconds,
    Hour24,
    Hour24WithSeconds,
    Military,
    Utc,
    UnknownValue(u64),
}

impl From<FieldContent> for TimeMode {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => TimeMode::Hour12,
                1 => TimeMode::Hour24,
                2 => TimeMode::Military,
                3 => TimeMode::Hour12WithSeconds,
                4 => TimeMode::Hour24WithSeconds,
                5 => TimeMode::Utc,
                n => TimeMode::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert TimeMode to {:?}", field);
        }
    }
}
