use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageIndex {
    Mask,
    Reserved,
    Selected,
    UnknownValue(u64),
}

impl From<FieldContent> for MessageIndex {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                4095 => MessageIndex::Mask,
                28672 => MessageIndex::Reserved,
                32768 => MessageIndex::Selected,
                n => MessageIndex::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert MessageIndex to {:?}", field);
        }
    }
}
