use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SegmentDeleteStatus {
    DeleteAll,
    DeleteOne,
    DoNotDelete,
    UnknownValue(u64),
}

impl From<FieldContent> for SegmentDeleteStatus {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => SegmentDeleteStatus::DoNotDelete,
                1 => SegmentDeleteStatus::DeleteOne,
                2 => SegmentDeleteStatus::DeleteAll,
                n => SegmentDeleteStatus::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert SegmentDeleteStatus to {:?}", field);
        }
    }
}
