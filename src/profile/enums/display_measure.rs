use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DisplayMeasure {
    Metric,
    Nautical,
    Statute,
    UnknownValue(u64),
}

impl From<FieldContent> for DisplayMeasure {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => DisplayMeasure::Metric,
                1 => DisplayMeasure::Statute,
                2 => DisplayMeasure::Nautical,
                n => DisplayMeasure::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert DisplayMeasure to {:?}", field);
        }
    }
}
