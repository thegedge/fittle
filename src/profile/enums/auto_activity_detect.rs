use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AutoActivityDetect {
    Cycling,
    Elliptical,
    None,
    Running,
    Sedentary,
    Swimming,
    Walking,
    UnknownValue(u64),
}

impl From<FieldContent> for AutoActivityDetect {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt32(enum_value) = field {
            match enum_value {
                0 => AutoActivityDetect::None,
                1 => AutoActivityDetect::Running,
                2 => AutoActivityDetect::Cycling,
                4 => AutoActivityDetect::Swimming,
                8 => AutoActivityDetect::Walking,
                32 => AutoActivityDetect::Elliptical,
                1024 => AutoActivityDetect::Sedentary,
                n => AutoActivityDetect::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert AutoActivityDetect to {:?}", field);
        }
    }
}
