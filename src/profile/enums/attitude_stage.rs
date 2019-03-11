use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AttitudeStage {
    Aligning,
    Degraded,
    Failed,
    Valid,
    UnknownValue(u64),
}

impl From<FieldContent> for AttitudeStage {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => AttitudeStage::Failed,
                1 => AttitudeStage::Aligning,
                2 => AttitudeStage::Degraded,
                3 => AttitudeStage::Valid,
                n => AttitudeStage::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert AttitudeStage to {:?}", field);
        }
    }
}
