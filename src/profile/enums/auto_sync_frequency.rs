use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AutoSyncFrequency {
    Frequent,
    Never,
    Occasionally,
    OnceADay,
    Remote,
    UnknownValue(u64),
}

impl From<FieldContent> for AutoSyncFrequency {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => AutoSyncFrequency::Never,
                1 => AutoSyncFrequency::Occasionally,
                2 => AutoSyncFrequency::Frequent,
                3 => AutoSyncFrequency::OnceADay,
                4 => AutoSyncFrequency::Remote,
                n => AutoSyncFrequency::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert AutoSyncFrequency to {:?}", field);
        }
    }
}
