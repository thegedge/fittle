use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SubSport {
    All,
    ApneaDiving,
    ApneaHunting,
    Atv,
    Backcountry,
    BikeToRunTransition,
    Bmx,
    CardioTraining,
    CasualWalking,
    Challenge,
    Commuting,
    Cyclocross,
    Downhill,
    EBikeFitness,
    EBikeMountain,
    Elliptical,
    Exercise,
    FlexibilityTraining,
    GaugeDiving,
    Generic,
    GravelCycling,
    HandCycling,
    IndoorCycling,
    IndoorRowing,
    IndoorRunning,
    IndoorSkiing,
    IndoorWalking,
    LapSwimming,
    Map,
    Match,
    MixedSurface,
    Motocross,
    Mountain,
    MultiGasDiving,
    Navigate,
    Obstacle,
    OpenWater,
    Pilates,
    RcDrone,
    Recumbent,
    Resort,
    Road,
    RunToBikeTransition,
    SingleGasDiving,
    SkateSkiing,
    SpeedWalking,
    Spin,
    StairClimbing,
    Street,
    StrengthTraining,
    SwimToBikeTransition,
    Track,
    TrackCycling,
    TrackMe,
    Trail,
    Treadmill,
    VirtualActivity,
    WarmUp,
    Whitewater,
    Wingsuit,
    Yoga,
    UnknownValue(u64),
}

impl From<FieldContent> for SubSport {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => SubSport::Generic,
                1 => SubSport::Treadmill,
                2 => SubSport::Street,
                3 => SubSport::Trail,
                4 => SubSport::Track,
                5 => SubSport::Spin,
                6 => SubSport::IndoorCycling,
                7 => SubSport::Road,
                8 => SubSport::Mountain,
                9 => SubSport::Downhill,
                10 => SubSport::Recumbent,
                11 => SubSport::Cyclocross,
                12 => SubSport::HandCycling,
                13 => SubSport::TrackCycling,
                14 => SubSport::IndoorRowing,
                15 => SubSport::Elliptical,
                16 => SubSport::StairClimbing,
                17 => SubSport::LapSwimming,
                18 => SubSport::OpenWater,
                19 => SubSport::FlexibilityTraining,
                20 => SubSport::StrengthTraining,
                21 => SubSport::WarmUp,
                22 => SubSport::Match,
                23 => SubSport::Exercise,
                24 => SubSport::Challenge,
                25 => SubSport::IndoorSkiing,
                26 => SubSport::CardioTraining,
                27 => SubSport::IndoorWalking,
                28 => SubSport::EBikeFitness,
                29 => SubSport::Bmx,
                30 => SubSport::CasualWalking,
                31 => SubSport::SpeedWalking,
                32 => SubSport::BikeToRunTransition,
                33 => SubSport::RunToBikeTransition,
                34 => SubSport::SwimToBikeTransition,
                35 => SubSport::Atv,
                36 => SubSport::Motocross,
                37 => SubSport::Backcountry,
                38 => SubSport::Resort,
                39 => SubSport::RcDrone,
                40 => SubSport::Wingsuit,
                41 => SubSport::Whitewater,
                42 => SubSport::SkateSkiing,
                43 => SubSport::Yoga,
                44 => SubSport::Pilates,
                45 => SubSport::IndoorRunning,
                46 => SubSport::GravelCycling,
                47 => SubSport::EBikeMountain,
                48 => SubSport::Commuting,
                49 => SubSport::MixedSurface,
                50 => SubSport::Navigate,
                51 => SubSport::TrackMe,
                52 => SubSport::Map,
                53 => SubSport::SingleGasDiving,
                54 => SubSport::MultiGasDiving,
                55 => SubSport::GaugeDiving,
                56 => SubSport::ApneaDiving,
                57 => SubSport::ApneaHunting,
                58 => SubSport::VirtualActivity,
                59 => SubSport::Obstacle,
                254 => SubSport::All,
                n => SubSport::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert SubSport to {:?}", field);
        }
    }
}
