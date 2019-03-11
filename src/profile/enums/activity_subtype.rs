use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivitySubtype {
    All,
    Cyclocross,
    Downhill,
    Elliptical,
    Generic,
    HandCycling,
    IndoorCycling,
    IndoorRowing,
    LapSwimming,
    Mountain,
    OpenWater,
    Recumbent,
    Road,
    Spin,
    StairClimbing,
    Street,
    Track,
    TrackCycling,
    Trail,
    Treadmill,
    UnknownValue(u64),
}

impl From<FieldContent> for ActivitySubtype {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => ActivitySubtype::Generic,
                1 => ActivitySubtype::Treadmill,
                2 => ActivitySubtype::Street,
                3 => ActivitySubtype::Trail,
                4 => ActivitySubtype::Track,
                5 => ActivitySubtype::Spin,
                6 => ActivitySubtype::IndoorCycling,
                7 => ActivitySubtype::Road,
                8 => ActivitySubtype::Mountain,
                9 => ActivitySubtype::Downhill,
                10 => ActivitySubtype::Recumbent,
                11 => ActivitySubtype::Cyclocross,
                12 => ActivitySubtype::HandCycling,
                13 => ActivitySubtype::TrackCycling,
                14 => ActivitySubtype::IndoorRowing,
                15 => ActivitySubtype::Elliptical,
                16 => ActivitySubtype::StairClimbing,
                17 => ActivitySubtype::LapSwimming,
                18 => ActivitySubtype::OpenWater,
                254 => ActivitySubtype::All,
                n => ActivitySubtype::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert ActivitySubtype to {:?}", field);
        }
    }
}
