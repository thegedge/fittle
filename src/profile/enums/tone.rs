use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Tone {
    Off,
    Tone,
    ToneAndVibrate,
    Vibrate,
    UnknownValue(u64),
}

impl From<FieldContent> for Tone {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => Tone::Off,
                1 => Tone::Tone,
                2 => Tone::Vibrate,
                3 => Tone::ToneAndVibrate,
                n => Tone::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert Tone to {:?}", field);
        }
    }
}
