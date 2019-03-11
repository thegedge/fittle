use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BatteryStatus {
    Charging,
    Critical,
    Good,
    Low,
    New,
    Ok,
    Unknown,
    UnknownValue(u64),
}

impl From<FieldContent> for BatteryStatus {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt8(enum_value) = field {
            match enum_value {
                1 => BatteryStatus::New,
                2 => BatteryStatus::Good,
                3 => BatteryStatus::Ok,
                4 => BatteryStatus::Low,
                5 => BatteryStatus::Critical,
                6 => BatteryStatus::Charging,
                7 => BatteryStatus::Unknown,
                n => BatteryStatus::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert BatteryStatus to {:?}", field);
        }
    }
}
