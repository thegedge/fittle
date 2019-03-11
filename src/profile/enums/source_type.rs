use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    Ant,
    Antplus,
    Bluetooth,
    BluetoothLowEnergy,
    Local,
    Wifi,
    UnknownValue(u64),
}

impl From<FieldContent> for SourceType {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => SourceType::Ant,
                1 => SourceType::Antplus,
                2 => SourceType::Bluetooth,
                3 => SourceType::BluetoothLowEnergy,
                4 => SourceType::Wifi,
                5 => SourceType::Local,
                n => SourceType::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert SourceType to {:?}", field);
        }
    }
}
