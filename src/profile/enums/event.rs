use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Event {
    Activity,
    Battery,
    BatteryLow,
    CadHighAlert,
    CadLowAlert,
    Calibration,
    CalorieDurationAlert,
    CommTimeout,
    CoursePoint,
    DistanceDurationAlert,
    ElevHighAlert,
    ElevLowAlert,
    FitnessEquipment,
    FrontGearChange,
    HrHighAlert,
    HrLowAlert,
    Lap,
    Length,
    OffCourse,
    PowerDown,
    PowerHighAlert,
    PowerLowAlert,
    PowerUp,
    RearGearChange,
    RecoveryHr,
    RiderPositionChange,
    Session,
    SpeedHighAlert,
    SpeedLowAlert,
    SportPoint,
    TimeDurationAlert,
    Timer,
    UserMarker,
    VirtualPartnerPace,
    Workout,
    WorkoutStep,
    UnknownValue(u64),
}

impl From<FieldContent> for Event {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => Event::Timer,
                3 => Event::Workout,
                4 => Event::WorkoutStep,
                5 => Event::PowerDown,
                6 => Event::PowerUp,
                7 => Event::OffCourse,
                8 => Event::Session,
                9 => Event::Lap,
                10 => Event::CoursePoint,
                11 => Event::Battery,
                12 => Event::VirtualPartnerPace,
                13 => Event::HrHighAlert,
                14 => Event::HrLowAlert,
                15 => Event::SpeedHighAlert,
                16 => Event::SpeedLowAlert,
                17 => Event::CadHighAlert,
                18 => Event::CadLowAlert,
                19 => Event::PowerHighAlert,
                20 => Event::PowerLowAlert,
                21 => Event::RecoveryHr,
                22 => Event::BatteryLow,
                23 => Event::TimeDurationAlert,
                24 => Event::DistanceDurationAlert,
                25 => Event::CalorieDurationAlert,
                26 => Event::Activity,
                27 => Event::FitnessEquipment,
                28 => Event::Length,
                32 => Event::UserMarker,
                33 => Event::SportPoint,
                36 => Event::Calibration,
                42 => Event::FrontGearChange,
                43 => Event::RearGearChange,
                44 => Event::RiderPositionChange,
                45 => Event::ElevHighAlert,
                46 => Event::ElevLowAlert,
                47 => Event::CommTimeout,
                n => Event::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert Event to {:?}", field);
        }
    }
}
