use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalDeviceType {
    UnknownValue(u64),
}

impl From<FieldContent> for LocalDeviceType {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt8(enum_value) = field {
            match enum_value {
                n => LocalDeviceType::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert LocalDeviceType to {:?}", field);
        }
    }
}
