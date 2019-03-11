use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExdDescriptors {
    AmbientPressure,
    AnaerobicTrainingEffect,
    Ascent,
    Balance,
    BateryLevel,
    BatteryLevel,
    BeamAngleStatus,
    BikeLightBatteryStatus,
    Cadence,
    Calories,
    Compass,
    CompassHeading,
    Course,
    CourseDistance,
    CourseEstimatedTimeOfArrival,
    CourseHeading,
    CourseLocation,
    CourseTime,
    CourseType,
    Descent,
    Di2BatteryLevel,
    Distance,
    Elevation,
    EstimatedTimeOfArrival,
    FrontGear,
    FunctionalThresholdPower,
    GearCombo,
    GearRatio,
    Gears,
    GlideRatio,
    GpsAccuracy,
    GpsElevation,
    GpsHeading,
    GpsSignalStrength,
    Grade,
    GroundContactTime,
    Heading,
    HeartRate,
    HeartRateReserve,
    HeartRateZone,
    Icon,
    IntensityFactor,
    Laps,
    LeftGroundContactTimeBalance,
    LeftPlatformCenterOffset,
    LeftPowerPhaseFinishAngle,
    LeftPowerPhaseStartAngle,
    LightNetworkMode,
    MuscleOxygen,
    NavigationDistance,
    NavigationEstimatedTimeOfArrival,
    NavigationHeading,
    NavigationLocation,
    NavigationTime,
    NavigationTurn,
    NormalizedPower,
    NumberLightsConnected,
    OffCourse,
    Pace,
    PedalSmoothness,
    PerformanceCondition,
    Power,
    PowerRatio,
    PowerWeightRatio,
    PowerZone,
    Pressure,
    RearGear,
    Reps,
    RightGroundContactTimeBalance,
    RightPlatformCenterOffset,
    RightPowerPhaseFinishAngle,
    RightPowerPhaseStartAngle,
    RunningCadence,
    Speed,
    StrideLength,
    Temperature,
    Time,
    TimeInHeartRateZone,
    TimeInPowerZone,
    TimeOfDay,
    TimeOnZone,
    TimeSeated,
    TimeStanding,
    TimerTime,
    TorqueEffectiveness,
    TrainerResistance,
    TrainerTargetPower,
    TrainingEffect,
    TrainingStressScore,
    Vam,
    VerticalDistance,
    VerticalOscillation,
    VerticalRatio,
    VerticalSpeed,
    Vmg,
    Work,
    WorkoutStep,
    UnknownValue(u64),
}

impl From<FieldContent> for ExdDescriptors {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => ExdDescriptors::BikeLightBatteryStatus,
                1 => ExdDescriptors::BeamAngleStatus,
                2 => ExdDescriptors::BateryLevel,
                3 => ExdDescriptors::LightNetworkMode,
                4 => ExdDescriptors::NumberLightsConnected,
                5 => ExdDescriptors::Cadence,
                6 => ExdDescriptors::Distance,
                7 => ExdDescriptors::EstimatedTimeOfArrival,
                8 => ExdDescriptors::Heading,
                9 => ExdDescriptors::Time,
                10 => ExdDescriptors::BatteryLevel,
                11 => ExdDescriptors::TrainerResistance,
                12 => ExdDescriptors::TrainerTargetPower,
                13 => ExdDescriptors::TimeSeated,
                14 => ExdDescriptors::TimeStanding,
                15 => ExdDescriptors::Elevation,
                16 => ExdDescriptors::Grade,
                17 => ExdDescriptors::Ascent,
                18 => ExdDescriptors::Descent,
                19 => ExdDescriptors::VerticalSpeed,
                20 => ExdDescriptors::Di2BatteryLevel,
                21 => ExdDescriptors::FrontGear,
                22 => ExdDescriptors::RearGear,
                23 => ExdDescriptors::GearRatio,
                24 => ExdDescriptors::HeartRate,
                25 => ExdDescriptors::HeartRateZone,
                26 => ExdDescriptors::TimeInHeartRateZone,
                27 => ExdDescriptors::HeartRateReserve,
                28 => ExdDescriptors::Calories,
                29 => ExdDescriptors::GpsAccuracy,
                30 => ExdDescriptors::GpsSignalStrength,
                31 => ExdDescriptors::Temperature,
                32 => ExdDescriptors::TimeOfDay,
                33 => ExdDescriptors::Balance,
                34 => ExdDescriptors::PedalSmoothness,
                35 => ExdDescriptors::Power,
                36 => ExdDescriptors::FunctionalThresholdPower,
                37 => ExdDescriptors::IntensityFactor,
                38 => ExdDescriptors::Work,
                39 => ExdDescriptors::PowerRatio,
                40 => ExdDescriptors::NormalizedPower,
                41 => ExdDescriptors::TrainingStressScore,
                42 => ExdDescriptors::TimeOnZone,
                43 => ExdDescriptors::Speed,
                44 => ExdDescriptors::Laps,
                45 => ExdDescriptors::Reps,
                46 => ExdDescriptors::WorkoutStep,
                47 => ExdDescriptors::CourseDistance,
                48 => ExdDescriptors::NavigationDistance,
                49 => ExdDescriptors::CourseEstimatedTimeOfArrival,
                50 => ExdDescriptors::NavigationEstimatedTimeOfArrival,
                51 => ExdDescriptors::CourseTime,
                52 => ExdDescriptors::NavigationTime,
                53 => ExdDescriptors::CourseHeading,
                54 => ExdDescriptors::NavigationHeading,
                55 => ExdDescriptors::PowerZone,
                56 => ExdDescriptors::TorqueEffectiveness,
                57 => ExdDescriptors::TimerTime,
                58 => ExdDescriptors::PowerWeightRatio,
                59 => ExdDescriptors::LeftPlatformCenterOffset,
                60 => ExdDescriptors::RightPlatformCenterOffset,
                61 => ExdDescriptors::LeftPowerPhaseStartAngle,
                62 => ExdDescriptors::RightPowerPhaseStartAngle,
                63 => ExdDescriptors::LeftPowerPhaseFinishAngle,
                64 => ExdDescriptors::RightPowerPhaseFinishAngle,
                65 => ExdDescriptors::Gears,
                66 => ExdDescriptors::Pace,
                67 => ExdDescriptors::TrainingEffect,
                68 => ExdDescriptors::VerticalOscillation,
                69 => ExdDescriptors::VerticalRatio,
                70 => ExdDescriptors::GroundContactTime,
                71 => ExdDescriptors::LeftGroundContactTimeBalance,
                72 => ExdDescriptors::RightGroundContactTimeBalance,
                73 => ExdDescriptors::StrideLength,
                74 => ExdDescriptors::RunningCadence,
                75 => ExdDescriptors::PerformanceCondition,
                76 => ExdDescriptors::CourseType,
                77 => ExdDescriptors::TimeInPowerZone,
                78 => ExdDescriptors::NavigationTurn,
                79 => ExdDescriptors::CourseLocation,
                80 => ExdDescriptors::NavigationLocation,
                81 => ExdDescriptors::Compass,
                82 => ExdDescriptors::GearCombo,
                83 => ExdDescriptors::MuscleOxygen,
                84 => ExdDescriptors::Icon,
                85 => ExdDescriptors::CompassHeading,
                86 => ExdDescriptors::GpsHeading,
                87 => ExdDescriptors::GpsElevation,
                88 => ExdDescriptors::AnaerobicTrainingEffect,
                89 => ExdDescriptors::Course,
                90 => ExdDescriptors::OffCourse,
                91 => ExdDescriptors::GlideRatio,
                92 => ExdDescriptors::VerticalDistance,
                93 => ExdDescriptors::Vmg,
                94 => ExdDescriptors::AmbientPressure,
                95 => ExdDescriptors::Pressure,
                96 => ExdDescriptors::Vam,
                n => ExdDescriptors::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert ExdDescriptors to {:?}", field);
        }
    }
}
