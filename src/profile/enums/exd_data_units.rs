use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExdDataUnits {
    Bpm,
    Bradians,
    Calories,
    Centimeter,
    Degrees,
    DegreesCelsius,
    DegreesFarenheit,
    EightCardinal,
    EnumBatteryStatus,
    EnumBikeLightBatteryStatus,
    EnumBikeLightBeamAngleMode,
    EnumBikeLightNetworkConfigType,
    EnumCoursePoint,
    EnumSport,
    EnumTurnType,
    Feet,
    FeetPerHour,
    FeetPerMin,
    Gear,
    HectoPascals,
    Hours,
    InchesHg,
    Kilofeet,
    Kilojoules,
    Kilometers,
    KilometersPerHour,
    Laps,
    Lights,
    Mbars,
    Meters,
    MetersPerHour,
    MetersPerMin,
    MetersPerSec,
    Miles,
    MilesPerHour,
    Millimeters,
    Milliseconds,
    Minutes,
    MmHg,
    NoUnits,
    Percent,
    Rpm,
    SecondPerKilometer,
    SecondPerMile,
    Seconds,
    Time,
    Watts,
    WattsPerKilogram,
    Yards,
    Zone,
    UnknownValue(u64),
}

impl From<FieldContent> for ExdDataUnits {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => ExdDataUnits::NoUnits,
                1 => ExdDataUnits::Laps,
                2 => ExdDataUnits::MilesPerHour,
                3 => ExdDataUnits::KilometersPerHour,
                4 => ExdDataUnits::FeetPerHour,
                5 => ExdDataUnits::MetersPerHour,
                6 => ExdDataUnits::DegreesCelsius,
                7 => ExdDataUnits::DegreesFarenheit,
                8 => ExdDataUnits::Zone,
                9 => ExdDataUnits::Gear,
                10 => ExdDataUnits::Rpm,
                11 => ExdDataUnits::Bpm,
                12 => ExdDataUnits::Degrees,
                13 => ExdDataUnits::Millimeters,
                14 => ExdDataUnits::Meters,
                15 => ExdDataUnits::Kilometers,
                16 => ExdDataUnits::Feet,
                17 => ExdDataUnits::Yards,
                18 => ExdDataUnits::Kilofeet,
                19 => ExdDataUnits::Miles,
                20 => ExdDataUnits::Time,
                21 => ExdDataUnits::EnumTurnType,
                22 => ExdDataUnits::Percent,
                23 => ExdDataUnits::Watts,
                24 => ExdDataUnits::WattsPerKilogram,
                25 => ExdDataUnits::EnumBatteryStatus,
                26 => ExdDataUnits::EnumBikeLightBeamAngleMode,
                27 => ExdDataUnits::EnumBikeLightBatteryStatus,
                28 => ExdDataUnits::EnumBikeLightNetworkConfigType,
                29 => ExdDataUnits::Lights,
                30 => ExdDataUnits::Seconds,
                31 => ExdDataUnits::Minutes,
                32 => ExdDataUnits::Hours,
                33 => ExdDataUnits::Calories,
                34 => ExdDataUnits::Kilojoules,
                35 => ExdDataUnits::Milliseconds,
                36 => ExdDataUnits::SecondPerMile,
                37 => ExdDataUnits::SecondPerKilometer,
                38 => ExdDataUnits::Centimeter,
                39 => ExdDataUnits::EnumCoursePoint,
                40 => ExdDataUnits::Bradians,
                41 => ExdDataUnits::EnumSport,
                42 => ExdDataUnits::InchesHg,
                43 => ExdDataUnits::MmHg,
                44 => ExdDataUnits::Mbars,
                45 => ExdDataUnits::HectoPascals,
                46 => ExdDataUnits::FeetPerMin,
                47 => ExdDataUnits::MetersPerMin,
                48 => ExdDataUnits::MetersPerSec,
                49 => ExdDataUnits::EightCardinal,
                n => ExdDataUnits::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert ExdDataUnits to {:?}", field);
        }
    }
}