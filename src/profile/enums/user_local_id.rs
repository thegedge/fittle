use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserLocalId {
    LocalMax,
    LocalMin,
    PortableMax,
    PortableMin,
    StationaryMax,
    StationaryMin,
    UnknownValue(u64),
}

impl From<FieldContent> for UserLocalId {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => UserLocalId::LocalMin,
                15 => UserLocalId::LocalMax,
                16 => UserLocalId::StationaryMin,
                255 => UserLocalId::StationaryMax,
                256 => UserLocalId::PortableMin,
                65534 => UserLocalId::PortableMax,
                n => UserLocalId::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert UserLocalId to {:?}", field);
        }
    }
}
