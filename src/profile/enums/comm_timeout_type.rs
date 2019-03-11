use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CommTimeoutType {
    ConnectionLost,
    ConnectionTimeout,
    PairingTimeout,
    WildcardPairingTimeout,
    UnknownValue(u64),
}

impl From<FieldContent> for CommTimeoutType {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                0 => CommTimeoutType::WildcardPairingTimeout,
                1 => CommTimeoutType::PairingTimeout,
                2 => CommTimeoutType::ConnectionLost,
                3 => CommTimeoutType::ConnectionTimeout,
                n => CommTimeoutType::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert CommTimeoutType to {:?}", field);
        }
    }
}
