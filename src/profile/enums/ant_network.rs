use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AntNetwork {
    Antfs,
    Antplus,
    Private,
    Public,
    UnknownValue(u64),
}

impl From<FieldContent> for AntNetwork {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => AntNetwork::Public,
                1 => AntNetwork::Antplus,
                2 => AntNetwork::Antfs,
                3 => AntNetwork::Private,
                n => AntNetwork::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert AntNetwork to {:?}", field);
        }
    }
}
