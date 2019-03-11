use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FitBaseType {
    Byte,
    Enum,
    Float32,
    Float64,
    Sint16,
    Sint32,
    Sint64,
    Sint8,
    String,
    Uint16,
    Uint16Z,
    Uint32,
    Uint32Z,
    Uint64,
    Uint64Z,
    Uint8,
    Uint8Z,
    UnknownValue(u64),
}

impl From<FieldContent> for FitBaseType {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt8(enum_value) = field {
            match enum_value {
                0 => FitBaseType::Enum,
                1 => FitBaseType::Sint8,
                2 => FitBaseType::Uint8,
                7 => FitBaseType::String,
                10 => FitBaseType::Uint8Z,
                13 => FitBaseType::Byte,
                131 => FitBaseType::Sint16,
                132 => FitBaseType::Uint16,
                133 => FitBaseType::Sint32,
                134 => FitBaseType::Uint32,
                136 => FitBaseType::Float32,
                137 => FitBaseType::Float64,
                139 => FitBaseType::Uint16Z,
                140 => FitBaseType::Uint32Z,
                142 => FitBaseType::Sint64,
                143 => FitBaseType::Uint64,
                144 => FitBaseType::Uint64Z,
                n => FitBaseType::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert FitBaseType to {:?}", field);
        }
    }
}
