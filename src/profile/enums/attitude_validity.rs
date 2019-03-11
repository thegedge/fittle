use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AttitudeValidity {
    GpsInvalid,
    HwFail,
    LateralBodyAccelValid,
    MagInvalid,
    MagneticHeading,
    NoGps,
    NormalBodyAccelValid,
    PitchValid,
    RollValid,
    SolutionCoasting,
    TrackAngleHeadingValid,
    TrueTrackAngle,
    TurnRateValid,
    UnknownValue(u64),
}

impl From<FieldContent> for AttitudeValidity {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt16(enum_value) = field {
            match enum_value {
                1 => AttitudeValidity::TrackAngleHeadingValid,
                2 => AttitudeValidity::PitchValid,
                4 => AttitudeValidity::RollValid,
                8 => AttitudeValidity::LateralBodyAccelValid,
                16 => AttitudeValidity::NormalBodyAccelValid,
                32 => AttitudeValidity::TurnRateValid,
                64 => AttitudeValidity::HwFail,
                128 => AttitudeValidity::MagInvalid,
                256 => AttitudeValidity::NoGps,
                512 => AttitudeValidity::GpsInvalid,
                1024 => AttitudeValidity::SolutionCoasting,
                2048 => AttitudeValidity::TrueTrackAngle,
                4096 => AttitudeValidity::MagneticHeading,
                n => AttitudeValidity::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert AttitudeValidity to {:?}", field);
        }
    }
}
