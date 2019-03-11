use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MesgCount {
    MaxPerFile,
    MaxPerFileType,
    NumPerFile,
    UnknownValue(u64),
}

impl From<FieldContent> for MesgCount {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => MesgCount::NumPerFile,
                1 => MesgCount::MaxPerFile,
                2 => MesgCount::MaxPerFileType,
                n => MesgCount::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert MesgCount to {:?}", field);
        }
    }
}
