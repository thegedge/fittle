use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SwimStroke {
    Backstroke,
    Breaststroke,
    Butterfly,
    Drill,
    Freestyle,
    Im,
    Mixed,
    UnknownValue(u64),
}

impl From<FieldContent> for SwimStroke {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => SwimStroke::Freestyle,
                1 => SwimStroke::Backstroke,
                2 => SwimStroke::Breaststroke,
                3 => SwimStroke::Butterfly,
                4 => SwimStroke::Drill,
                5 => SwimStroke::Mixed,
                6 => SwimStroke::Im,
                n => SwimStroke::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert SwimStroke to {:?}", field);
        }
    }
}
