use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Checksum {
    Clear,
    Ok,
    UnknownValue(u64),
}

impl From<FieldContent> for Checksum {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt8(enum_value) = field {
            match enum_value {
                0 => Checksum::Clear,
                1 => Checksum::Ok,
                n => Checksum::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert Checksum to {:?}", field);
        }
    }
}
