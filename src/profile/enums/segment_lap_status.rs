use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SegmentLapStatus {
    End,
    Fail,
    UnknownValue(u64),
}

impl From<FieldContent> for SegmentLapStatus {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => SegmentLapStatus::End,
                1 => SegmentLapStatus::Fail,
                n => SegmentLapStatus::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert SegmentLapStatus to {:?}", field);
        }
    }
}
