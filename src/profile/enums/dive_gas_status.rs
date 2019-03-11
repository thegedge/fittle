use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiveGasStatus {
    BackupOnly,
    Disabled,
    Enabled,
    UnknownValue(u64),
}

impl From<FieldContent> for DiveGasStatus {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => DiveGasStatus::Disabled,
                1 => DiveGasStatus::Enabled,
                2 => DiveGasStatus::BackupOnly,
                n => DiveGasStatus::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert DiveGasStatus to {:?}", field);
        }
    }
}
