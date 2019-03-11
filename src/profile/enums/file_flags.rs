use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FileFlags {
    Erase,
    Read,
    Write,
    UnknownValue(u64),
}

impl From<FieldContent> for FileFlags {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt8z(enum_value) = field {
            match enum_value {
                2 => FileFlags::Read,
                4 => FileFlags::Write,
                8 => FileFlags::Erase,
                n => FileFlags::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert FileFlags to {:?}", field);
        }
    }
}
