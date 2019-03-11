use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CameraOrientationType {
    CameraOrientation0,
    CameraOrientation180,
    CameraOrientation270,
    CameraOrientation90,
    UnknownValue(u64),
}

impl From<FieldContent> for CameraOrientationType {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => CameraOrientationType::CameraOrientation0,
                1 => CameraOrientationType::CameraOrientation90,
                2 => CameraOrientationType::CameraOrientation180,
                3 => CameraOrientationType::CameraOrientation270,
                n => CameraOrientationType::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert CameraOrientationType to {:?}", field);
        }
    }
}
