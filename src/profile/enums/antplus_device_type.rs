use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AntplusDeviceType {
    Antfs,
    BikeAero,
    BikeCadence,
    BikeLightMain,
    BikeLightShared,
    BikePower,
    BikeRadar,
    BikeSpeed,
    BikeSpeedCadence,
    BloodPressure,
    Control,
    ControlHub,
    EnvSensor,
    EnvironmentSensorLegacy,
    Exd,
    FitnessEquipment,
    GeocacheNode,
    HeartRate,
    LightElectricVehicle,
    MultiSportSpeedDistance,
    MuscleOxygen,
    Racquet,
    StrideSpeedDistance,
    WeightScale,
    UnknownValue(u64),
}

impl From<FieldContent> for AntplusDeviceType {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt8(enum_value) = field {
            match enum_value {
                1 => AntplusDeviceType::Antfs,
                11 => AntplusDeviceType::BikePower,
                12 => AntplusDeviceType::EnvironmentSensorLegacy,
                15 => AntplusDeviceType::MultiSportSpeedDistance,
                16 => AntplusDeviceType::Control,
                17 => AntplusDeviceType::FitnessEquipment,
                18 => AntplusDeviceType::BloodPressure,
                19 => AntplusDeviceType::GeocacheNode,
                20 => AntplusDeviceType::LightElectricVehicle,
                25 => AntplusDeviceType::EnvSensor,
                26 => AntplusDeviceType::Racquet,
                27 => AntplusDeviceType::ControlHub,
                31 => AntplusDeviceType::MuscleOxygen,
                35 => AntplusDeviceType::BikeLightMain,
                36 => AntplusDeviceType::BikeLightShared,
                38 => AntplusDeviceType::Exd,
                40 => AntplusDeviceType::BikeRadar,
                46 => AntplusDeviceType::BikeAero,
                119 => AntplusDeviceType::WeightScale,
                120 => AntplusDeviceType::HeartRate,
                121 => AntplusDeviceType::BikeSpeedCadence,
                122 => AntplusDeviceType::BikeCadence,
                123 => AntplusDeviceType::BikeSpeed,
                124 => AntplusDeviceType::StrideSpeedDistance,
                n => AntplusDeviceType::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert AntplusDeviceType to {:?}", field);
        }
    }
}
