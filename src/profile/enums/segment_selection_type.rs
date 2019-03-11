use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SegmentSelectionType {
    Starred,
    Suggested,
    UnknownValue(u64),
}

impl From<FieldContent> for SegmentSelectionType {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => SegmentSelectionType::Starred,
                1 => SegmentSelectionType::Suggested,
                n => SegmentSelectionType::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert SegmentSelectionType to {:?}", field);
        }
    }
}
