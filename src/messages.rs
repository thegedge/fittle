// DO NOT EDIT -- generated code

use crate::enums;

use crate::fields::Field;

#[derive(Debug, Default)]
pub struct FileId {
    type_: Option<enums::File>,
    manufacturer: Option<enums::Manufacturer>,
    product: Option<u16>,
    serial_number: Option<u32>,
    time_created: Option<enums::DateTime>,
    number: Option<u16>,
    product_name: Option<String>,
}

impl From<Vec<(u8, Field)>> for FileId {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.type_ = field.one().map(<enums::File>::from),
                1 => msg.manufacturer = field.one().map(<enums::Manufacturer>::from),
                2 => msg.product = field.one().map(<u16>::from),
                3 => msg.serial_number = field.one().map(<u32>::from),
                4 => msg.time_created = field.one().map(<enums::DateTime>::from),
                5 => msg.number = field.one().map(<u16>::from),
                8 => msg.product_name = field.one().map(<String>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct FileCreator {
    software_version: Option<u16>,
    hardware_version: Option<u8>,
}

impl From<Vec<(u8, Field)>> for FileCreator {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.software_version = field.one().map(<u16>::from),
                1 => msg.hardware_version = field.one().map(<u8>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct TimestampCorrelation {
    timestamp: Option<enums::DateTime>,
    fractional_timestamp: Option<u16>,
    system_timestamp: Option<enums::DateTime>,
    fractional_system_timestamp: Option<u16>,
    local_timestamp: Option<enums::LocalDateTime>,
    timestamp_ms: Option<u16>,
    system_timestamp_ms: Option<u16>,
}

impl From<Vec<(u8, Field)>> for TimestampCorrelation {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.fractional_timestamp = field.one().map(<u16>::from),
                1 => msg.system_timestamp = field.one().map(<enums::DateTime>::from),
                2 => msg.fractional_system_timestamp = field.one().map(<u16>::from),
                3 => msg.local_timestamp = field.one().map(<enums::LocalDateTime>::from),
                4 => msg.timestamp_ms = field.one().map(<u16>::from),
                5 => msg.system_timestamp_ms = field.one().map(<u16>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct Software {
    message_index: Option<enums::MessageIndex>,
    version: Option<u16>,
    part_number: Option<String>,
}

impl From<Vec<(u8, Field)>> for Software {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                3 => msg.version = field.one().map(<u16>::from),
                5 => msg.part_number = field.one().map(<String>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct SlaveDevice {
    manufacturer: Option<enums::Manufacturer>,
    product: Option<u16>,
}

impl From<Vec<(u8, Field)>> for SlaveDevice {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.manufacturer = field.one().map(<enums::Manufacturer>::from),
                1 => msg.product = field.one().map(<u16>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct Capabilities {
    workouts_supported: Option<enums::WorkoutCapabilities>,
    connectivity_supported: Option<enums::ConnectivityCapabilities>,
}

impl From<Vec<(u8, Field)>> for Capabilities {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                21 => msg.workouts_supported = field.one().map(<enums::WorkoutCapabilities>::from),
                23 => msg.connectivity_supported = field.one().map(<enums::ConnectivityCapabilities>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct FileCapabilities {
    message_index: Option<enums::MessageIndex>,
    type_: Option<enums::File>,
    flags: Option<enums::FileFlags>,
    directory: Option<String>,
    max_count: Option<u16>,
    max_size: Option<u32>,
}

impl From<Vec<(u8, Field)>> for FileCapabilities {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.type_ = field.one().map(<enums::File>::from),
                1 => msg.flags = field.one().map(<enums::FileFlags>::from),
                2 => msg.directory = field.one().map(<String>::from),
                3 => msg.max_count = field.one().map(<u16>::from),
                4 => msg.max_size = field.one().map(<u32>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct MesgCapabilities {
    message_index: Option<enums::MessageIndex>,
    file: Option<enums::File>,
    mesg_num: Option<enums::MesgNum>,
    count_type: Option<enums::MesgCount>,
    count: Option<u16>,
}

impl From<Vec<(u8, Field)>> for MesgCapabilities {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.file = field.one().map(<enums::File>::from),
                1 => msg.mesg_num = field.one().map(<enums::MesgNum>::from),
                2 => msg.count_type = field.one().map(<enums::MesgCount>::from),
                3 => msg.count = field.one().map(<u16>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct FieldCapabilities {
    message_index: Option<enums::MessageIndex>,
    file: Option<enums::File>,
    mesg_num: Option<enums::MesgNum>,
    field_num: Option<u8>,
    count: Option<u16>,
}

impl From<Vec<(u8, Field)>> for FieldCapabilities {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.file = field.one().map(<enums::File>::from),
                1 => msg.mesg_num = field.one().map(<enums::MesgNum>::from),
                2 => msg.field_num = field.one().map(<u8>::from),
                3 => msg.count = field.one().map(<u16>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct DeviceSettings {
    active_time_zone: Option<u8>,
    utc_offset: Option<u32>,
    time_offset: Option<Vec<u32>>,
    time_mode: Option<Vec<enums::TimeMode>>,
    time_zone_offset: Option<Vec<i8>>,
    backlight_mode: Option<enums::BacklightMode>,
    activity_tracker_enabled: Option<bool>,
    clock_time: Option<enums::DateTime>,
    pages_enabled: Option<Vec<u16>>,
    move_alert_enabled: Option<bool>,
    date_mode: Option<enums::DateMode>,
    display_orientation: Option<enums::DisplayOrientation>,
    mounting_side: Option<enums::Side>,
    default_page: Option<Vec<u16>>,
    autosync_min_steps: Option<u16>,
    autosync_min_time: Option<u16>,
    lactate_threshold_autodetect_enabled: Option<bool>,
    ble_auto_upload_enabled: Option<bool>,
    auto_sync_frequency: Option<enums::AutoSyncFrequency>,
    auto_activity_detect: Option<enums::AutoActivityDetect>,
    number_of_screens: Option<u8>,
    smart_notification_display_orientation: Option<enums::DisplayOrientation>,
    tap_interface: Option<enums::Switch>,
}

impl From<Vec<(u8, Field)>> for DeviceSettings {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.active_time_zone = field.one().map(<u8>::from),
                1 => msg.utc_offset = field.one().map(<u32>::from),
                2 => msg.time_offset = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                4 => msg.time_mode = field.many().map(|vec| vec.into_iter().map(<enums::TimeMode>::from).collect()),
                5 => msg.time_zone_offset = field.many().map(|vec| vec.into_iter().map(<i8>::from).collect()),
                12 => msg.backlight_mode = field.one().map(<enums::BacklightMode>::from),
                36 => msg.activity_tracker_enabled = field.one().map(<bool>::from),
                39 => msg.clock_time = field.one().map(<enums::DateTime>::from),
                40 => msg.pages_enabled = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                46 => msg.move_alert_enabled = field.one().map(<bool>::from),
                47 => msg.date_mode = field.one().map(<enums::DateMode>::from),
                55 => msg.display_orientation = field.one().map(<enums::DisplayOrientation>::from),
                56 => msg.mounting_side = field.one().map(<enums::Side>::from),
                57 => msg.default_page = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                58 => msg.autosync_min_steps = field.one().map(<u16>::from),
                59 => msg.autosync_min_time = field.one().map(<u16>::from),
                80 => msg.lactate_threshold_autodetect_enabled = field.one().map(<bool>::from),
                86 => msg.ble_auto_upload_enabled = field.one().map(<bool>::from),
                89 => msg.auto_sync_frequency = field.one().map(<enums::AutoSyncFrequency>::from),
                90 => msg.auto_activity_detect = field.one().map(<enums::AutoActivityDetect>::from),
                94 => msg.number_of_screens = field.one().map(<u8>::from),
                95 => msg.smart_notification_display_orientation = field.one().map(<enums::DisplayOrientation>::from),
                134 => msg.tap_interface = field.one().map(<enums::Switch>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct HrmProfile {
    message_index: Option<enums::MessageIndex>,
    enabled: Option<bool>,
    hrm_ant_id: Option<u16>,
    log_hrv: Option<bool>,
    hrm_ant_id_trans_type: Option<u8>,
}

impl From<Vec<(u8, Field)>> for HrmProfile {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.enabled = field.one().map(<bool>::from),
                1 => msg.hrm_ant_id = field.one().map(<u16>::from),
                2 => msg.log_hrv = field.one().map(<bool>::from),
                3 => msg.hrm_ant_id_trans_type = field.one().map(<u8>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct SdmProfile {
    message_index: Option<enums::MessageIndex>,
    enabled: Option<bool>,
    sdm_ant_id: Option<u16>,
    sdm_cal_factor: Option<u16>,
    odometer: Option<u32>,
    speed_source: Option<bool>,
    sdm_ant_id_trans_type: Option<u8>,
    odometer_rollover: Option<u8>,
}

impl From<Vec<(u8, Field)>> for SdmProfile {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.enabled = field.one().map(<bool>::from),
                1 => msg.sdm_ant_id = field.one().map(<u16>::from),
                2 => msg.sdm_cal_factor = field.one().map(<u16>::from),
                3 => msg.odometer = field.one().map(<u32>::from),
                4 => msg.speed_source = field.one().map(<bool>::from),
                5 => msg.sdm_ant_id_trans_type = field.one().map(<u8>::from),
                7 => msg.odometer_rollover = field.one().map(<u8>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct BikeProfile {
    message_index: Option<enums::MessageIndex>,
    name: Option<String>,
    sport: Option<enums::Sport>,
    sub_sport: Option<enums::SubSport>,
    odometer: Option<u32>,
    bike_spd_ant_id: Option<u16>,
    bike_cad_ant_id: Option<u16>,
    bike_spdcad_ant_id: Option<u16>,
    bike_power_ant_id: Option<u16>,
    custom_wheelsize: Option<u16>,
    auto_wheelsize: Option<u16>,
    bike_weight: Option<u16>,
    power_cal_factor: Option<u16>,
    auto_wheel_cal: Option<bool>,
    auto_power_zero: Option<bool>,
    id: Option<u8>,
    spd_enabled: Option<bool>,
    cad_enabled: Option<bool>,
    spdcad_enabled: Option<bool>,
    power_enabled: Option<bool>,
    crank_length: Option<u8>,
    enabled: Option<bool>,
    bike_spd_ant_id_trans_type: Option<u8>,
    bike_cad_ant_id_trans_type: Option<u8>,
    bike_spdcad_ant_id_trans_type: Option<u8>,
    bike_power_ant_id_trans_type: Option<u8>,
    odometer_rollover: Option<u8>,
    front_gear_num: Option<u8>,
    front_gear: Option<Vec<u8>>,
    rear_gear_num: Option<u8>,
    rear_gear: Option<Vec<u8>>,
    shimano_di2_enabled: Option<bool>,
}

impl From<Vec<(u8, Field)>> for BikeProfile {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.name = field.one().map(<String>::from),
                1 => msg.sport = field.one().map(<enums::Sport>::from),
                2 => msg.sub_sport = field.one().map(<enums::SubSport>::from),
                3 => msg.odometer = field.one().map(<u32>::from),
                4 => msg.bike_spd_ant_id = field.one().map(<u16>::from),
                5 => msg.bike_cad_ant_id = field.one().map(<u16>::from),
                6 => msg.bike_spdcad_ant_id = field.one().map(<u16>::from),
                7 => msg.bike_power_ant_id = field.one().map(<u16>::from),
                8 => msg.custom_wheelsize = field.one().map(<u16>::from),
                9 => msg.auto_wheelsize = field.one().map(<u16>::from),
                10 => msg.bike_weight = field.one().map(<u16>::from),
                11 => msg.power_cal_factor = field.one().map(<u16>::from),
                12 => msg.auto_wheel_cal = field.one().map(<bool>::from),
                13 => msg.auto_power_zero = field.one().map(<bool>::from),
                14 => msg.id = field.one().map(<u8>::from),
                15 => msg.spd_enabled = field.one().map(<bool>::from),
                16 => msg.cad_enabled = field.one().map(<bool>::from),
                17 => msg.spdcad_enabled = field.one().map(<bool>::from),
                18 => msg.power_enabled = field.one().map(<bool>::from),
                19 => msg.crank_length = field.one().map(<u8>::from),
                20 => msg.enabled = field.one().map(<bool>::from),
                21 => msg.bike_spd_ant_id_trans_type = field.one().map(<u8>::from),
                22 => msg.bike_cad_ant_id_trans_type = field.one().map(<u8>::from),
                23 => msg.bike_spdcad_ant_id_trans_type = field.one().map(<u8>::from),
                24 => msg.bike_power_ant_id_trans_type = field.one().map(<u8>::from),
                37 => msg.odometer_rollover = field.one().map(<u8>::from),
                38 => msg.front_gear_num = field.one().map(<u8>::from),
                39 => msg.front_gear = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                40 => msg.rear_gear_num = field.one().map(<u8>::from),
                41 => msg.rear_gear = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                44 => msg.shimano_di2_enabled = field.one().map(<bool>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct Connectivity {
    bluetooth_enabled: Option<bool>,
    bluetooth_le_enabled: Option<bool>,
    ant_enabled: Option<bool>,
    name: Option<String>,
    live_tracking_enabled: Option<bool>,
    weather_conditions_enabled: Option<bool>,
    weather_alerts_enabled: Option<bool>,
    auto_activity_upload_enabled: Option<bool>,
    course_download_enabled: Option<bool>,
    workout_download_enabled: Option<bool>,
    gps_ephemeris_download_enabled: Option<bool>,
    incident_detection_enabled: Option<bool>,
    grouptrack_enabled: Option<bool>,
}

impl From<Vec<(u8, Field)>> for Connectivity {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.bluetooth_enabled = field.one().map(<bool>::from),
                1 => msg.bluetooth_le_enabled = field.one().map(<bool>::from),
                2 => msg.ant_enabled = field.one().map(<bool>::from),
                3 => msg.name = field.one().map(<String>::from),
                4 => msg.live_tracking_enabled = field.one().map(<bool>::from),
                5 => msg.weather_conditions_enabled = field.one().map(<bool>::from),
                6 => msg.weather_alerts_enabled = field.one().map(<bool>::from),
                7 => msg.auto_activity_upload_enabled = field.one().map(<bool>::from),
                8 => msg.course_download_enabled = field.one().map(<bool>::from),
                9 => msg.workout_download_enabled = field.one().map(<bool>::from),
                10 => msg.gps_ephemeris_download_enabled = field.one().map(<bool>::from),
                11 => msg.incident_detection_enabled = field.one().map(<bool>::from),
                12 => msg.grouptrack_enabled = field.one().map(<bool>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct OhrSettings {
    timestamp: Option<enums::DateTime>,
    enabled: Option<enums::Switch>,
}

impl From<Vec<(u8, Field)>> for OhrSettings {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.enabled = field.one().map(<enums::Switch>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct ZonesTarget {
    max_heart_rate: Option<u8>,
    threshold_heart_rate: Option<u8>,
    functional_threshold_power: Option<u16>,
    hr_calc_type: Option<enums::HrZoneCalc>,
    pwr_calc_type: Option<enums::PwrZoneCalc>,
}

impl From<Vec<(u8, Field)>> for ZonesTarget {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                1 => msg.max_heart_rate = field.one().map(<u8>::from),
                2 => msg.threshold_heart_rate = field.one().map(<u8>::from),
                3 => msg.functional_threshold_power = field.one().map(<u16>::from),
                5 => msg.hr_calc_type = field.one().map(<enums::HrZoneCalc>::from),
                7 => msg.pwr_calc_type = field.one().map(<enums::PwrZoneCalc>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct Sport {
    sport: Option<enums::Sport>,
    sub_sport: Option<enums::SubSport>,
    name: Option<String>,
}

impl From<Vec<(u8, Field)>> for Sport {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.sport = field.one().map(<enums::Sport>::from),
                1 => msg.sub_sport = field.one().map(<enums::SubSport>::from),
                3 => msg.name = field.one().map(<String>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct HrZone {
    message_index: Option<enums::MessageIndex>,
    high_bpm: Option<u8>,
    name: Option<String>,
}

impl From<Vec<(u8, Field)>> for HrZone {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                1 => msg.high_bpm = field.one().map(<u8>::from),
                2 => msg.name = field.one().map(<String>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct SpeedZone {
    message_index: Option<enums::MessageIndex>,
    high_value: Option<u16>,
    name: Option<String>,
}

impl From<Vec<(u8, Field)>> for SpeedZone {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.high_value = field.one().map(<u16>::from),
                1 => msg.name = field.one().map(<String>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct CadenceZone {
    message_index: Option<enums::MessageIndex>,
    high_value: Option<u8>,
    name: Option<String>,
}

impl From<Vec<(u8, Field)>> for CadenceZone {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.high_value = field.one().map(<u8>::from),
                1 => msg.name = field.one().map(<String>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct PowerZone {
    message_index: Option<enums::MessageIndex>,
    high_value: Option<u16>,
    name: Option<String>,
}

impl From<Vec<(u8, Field)>> for PowerZone {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                1 => msg.high_value = field.one().map(<u16>::from),
                2 => msg.name = field.one().map(<String>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct MetZone {
    message_index: Option<enums::MessageIndex>,
    high_bpm: Option<u8>,
    calories: Option<u16>,
    fat_calories: Option<u8>,
}

impl From<Vec<(u8, Field)>> for MetZone {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                1 => msg.high_bpm = field.one().map(<u8>::from),
                2 => msg.calories = field.one().map(<u16>::from),
                3 => msg.fat_calories = field.one().map(<u8>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct DiveSettings {
    message_index: Option<enums::MessageIndex>,
    name: Option<String>,
    model: Option<enums::TissueModelType>,
    gf_low: Option<u8>,
    gf_high: Option<u8>,
    water_type: Option<enums::WaterType>,
    water_density: Option<f32>,
    po2_warn: Option<u8>,
    po2_critical: Option<u8>,
    po2_deco: Option<u8>,
    safety_stop_enabled: Option<bool>,
    bottom_depth: Option<f32>,
    bottom_time: Option<u32>,
    apnea_countdown_enabled: Option<bool>,
    apnea_countdown_time: Option<u32>,
    backlight_mode: Option<enums::DiveBacklightMode>,
    backlight_brightness: Option<u8>,
    backlight_timeout: Option<enums::BacklightTimeout>,
    repeat_dive_interval: Option<u16>,
    safety_stop_time: Option<u16>,
    heart_rate_source_type: Option<enums::SourceType>,
    heart_rate_source: Option<u8>,
}

impl From<Vec<(u8, Field)>> for DiveSettings {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.name = field.one().map(<String>::from),
                1 => msg.model = field.one().map(<enums::TissueModelType>::from),
                2 => msg.gf_low = field.one().map(<u8>::from),
                3 => msg.gf_high = field.one().map(<u8>::from),
                4 => msg.water_type = field.one().map(<enums::WaterType>::from),
                5 => msg.water_density = field.one().map(<f32>::from),
                6 => msg.po2_warn = field.one().map(<u8>::from),
                7 => msg.po2_critical = field.one().map(<u8>::from),
                8 => msg.po2_deco = field.one().map(<u8>::from),
                9 => msg.safety_stop_enabled = field.one().map(<bool>::from),
                10 => msg.bottom_depth = field.one().map(<f32>::from),
                11 => msg.bottom_time = field.one().map(<u32>::from),
                12 => msg.apnea_countdown_enabled = field.one().map(<bool>::from),
                13 => msg.apnea_countdown_time = field.one().map(<u32>::from),
                14 => msg.backlight_mode = field.one().map(<enums::DiveBacklightMode>::from),
                15 => msg.backlight_brightness = field.one().map(<u8>::from),
                16 => msg.backlight_timeout = field.one().map(<enums::BacklightTimeout>::from),
                17 => msg.repeat_dive_interval = field.one().map(<u16>::from),
                18 => msg.safety_stop_time = field.one().map(<u16>::from),
                19 => msg.heart_rate_source_type = field.one().map(<enums::SourceType>::from),
                20 => msg.heart_rate_source = field.one().map(<u8>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct DiveGas {
    message_index: Option<enums::MessageIndex>,
    helium_content: Option<u8>,
    oxygen_content: Option<u8>,
    status: Option<enums::DiveGasStatus>,
}

impl From<Vec<(u8, Field)>> for DiveGas {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.helium_content = field.one().map(<u8>::from),
                1 => msg.oxygen_content = field.one().map(<u8>::from),
                2 => msg.status = field.one().map(<enums::DiveGasStatus>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct Goal {
    message_index: Option<enums::MessageIndex>,
    sport: Option<enums::Sport>,
    sub_sport: Option<enums::SubSport>,
    start_date: Option<enums::DateTime>,
    end_date: Option<enums::DateTime>,
    type_: Option<enums::Goal>,
    value: Option<u32>,
    repeat: Option<bool>,
    target_value: Option<u32>,
    recurrence: Option<enums::GoalRecurrence>,
    recurrence_value: Option<u16>,
    enabled: Option<bool>,
    source: Option<enums::GoalSource>,
}

impl From<Vec<(u8, Field)>> for Goal {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.sport = field.one().map(<enums::Sport>::from),
                1 => msg.sub_sport = field.one().map(<enums::SubSport>::from),
                2 => msg.start_date = field.one().map(<enums::DateTime>::from),
                3 => msg.end_date = field.one().map(<enums::DateTime>::from),
                4 => msg.type_ = field.one().map(<enums::Goal>::from),
                5 => msg.value = field.one().map(<u32>::from),
                6 => msg.repeat = field.one().map(<bool>::from),
                7 => msg.target_value = field.one().map(<u32>::from),
                8 => msg.recurrence = field.one().map(<enums::GoalRecurrence>::from),
                9 => msg.recurrence_value = field.one().map(<u16>::from),
                10 => msg.enabled = field.one().map(<bool>::from),
                11 => msg.source = field.one().map(<enums::GoalSource>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct Activity {
    timestamp: Option<enums::DateTime>,
    total_timer_time: Option<u32>,
    num_sessions: Option<u16>,
    type_: Option<enums::Activity>,
    event: Option<enums::Event>,
    event_type: Option<enums::EventType>,
    local_timestamp: Option<enums::LocalDateTime>,
    event_group: Option<u8>,
}

impl From<Vec<(u8, Field)>> for Activity {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.total_timer_time = field.one().map(<u32>::from),
                1 => msg.num_sessions = field.one().map(<u16>::from),
                2 => msg.type_ = field.one().map(<enums::Activity>::from),
                3 => msg.event = field.one().map(<enums::Event>::from),
                4 => msg.event_type = field.one().map(<enums::EventType>::from),
                5 => msg.local_timestamp = field.one().map(<enums::LocalDateTime>::from),
                6 => msg.event_group = field.one().map(<u8>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct Session {
    message_index: Option<enums::MessageIndex>,
    timestamp: Option<enums::DateTime>,
    event: Option<enums::Event>,
    event_type: Option<enums::EventType>,
    start_time: Option<enums::DateTime>,
    start_position_lat: Option<i32>,
    start_position_long: Option<i32>,
    sport: Option<enums::Sport>,
    sub_sport: Option<enums::SubSport>,
    total_elapsed_time: Option<u32>,
    total_timer_time: Option<u32>,
    total_distance: Option<u32>,
    total_cycles: Option<u32>,
    total_calories: Option<u16>,
    total_fat_calories: Option<u16>,
    avg_speed: Option<u16>,
    max_speed: Option<u16>,
    avg_heart_rate: Option<u8>,
    max_heart_rate: Option<u8>,
    avg_cadence: Option<u8>,
    max_cadence: Option<u8>,
    avg_power: Option<u16>,
    max_power: Option<u16>,
    total_ascent: Option<u16>,
    total_descent: Option<u16>,
    total_training_effect: Option<u8>,
    first_lap_index: Option<u16>,
    num_laps: Option<u16>,
    event_group: Option<u8>,
    trigger: Option<enums::SessionTrigger>,
    nec_lat: Option<i32>,
    nec_long: Option<i32>,
    swc_lat: Option<i32>,
    swc_long: Option<i32>,
    normalized_power: Option<u16>,
    training_stress_score: Option<u16>,
    intensity_factor: Option<u16>,
    left_right_balance: Option<enums::LeftRightBalance100>,
    avg_stroke_count: Option<u32>,
    avg_stroke_distance: Option<u16>,
    swim_stroke: Option<enums::SwimStroke>,
    pool_length: Option<u16>,
    threshold_power: Option<u16>,
    pool_length_unit: Option<enums::DisplayMeasure>,
    num_active_lengths: Option<u16>,
    total_work: Option<u32>,
    avg_altitude: Option<u16>,
    max_altitude: Option<u16>,
    gps_accuracy: Option<u8>,
    avg_grade: Option<i16>,
    avg_pos_grade: Option<i16>,
    avg_neg_grade: Option<i16>,
    max_pos_grade: Option<i16>,
    max_neg_grade: Option<i16>,
    avg_temperature: Option<i8>,
    max_temperature: Option<i8>,
    total_moving_time: Option<u32>,
    avg_pos_vertical_speed: Option<i16>,
    avg_neg_vertical_speed: Option<i16>,
    max_pos_vertical_speed: Option<i16>,
    max_neg_vertical_speed: Option<i16>,
    min_heart_rate: Option<u8>,
    time_in_hr_zone: Option<Vec<u32>>,
    time_in_speed_zone: Option<Vec<u32>>,
    time_in_cadence_zone: Option<Vec<u32>>,
    time_in_power_zone: Option<Vec<u32>>,
    avg_lap_time: Option<u32>,
    best_lap_index: Option<u16>,
    min_altitude: Option<u16>,
    player_score: Option<u16>,
    opponent_score: Option<u16>,
    opponent_name: Option<String>,
    stroke_count: Option<Vec<u16>>,
    zone_count: Option<Vec<u16>>,
    max_ball_speed: Option<u16>,
    avg_ball_speed: Option<u16>,
    avg_vertical_oscillation: Option<u16>,
    avg_stance_time_percent: Option<u16>,
    avg_stance_time: Option<u16>,
    avg_fractional_cadence: Option<u8>,
    max_fractional_cadence: Option<u8>,
    total_fractional_cycles: Option<u8>,
    avg_total_hemoglobin_conc: Option<Vec<u16>>,
    min_total_hemoglobin_conc: Option<Vec<u16>>,
    max_total_hemoglobin_conc: Option<Vec<u16>>,
    avg_saturated_hemoglobin_percent: Option<Vec<u16>>,
    min_saturated_hemoglobin_percent: Option<Vec<u16>>,
    max_saturated_hemoglobin_percent: Option<Vec<u16>>,
    avg_left_torque_effectiveness: Option<u8>,
    avg_right_torque_effectiveness: Option<u8>,
    avg_left_pedal_smoothness: Option<u8>,
    avg_right_pedal_smoothness: Option<u8>,
    avg_combined_pedal_smoothness: Option<u8>,
    sport_index: Option<u8>,
    time_standing: Option<u32>,
    stand_count: Option<u16>,
    avg_left_pco: Option<i8>,
    avg_right_pco: Option<i8>,
    avg_left_power_phase: Option<Vec<u8>>,
    avg_left_power_phase_peak: Option<Vec<u8>>,
    avg_right_power_phase: Option<Vec<u8>>,
    avg_right_power_phase_peak: Option<Vec<u8>>,
    avg_power_position: Option<Vec<u16>>,
    max_power_position: Option<Vec<u16>>,
    avg_cadence_position: Option<Vec<u8>>,
    max_cadence_position: Option<Vec<u8>>,
    enhanced_avg_speed: Option<u32>,
    enhanced_max_speed: Option<u32>,
    enhanced_avg_altitude: Option<u32>,
    enhanced_min_altitude: Option<u32>,
    enhanced_max_altitude: Option<u32>,
    avg_lev_motor_power: Option<u16>,
    max_lev_motor_power: Option<u16>,
    lev_battery_consumption: Option<u8>,
    avg_vertical_ratio: Option<u16>,
    avg_stance_time_balance: Option<u16>,
    avg_step_length: Option<u16>,
    total_anaerobic_training_effect: Option<u8>,
    avg_vam: Option<u16>,
}

impl From<Vec<(u8, Field)>> for Session {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.event = field.one().map(<enums::Event>::from),
                1 => msg.event_type = field.one().map(<enums::EventType>::from),
                2 => msg.start_time = field.one().map(<enums::DateTime>::from),
                3 => msg.start_position_lat = field.one().map(<i32>::from),
                4 => msg.start_position_long = field.one().map(<i32>::from),
                5 => msg.sport = field.one().map(<enums::Sport>::from),
                6 => msg.sub_sport = field.one().map(<enums::SubSport>::from),
                7 => msg.total_elapsed_time = field.one().map(<u32>::from),
                8 => msg.total_timer_time = field.one().map(<u32>::from),
                9 => msg.total_distance = field.one().map(<u32>::from),
                10 => msg.total_cycles = field.one().map(<u32>::from),
                11 => msg.total_calories = field.one().map(<u16>::from),
                13 => msg.total_fat_calories = field.one().map(<u16>::from),
                14 => msg.avg_speed = field.one().map(<u16>::from),
                15 => msg.max_speed = field.one().map(<u16>::from),
                16 => msg.avg_heart_rate = field.one().map(<u8>::from),
                17 => msg.max_heart_rate = field.one().map(<u8>::from),
                18 => msg.avg_cadence = field.one().map(<u8>::from),
                19 => msg.max_cadence = field.one().map(<u8>::from),
                20 => msg.avg_power = field.one().map(<u16>::from),
                21 => msg.max_power = field.one().map(<u16>::from),
                22 => msg.total_ascent = field.one().map(<u16>::from),
                23 => msg.total_descent = field.one().map(<u16>::from),
                24 => msg.total_training_effect = field.one().map(<u8>::from),
                25 => msg.first_lap_index = field.one().map(<u16>::from),
                26 => msg.num_laps = field.one().map(<u16>::from),
                27 => msg.event_group = field.one().map(<u8>::from),
                28 => msg.trigger = field.one().map(<enums::SessionTrigger>::from),
                29 => msg.nec_lat = field.one().map(<i32>::from),
                30 => msg.nec_long = field.one().map(<i32>::from),
                31 => msg.swc_lat = field.one().map(<i32>::from),
                32 => msg.swc_long = field.one().map(<i32>::from),
                34 => msg.normalized_power = field.one().map(<u16>::from),
                35 => msg.training_stress_score = field.one().map(<u16>::from),
                36 => msg.intensity_factor = field.one().map(<u16>::from),
                37 => msg.left_right_balance = field.one().map(<enums::LeftRightBalance100>::from),
                41 => msg.avg_stroke_count = field.one().map(<u32>::from),
                42 => msg.avg_stroke_distance = field.one().map(<u16>::from),
                43 => msg.swim_stroke = field.one().map(<enums::SwimStroke>::from),
                44 => msg.pool_length = field.one().map(<u16>::from),
                45 => msg.threshold_power = field.one().map(<u16>::from),
                46 => msg.pool_length_unit = field.one().map(<enums::DisplayMeasure>::from),
                47 => msg.num_active_lengths = field.one().map(<u16>::from),
                48 => msg.total_work = field.one().map(<u32>::from),
                49 => msg.avg_altitude = field.one().map(<u16>::from),
                50 => msg.max_altitude = field.one().map(<u16>::from),
                51 => msg.gps_accuracy = field.one().map(<u8>::from),
                52 => msg.avg_grade = field.one().map(<i16>::from),
                53 => msg.avg_pos_grade = field.one().map(<i16>::from),
                54 => msg.avg_neg_grade = field.one().map(<i16>::from),
                55 => msg.max_pos_grade = field.one().map(<i16>::from),
                56 => msg.max_neg_grade = field.one().map(<i16>::from),
                57 => msg.avg_temperature = field.one().map(<i8>::from),
                58 => msg.max_temperature = field.one().map(<i8>::from),
                59 => msg.total_moving_time = field.one().map(<u32>::from),
                60 => msg.avg_pos_vertical_speed = field.one().map(<i16>::from),
                61 => msg.avg_neg_vertical_speed = field.one().map(<i16>::from),
                62 => msg.max_pos_vertical_speed = field.one().map(<i16>::from),
                63 => msg.max_neg_vertical_speed = field.one().map(<i16>::from),
                64 => msg.min_heart_rate = field.one().map(<u8>::from),
                65 => msg.time_in_hr_zone = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                66 => msg.time_in_speed_zone = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                67 => msg.time_in_cadence_zone = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                68 => msg.time_in_power_zone = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                69 => msg.avg_lap_time = field.one().map(<u32>::from),
                70 => msg.best_lap_index = field.one().map(<u16>::from),
                71 => msg.min_altitude = field.one().map(<u16>::from),
                82 => msg.player_score = field.one().map(<u16>::from),
                83 => msg.opponent_score = field.one().map(<u16>::from),
                84 => msg.opponent_name = field.one().map(<String>::from),
                85 => msg.stroke_count = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                86 => msg.zone_count = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                87 => msg.max_ball_speed = field.one().map(<u16>::from),
                88 => msg.avg_ball_speed = field.one().map(<u16>::from),
                89 => msg.avg_vertical_oscillation = field.one().map(<u16>::from),
                90 => msg.avg_stance_time_percent = field.one().map(<u16>::from),
                91 => msg.avg_stance_time = field.one().map(<u16>::from),
                92 => msg.avg_fractional_cadence = field.one().map(<u8>::from),
                93 => msg.max_fractional_cadence = field.one().map(<u8>::from),
                94 => msg.total_fractional_cycles = field.one().map(<u8>::from),
                95 => msg.avg_total_hemoglobin_conc = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                96 => msg.min_total_hemoglobin_conc = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                97 => msg.max_total_hemoglobin_conc = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                98 => msg.avg_saturated_hemoglobin_percent = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                99 => msg.min_saturated_hemoglobin_percent = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                100 => msg.max_saturated_hemoglobin_percent = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                101 => msg.avg_left_torque_effectiveness = field.one().map(<u8>::from),
                102 => msg.avg_right_torque_effectiveness = field.one().map(<u8>::from),
                103 => msg.avg_left_pedal_smoothness = field.one().map(<u8>::from),
                104 => msg.avg_right_pedal_smoothness = field.one().map(<u8>::from),
                105 => msg.avg_combined_pedal_smoothness = field.one().map(<u8>::from),
                111 => msg.sport_index = field.one().map(<u8>::from),
                112 => msg.time_standing = field.one().map(<u32>::from),
                113 => msg.stand_count = field.one().map(<u16>::from),
                114 => msg.avg_left_pco = field.one().map(<i8>::from),
                115 => msg.avg_right_pco = field.one().map(<i8>::from),
                116 => msg.avg_left_power_phase = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                117 => msg.avg_left_power_phase_peak = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                118 => msg.avg_right_power_phase = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                119 => msg.avg_right_power_phase_peak = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                120 => msg.avg_power_position = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                121 => msg.max_power_position = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                122 => msg.avg_cadence_position = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                123 => msg.max_cadence_position = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                124 => msg.enhanced_avg_speed = field.one().map(<u32>::from),
                125 => msg.enhanced_max_speed = field.one().map(<u32>::from),
                126 => msg.enhanced_avg_altitude = field.one().map(<u32>::from),
                127 => msg.enhanced_min_altitude = field.one().map(<u32>::from),
                128 => msg.enhanced_max_altitude = field.one().map(<u32>::from),
                129 => msg.avg_lev_motor_power = field.one().map(<u16>::from),
                130 => msg.max_lev_motor_power = field.one().map(<u16>::from),
                131 => msg.lev_battery_consumption = field.one().map(<u8>::from),
                132 => msg.avg_vertical_ratio = field.one().map(<u16>::from),
                133 => msg.avg_stance_time_balance = field.one().map(<u16>::from),
                134 => msg.avg_step_length = field.one().map(<u16>::from),
                137 => msg.total_anaerobic_training_effect = field.one().map(<u8>::from),
                139 => msg.avg_vam = field.one().map(<u16>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct Length {
    message_index: Option<enums::MessageIndex>,
    timestamp: Option<enums::DateTime>,
    event: Option<enums::Event>,
    event_type: Option<enums::EventType>,
    start_time: Option<enums::DateTime>,
    total_elapsed_time: Option<u32>,
    total_timer_time: Option<u32>,
    total_strokes: Option<u16>,
    avg_speed: Option<u16>,
    swim_stroke: Option<enums::SwimStroke>,
    avg_swimming_cadence: Option<u8>,
    event_group: Option<u8>,
    total_calories: Option<u16>,
    length_type: Option<enums::LengthType>,
    player_score: Option<u16>,
    opponent_score: Option<u16>,
    stroke_count: Option<Vec<u16>>,
    zone_count: Option<Vec<u16>>,
}

impl From<Vec<(u8, Field)>> for Length {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.event = field.one().map(<enums::Event>::from),
                1 => msg.event_type = field.one().map(<enums::EventType>::from),
                2 => msg.start_time = field.one().map(<enums::DateTime>::from),
                3 => msg.total_elapsed_time = field.one().map(<u32>::from),
                4 => msg.total_timer_time = field.one().map(<u32>::from),
                5 => msg.total_strokes = field.one().map(<u16>::from),
                6 => msg.avg_speed = field.one().map(<u16>::from),
                7 => msg.swim_stroke = field.one().map(<enums::SwimStroke>::from),
                9 => msg.avg_swimming_cadence = field.one().map(<u8>::from),
                10 => msg.event_group = field.one().map(<u8>::from),
                11 => msg.total_calories = field.one().map(<u16>::from),
                12 => msg.length_type = field.one().map(<enums::LengthType>::from),
                18 => msg.player_score = field.one().map(<u16>::from),
                19 => msg.opponent_score = field.one().map(<u16>::from),
                20 => msg.stroke_count = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                21 => msg.zone_count = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct Record {
    timestamp: Option<enums::DateTime>,
    position_lat: Option<i32>,
    position_long: Option<i32>,
    altitude: Option<u16>,
    heart_rate: Option<u8>,
    cadence: Option<u8>,
    distance: Option<u32>,
    speed: Option<u16>,
    power: Option<u16>,
    compressed_speed_distance: Option<Vec<u8>>,
    grade: Option<i16>,
    resistance: Option<u8>,
    time_from_course: Option<i32>,
    cycle_length: Option<u8>,
    temperature: Option<i8>,
    speed_1s: Option<Vec<u8>>,
    cycles: Option<u8>,
    total_cycles: Option<u32>,
    compressed_accumulated_power: Option<u16>,
    accumulated_power: Option<u32>,
    left_right_balance: Option<enums::LeftRightBalance>,
    gps_accuracy: Option<u8>,
    vertical_speed: Option<i16>,
    calories: Option<u16>,
    vertical_oscillation: Option<u16>,
    stance_time_percent: Option<u16>,
    stance_time: Option<u16>,
    activity_type: Option<enums::ActivityType>,
    left_torque_effectiveness: Option<u8>,
    right_torque_effectiveness: Option<u8>,
    left_pedal_smoothness: Option<u8>,
    right_pedal_smoothness: Option<u8>,
    combined_pedal_smoothness: Option<u8>,
    time128: Option<u8>,
    stroke_type: Option<enums::StrokeType>,
    zone: Option<u8>,
    ball_speed: Option<u16>,
    cadence256: Option<u16>,
    fractional_cadence: Option<u8>,
    total_hemoglobin_conc: Option<u16>,
    total_hemoglobin_conc_min: Option<u16>,
    total_hemoglobin_conc_max: Option<u16>,
    saturated_hemoglobin_percent: Option<u16>,
    saturated_hemoglobin_percent_min: Option<u16>,
    saturated_hemoglobin_percent_max: Option<u16>,
    device_index: Option<enums::DeviceIndex>,
    left_pco: Option<i8>,
    right_pco: Option<i8>,
    left_power_phase: Option<Vec<u8>>,
    left_power_phase_peak: Option<Vec<u8>>,
    right_power_phase: Option<Vec<u8>>,
    right_power_phase_peak: Option<Vec<u8>>,
    enhanced_speed: Option<u32>,
    enhanced_altitude: Option<u32>,
    battery_soc: Option<u8>,
    motor_power: Option<u16>,
    vertical_ratio: Option<u16>,
    stance_time_balance: Option<u16>,
    step_length: Option<u16>,
    absolute_pressure: Option<u32>,
    depth: Option<u32>,
    next_stop_depth: Option<u32>,
    next_stop_time: Option<u32>,
    time_to_surface: Option<u32>,
    ndl_time: Option<u32>,
    cns_load: Option<u8>,
    n2_load: Option<u16>,
}

impl From<Vec<(u8, Field)>> for Record {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.position_lat = field.one().map(<i32>::from),
                1 => msg.position_long = field.one().map(<i32>::from),
                2 => msg.altitude = field.one().map(<u16>::from),
                3 => msg.heart_rate = field.one().map(<u8>::from),
                4 => msg.cadence = field.one().map(<u8>::from),
                5 => msg.distance = field.one().map(<u32>::from),
                6 => msg.speed = field.one().map(<u16>::from),
                7 => msg.power = field.one().map(<u16>::from),
                8 => msg.compressed_speed_distance = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                9 => msg.grade = field.one().map(<i16>::from),
                10 => msg.resistance = field.one().map(<u8>::from),
                11 => msg.time_from_course = field.one().map(<i32>::from),
                12 => msg.cycle_length = field.one().map(<u8>::from),
                13 => msg.temperature = field.one().map(<i8>::from),
                17 => msg.speed_1s = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                18 => msg.cycles = field.one().map(<u8>::from),
                19 => msg.total_cycles = field.one().map(<u32>::from),
                28 => msg.compressed_accumulated_power = field.one().map(<u16>::from),
                29 => msg.accumulated_power = field.one().map(<u32>::from),
                30 => msg.left_right_balance = field.one().map(<enums::LeftRightBalance>::from),
                31 => msg.gps_accuracy = field.one().map(<u8>::from),
                32 => msg.vertical_speed = field.one().map(<i16>::from),
                33 => msg.calories = field.one().map(<u16>::from),
                39 => msg.vertical_oscillation = field.one().map(<u16>::from),
                40 => msg.stance_time_percent = field.one().map(<u16>::from),
                41 => msg.stance_time = field.one().map(<u16>::from),
                42 => msg.activity_type = field.one().map(<enums::ActivityType>::from),
                43 => msg.left_torque_effectiveness = field.one().map(<u8>::from),
                44 => msg.right_torque_effectiveness = field.one().map(<u8>::from),
                45 => msg.left_pedal_smoothness = field.one().map(<u8>::from),
                46 => msg.right_pedal_smoothness = field.one().map(<u8>::from),
                47 => msg.combined_pedal_smoothness = field.one().map(<u8>::from),
                48 => msg.time128 = field.one().map(<u8>::from),
                49 => msg.stroke_type = field.one().map(<enums::StrokeType>::from),
                50 => msg.zone = field.one().map(<u8>::from),
                51 => msg.ball_speed = field.one().map(<u16>::from),
                52 => msg.cadence256 = field.one().map(<u16>::from),
                53 => msg.fractional_cadence = field.one().map(<u8>::from),
                54 => msg.total_hemoglobin_conc = field.one().map(<u16>::from),
                55 => msg.total_hemoglobin_conc_min = field.one().map(<u16>::from),
                56 => msg.total_hemoglobin_conc_max = field.one().map(<u16>::from),
                57 => msg.saturated_hemoglobin_percent = field.one().map(<u16>::from),
                58 => msg.saturated_hemoglobin_percent_min = field.one().map(<u16>::from),
                59 => msg.saturated_hemoglobin_percent_max = field.one().map(<u16>::from),
                62 => msg.device_index = field.one().map(<enums::DeviceIndex>::from),
                67 => msg.left_pco = field.one().map(<i8>::from),
                68 => msg.right_pco = field.one().map(<i8>::from),
                69 => msg.left_power_phase = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                70 => msg.left_power_phase_peak = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                71 => msg.right_power_phase = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                72 => msg.right_power_phase_peak = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                73 => msg.enhanced_speed = field.one().map(<u32>::from),
                78 => msg.enhanced_altitude = field.one().map(<u32>::from),
                81 => msg.battery_soc = field.one().map(<u8>::from),
                82 => msg.motor_power = field.one().map(<u16>::from),
                83 => msg.vertical_ratio = field.one().map(<u16>::from),
                84 => msg.stance_time_balance = field.one().map(<u16>::from),
                85 => msg.step_length = field.one().map(<u16>::from),
                91 => msg.absolute_pressure = field.one().map(<u32>::from),
                92 => msg.depth = field.one().map(<u32>::from),
                93 => msg.next_stop_depth = field.one().map(<u32>::from),
                94 => msg.next_stop_time = field.one().map(<u32>::from),
                95 => msg.time_to_surface = field.one().map(<u32>::from),
                96 => msg.ndl_time = field.one().map(<u32>::from),
                97 => msg.cns_load = field.one().map(<u8>::from),
                98 => msg.n2_load = field.one().map(<u16>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct Event {
    timestamp: Option<enums::DateTime>,
    event: Option<enums::Event>,
    event_type: Option<enums::EventType>,
    data16: Option<u16>,
    data: Option<u32>,
    event_group: Option<u8>,
    score: Option<u16>,
    opponent_score: Option<u16>,
    front_gear_num: Option<u8>,
    front_gear: Option<u8>,
    rear_gear_num: Option<u8>,
    rear_gear: Option<u8>,
    device_index: Option<enums::DeviceIndex>,
}

impl From<Vec<(u8, Field)>> for Event {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.event = field.one().map(<enums::Event>::from),
                1 => msg.event_type = field.one().map(<enums::EventType>::from),
                2 => msg.data16 = field.one().map(<u16>::from),
                3 => msg.data = field.one().map(<u32>::from),
                4 => msg.event_group = field.one().map(<u8>::from),
                7 => msg.score = field.one().map(<u16>::from),
                8 => msg.opponent_score = field.one().map(<u16>::from),
                9 => msg.front_gear_num = field.one().map(<u8>::from),
                10 => msg.front_gear = field.one().map(<u8>::from),
                11 => msg.rear_gear_num = field.one().map(<u8>::from),
                12 => msg.rear_gear = field.one().map(<u8>::from),
                13 => msg.device_index = field.one().map(<enums::DeviceIndex>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct TrainingFile {
    timestamp: Option<enums::DateTime>,
    type_: Option<enums::File>,
    manufacturer: Option<enums::Manufacturer>,
    product: Option<u16>,
    serial_number: Option<u32>,
    time_created: Option<enums::DateTime>,
}

impl From<Vec<(u8, Field)>> for TrainingFile {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.type_ = field.one().map(<enums::File>::from),
                1 => msg.manufacturer = field.one().map(<enums::Manufacturer>::from),
                2 => msg.product = field.one().map(<u16>::from),
                3 => msg.serial_number = field.one().map(<u32>::from),
                4 => msg.time_created = field.one().map(<enums::DateTime>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct Hrv {
    time: Option<Vec<u16>>,
}

impl From<Vec<(u8, Field)>> for Hrv {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.time = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct WeatherConditions {
    timestamp: Option<enums::DateTime>,
    weather_report: Option<enums::WeatherReport>,
    temperature: Option<i8>,
    condition: Option<enums::WeatherStatus>,
    wind_direction: Option<u16>,
    wind_speed: Option<u16>,
    precipitation_probability: Option<u8>,
    temperature_feels_like: Option<i8>,
    relative_humidity: Option<u8>,
    location: Option<String>,
    observed_at_time: Option<enums::DateTime>,
    observed_location_lat: Option<i32>,
    observed_location_long: Option<i32>,
    day_of_week: Option<enums::DayOfWeek>,
    high_temperature: Option<i8>,
    low_temperature: Option<i8>,
}

impl From<Vec<(u8, Field)>> for WeatherConditions {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.weather_report = field.one().map(<enums::WeatherReport>::from),
                1 => msg.temperature = field.one().map(<i8>::from),
                2 => msg.condition = field.one().map(<enums::WeatherStatus>::from),
                3 => msg.wind_direction = field.one().map(<u16>::from),
                4 => msg.wind_speed = field.one().map(<u16>::from),
                5 => msg.precipitation_probability = field.one().map(<u8>::from),
                6 => msg.temperature_feels_like = field.one().map(<i8>::from),
                7 => msg.relative_humidity = field.one().map(<u8>::from),
                8 => msg.location = field.one().map(<String>::from),
                9 => msg.observed_at_time = field.one().map(<enums::DateTime>::from),
                10 => msg.observed_location_lat = field.one().map(<i32>::from),
                11 => msg.observed_location_long = field.one().map(<i32>::from),
                12 => msg.day_of_week = field.one().map(<enums::DayOfWeek>::from),
                13 => msg.high_temperature = field.one().map(<i8>::from),
                14 => msg.low_temperature = field.one().map(<i8>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct WeatherAlert {
    timestamp: Option<enums::DateTime>,
    report_id: Option<String>,
    issue_time: Option<enums::DateTime>,
    expire_time: Option<enums::DateTime>,
    severity: Option<enums::WeatherSeverity>,
    type_: Option<enums::WeatherSevereType>,
}

impl From<Vec<(u8, Field)>> for WeatherAlert {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.report_id = field.one().map(<String>::from),
                1 => msg.issue_time = field.one().map(<enums::DateTime>::from),
                2 => msg.expire_time = field.one().map(<enums::DateTime>::from),
                3 => msg.severity = field.one().map(<enums::WeatherSeverity>::from),
                4 => msg.type_ = field.one().map(<enums::WeatherSevereType>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct CameraEvent {
    timestamp: Option<enums::DateTime>,
    timestamp_ms: Option<u16>,
    camera_event_type: Option<enums::CameraEventType>,
    camera_file_uuid: Option<String>,
    camera_orientation: Option<enums::CameraOrientationType>,
}

impl From<Vec<(u8, Field)>> for CameraEvent {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.timestamp_ms = field.one().map(<u16>::from),
                1 => msg.camera_event_type = field.one().map(<enums::CameraEventType>::from),
                2 => msg.camera_file_uuid = field.one().map(<String>::from),
                3 => msg.camera_orientation = field.one().map(<enums::CameraOrientationType>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct GyroscopeData {
    timestamp: Option<enums::DateTime>,
    timestamp_ms: Option<u16>,
    sample_time_offset: Option<Vec<u16>>,
    gyro_x: Option<Vec<u16>>,
    gyro_y: Option<Vec<u16>>,
    gyro_z: Option<Vec<u16>>,
    calibrated_gyro_x: Option<Vec<f32>>,
    calibrated_gyro_y: Option<Vec<f32>>,
    calibrated_gyro_z: Option<Vec<f32>>,
}

impl From<Vec<(u8, Field)>> for GyroscopeData {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.timestamp_ms = field.one().map(<u16>::from),
                1 => msg.sample_time_offset = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                2 => msg.gyro_x = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                3 => msg.gyro_y = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                4 => msg.gyro_z = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                5 => msg.calibrated_gyro_x = field.many().map(|vec| vec.into_iter().map(<f32>::from).collect()),
                6 => msg.calibrated_gyro_y = field.many().map(|vec| vec.into_iter().map(<f32>::from).collect()),
                7 => msg.calibrated_gyro_z = field.many().map(|vec| vec.into_iter().map(<f32>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct AccelerometerData {
    timestamp: Option<enums::DateTime>,
    timestamp_ms: Option<u16>,
    sample_time_offset: Option<Vec<u16>>,
    accel_x: Option<Vec<u16>>,
    accel_y: Option<Vec<u16>>,
    accel_z: Option<Vec<u16>>,
    calibrated_accel_x: Option<Vec<f32>>,
    calibrated_accel_y: Option<Vec<f32>>,
    calibrated_accel_z: Option<Vec<f32>>,
    compressed_calibrated_accel_x: Option<Vec<i16>>,
    compressed_calibrated_accel_y: Option<Vec<i16>>,
    compressed_calibrated_accel_z: Option<Vec<i16>>,
}

impl From<Vec<(u8, Field)>> for AccelerometerData {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.timestamp_ms = field.one().map(<u16>::from),
                1 => msg.sample_time_offset = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                2 => msg.accel_x = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                3 => msg.accel_y = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                4 => msg.accel_z = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                5 => msg.calibrated_accel_x = field.many().map(|vec| vec.into_iter().map(<f32>::from).collect()),
                6 => msg.calibrated_accel_y = field.many().map(|vec| vec.into_iter().map(<f32>::from).collect()),
                7 => msg.calibrated_accel_z = field.many().map(|vec| vec.into_iter().map(<f32>::from).collect()),
                8 => msg.compressed_calibrated_accel_x = field.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                9 => msg.compressed_calibrated_accel_y = field.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                10 => msg.compressed_calibrated_accel_z = field.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct MagnetometerData {
    timestamp: Option<enums::DateTime>,
    timestamp_ms: Option<u16>,
    sample_time_offset: Option<Vec<u16>>,
    mag_x: Option<Vec<u16>>,
    mag_y: Option<Vec<u16>>,
    mag_z: Option<Vec<u16>>,
    calibrated_mag_x: Option<Vec<f32>>,
    calibrated_mag_y: Option<Vec<f32>>,
    calibrated_mag_z: Option<Vec<f32>>,
}

impl From<Vec<(u8, Field)>> for MagnetometerData {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.timestamp_ms = field.one().map(<u16>::from),
                1 => msg.sample_time_offset = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                2 => msg.mag_x = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                3 => msg.mag_y = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                4 => msg.mag_z = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                5 => msg.calibrated_mag_x = field.many().map(|vec| vec.into_iter().map(<f32>::from).collect()),
                6 => msg.calibrated_mag_y = field.many().map(|vec| vec.into_iter().map(<f32>::from).collect()),
                7 => msg.calibrated_mag_z = field.many().map(|vec| vec.into_iter().map(<f32>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct BarometerData {
    timestamp: Option<enums::DateTime>,
    timestamp_ms: Option<u16>,
    sample_time_offset: Option<Vec<u16>>,
    baro_pres: Option<Vec<u32>>,
}

impl From<Vec<(u8, Field)>> for BarometerData {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.timestamp_ms = field.one().map(<u16>::from),
                1 => msg.sample_time_offset = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                2 => msg.baro_pres = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct ThreeDSensorCalibration {
    timestamp: Option<enums::DateTime>,
    sensor_type: Option<enums::SensorType>,
    calibration_factor: Option<u32>,
    calibration_divisor: Option<u32>,
    level_shift: Option<u32>,
    offset_cal: Option<Vec<i32>>,
    orientation_matrix: Option<Vec<i32>>,
}

impl From<Vec<(u8, Field)>> for ThreeDSensorCalibration {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.sensor_type = field.one().map(<enums::SensorType>::from),
                1 => msg.calibration_factor = field.one().map(<u32>::from),
                2 => msg.calibration_divisor = field.one().map(<u32>::from),
                3 => msg.level_shift = field.one().map(<u32>::from),
                4 => msg.offset_cal = field.many().map(|vec| vec.into_iter().map(<i32>::from).collect()),
                5 => msg.orientation_matrix = field.many().map(|vec| vec.into_iter().map(<i32>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct OneDSensorCalibration {
    timestamp: Option<enums::DateTime>,
    sensor_type: Option<enums::SensorType>,
    calibration_factor: Option<u32>,
    calibration_divisor: Option<u32>,
    level_shift: Option<u32>,
    offset_cal: Option<i32>,
}

impl From<Vec<(u8, Field)>> for OneDSensorCalibration {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.sensor_type = field.one().map(<enums::SensorType>::from),
                1 => msg.calibration_factor = field.one().map(<u32>::from),
                2 => msg.calibration_divisor = field.one().map(<u32>::from),
                3 => msg.level_shift = field.one().map(<u32>::from),
                4 => msg.offset_cal = field.one().map(<i32>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct VideoFrame {
    timestamp: Option<enums::DateTime>,
    timestamp_ms: Option<u16>,
    frame_number: Option<u32>,
}

impl From<Vec<(u8, Field)>> for VideoFrame {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.timestamp_ms = field.one().map(<u16>::from),
                1 => msg.frame_number = field.one().map(<u32>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct ObdiiData {
    timestamp: Option<enums::DateTime>,
    timestamp_ms: Option<u16>,
    time_offset: Option<Vec<u16>>,
    pid: Option<u8>,
    raw_data: Option<Vec<u8>>,
    pid_data_size: Option<Vec<u8>>,
    system_time: Option<Vec<u32>>,
    start_timestamp: Option<enums::DateTime>,
    start_timestamp_ms: Option<u16>,
}

impl From<Vec<(u8, Field)>> for ObdiiData {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.timestamp_ms = field.one().map(<u16>::from),
                1 => msg.time_offset = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                2 => msg.pid = field.one().map(<u8>::from),
                3 => msg.raw_data = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                4 => msg.pid_data_size = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                5 => msg.system_time = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                6 => msg.start_timestamp = field.one().map(<enums::DateTime>::from),
                7 => msg.start_timestamp_ms = field.one().map(<u16>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct NmeaSentence {
    timestamp: Option<enums::DateTime>,
    timestamp_ms: Option<u16>,
    sentence: Option<String>,
}

impl From<Vec<(u8, Field)>> for NmeaSentence {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.timestamp_ms = field.one().map(<u16>::from),
                1 => msg.sentence = field.one().map(<String>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct AviationAttitude {
    timestamp: Option<enums::DateTime>,
    timestamp_ms: Option<u16>,
    system_time: Option<Vec<u32>>,
    pitch: Option<Vec<i16>>,
    roll: Option<Vec<i16>>,
    accel_lateral: Option<Vec<i16>>,
    accel_normal: Option<Vec<i16>>,
    turn_rate: Option<Vec<i16>>,
    stage: Option<Vec<enums::AttitudeStage>>,
    attitude_stage_complete: Option<Vec<u8>>,
    track: Option<Vec<u16>>,
    validity: Option<Vec<enums::AttitudeValidity>>,
}

impl From<Vec<(u8, Field)>> for AviationAttitude {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.timestamp_ms = field.one().map(<u16>::from),
                1 => msg.system_time = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                2 => msg.pitch = field.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                3 => msg.roll = field.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                4 => msg.accel_lateral = field.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                5 => msg.accel_normal = field.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                6 => msg.turn_rate = field.many().map(|vec| vec.into_iter().map(<i16>::from).collect()),
                7 => msg.stage = field.many().map(|vec| vec.into_iter().map(<enums::AttitudeStage>::from).collect()),
                8 => msg.attitude_stage_complete = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                9 => msg.track = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                10 => msg.validity = field.many().map(|vec| vec.into_iter().map(<enums::AttitudeValidity>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct Video {
    url: Option<String>,
    hosting_provider: Option<String>,
    duration: Option<u32>,
}

impl From<Vec<(u8, Field)>> for Video {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.url = field.one().map(<String>::from),
                1 => msg.hosting_provider = field.one().map(<String>::from),
                2 => msg.duration = field.one().map(<u32>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct VideoTitle {
    message_index: Option<enums::MessageIndex>,
    message_count: Option<u16>,
    text: Option<String>,
}

impl From<Vec<(u8, Field)>> for VideoTitle {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.message_count = field.one().map(<u16>::from),
                1 => msg.text = field.one().map(<String>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct VideoDescription {
    message_index: Option<enums::MessageIndex>,
    message_count: Option<u16>,
    text: Option<String>,
}

impl From<Vec<(u8, Field)>> for VideoDescription {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.message_count = field.one().map(<u16>::from),
                1 => msg.text = field.one().map(<String>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct VideoClip {
    clip_number: Option<u16>,
    start_timestamp: Option<enums::DateTime>,
    start_timestamp_ms: Option<u16>,
    end_timestamp: Option<enums::DateTime>,
    end_timestamp_ms: Option<u16>,
    clip_start: Option<u32>,
    clip_end: Option<u32>,
}

impl From<Vec<(u8, Field)>> for VideoClip {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.clip_number = field.one().map(<u16>::from),
                1 => msg.start_timestamp = field.one().map(<enums::DateTime>::from),
                2 => msg.start_timestamp_ms = field.one().map(<u16>::from),
                3 => msg.end_timestamp = field.one().map(<enums::DateTime>::from),
                4 => msg.end_timestamp_ms = field.one().map(<u16>::from),
                6 => msg.clip_start = field.one().map(<u32>::from),
                7 => msg.clip_end = field.one().map(<u32>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct Set {
    timestamp: Option<enums::DateTime>,
    duration: Option<u32>,
    repetitions: Option<u16>,
    weight: Option<u16>,
    set_type: Option<enums::SetType>,
    start_time: Option<enums::DateTime>,
    category: Option<Vec<enums::ExerciseCategory>>,
    category_subtype: Option<Vec<u16>>,
    weight_display_unit: Option<enums::FitBaseUnit>,
    message_index: Option<enums::MessageIndex>,
    wkt_step_index: Option<enums::MessageIndex>,
}

impl From<Vec<(u8, Field)>> for Set {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.duration = field.one().map(<u32>::from),
                3 => msg.repetitions = field.one().map(<u16>::from),
                4 => msg.weight = field.one().map(<u16>::from),
                5 => msg.set_type = field.one().map(<enums::SetType>::from),
                6 => msg.start_time = field.one().map(<enums::DateTime>::from),
                7 => msg.category = field.many().map(|vec| vec.into_iter().map(<enums::ExerciseCategory>::from).collect()),
                8 => msg.category_subtype = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                9 => msg.weight_display_unit = field.one().map(<enums::FitBaseUnit>::from),
                10 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                11 => msg.wkt_step_index = field.one().map(<enums::MessageIndex>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct Course {
    sport: Option<enums::Sport>,
    name: Option<String>,
    capabilities: Option<enums::CourseCapabilities>,
    sub_sport: Option<enums::SubSport>,
}

impl From<Vec<(u8, Field)>> for Course {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                4 => msg.sport = field.one().map(<enums::Sport>::from),
                5 => msg.name = field.one().map(<String>::from),
                6 => msg.capabilities = field.one().map(<enums::CourseCapabilities>::from),
                7 => msg.sub_sport = field.one().map(<enums::SubSport>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct CoursePoint {
    message_index: Option<enums::MessageIndex>,
    timestamp: Option<enums::DateTime>,
    position_lat: Option<i32>,
    position_long: Option<i32>,
    distance: Option<u32>,
    type_: Option<enums::CoursePoint>,
    name: Option<String>,
    favorite: Option<bool>,
}

impl From<Vec<(u8, Field)>> for CoursePoint {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                1 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                2 => msg.position_lat = field.one().map(<i32>::from),
                3 => msg.position_long = field.one().map(<i32>::from),
                4 => msg.distance = field.one().map(<u32>::from),
                5 => msg.type_ = field.one().map(<enums::CoursePoint>::from),
                6 => msg.name = field.one().map(<String>::from),
                8 => msg.favorite = field.one().map(<bool>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct SegmentId {
    name: Option<String>,
    uuid: Option<String>,
    sport: Option<enums::Sport>,
    enabled: Option<bool>,
    user_profile_primary_key: Option<u32>,
    device_id: Option<u32>,
    default_race_leader: Option<u8>,
    delete_status: Option<enums::SegmentDeleteStatus>,
    selection_type: Option<enums::SegmentSelectionType>,
}

impl From<Vec<(u8, Field)>> for SegmentId {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.name = field.one().map(<String>::from),
                1 => msg.uuid = field.one().map(<String>::from),
                2 => msg.sport = field.one().map(<enums::Sport>::from),
                3 => msg.enabled = field.one().map(<bool>::from),
                4 => msg.user_profile_primary_key = field.one().map(<u32>::from),
                5 => msg.device_id = field.one().map(<u32>::from),
                6 => msg.default_race_leader = field.one().map(<u8>::from),
                7 => msg.delete_status = field.one().map(<enums::SegmentDeleteStatus>::from),
                8 => msg.selection_type = field.one().map(<enums::SegmentSelectionType>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct SegmentLeaderboardEntry {
    message_index: Option<enums::MessageIndex>,
    name: Option<String>,
    type_: Option<enums::SegmentLeaderboardType>,
    group_primary_key: Option<u32>,
    activity_id: Option<u32>,
    segment_time: Option<u32>,
    activity_id_string: Option<String>,
}

impl From<Vec<(u8, Field)>> for SegmentLeaderboardEntry {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.name = field.one().map(<String>::from),
                1 => msg.type_ = field.one().map(<enums::SegmentLeaderboardType>::from),
                2 => msg.group_primary_key = field.one().map(<u32>::from),
                3 => msg.activity_id = field.one().map(<u32>::from),
                4 => msg.segment_time = field.one().map(<u32>::from),
                5 => msg.activity_id_string = field.one().map(<String>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct SegmentPoint {
    message_index: Option<enums::MessageIndex>,
    position_lat: Option<i32>,
    position_long: Option<i32>,
    distance: Option<u32>,
    altitude: Option<u16>,
    leader_time: Option<Vec<u32>>,
}

impl From<Vec<(u8, Field)>> for SegmentPoint {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                1 => msg.position_lat = field.one().map(<i32>::from),
                2 => msg.position_long = field.one().map(<i32>::from),
                3 => msg.distance = field.one().map(<u32>::from),
                4 => msg.altitude = field.one().map(<u16>::from),
                5 => msg.leader_time = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct SegmentLap {
    message_index: Option<enums::MessageIndex>,
    timestamp: Option<enums::DateTime>,
    event: Option<enums::Event>,
    event_type: Option<enums::EventType>,
    start_time: Option<enums::DateTime>,
    start_position_lat: Option<i32>,
    start_position_long: Option<i32>,
    end_position_lat: Option<i32>,
    end_position_long: Option<i32>,
    total_elapsed_time: Option<u32>,
    total_timer_time: Option<u32>,
    total_distance: Option<u32>,
    total_cycles: Option<u32>,
    total_calories: Option<u16>,
    total_fat_calories: Option<u16>,
    avg_speed: Option<u16>,
    max_speed: Option<u16>,
    avg_heart_rate: Option<u8>,
    max_heart_rate: Option<u8>,
    avg_cadence: Option<u8>,
    max_cadence: Option<u8>,
    avg_power: Option<u16>,
    max_power: Option<u16>,
    total_ascent: Option<u16>,
    total_descent: Option<u16>,
    sport: Option<enums::Sport>,
    event_group: Option<u8>,
    nec_lat: Option<i32>,
    nec_long: Option<i32>,
    swc_lat: Option<i32>,
    swc_long: Option<i32>,
    name: Option<String>,
    normalized_power: Option<u16>,
    left_right_balance: Option<enums::LeftRightBalance100>,
    sub_sport: Option<enums::SubSport>,
    total_work: Option<u32>,
    avg_altitude: Option<u16>,
    max_altitude: Option<u16>,
    gps_accuracy: Option<u8>,
    avg_grade: Option<i16>,
    avg_pos_grade: Option<i16>,
    avg_neg_grade: Option<i16>,
    max_pos_grade: Option<i16>,
    max_neg_grade: Option<i16>,
    avg_temperature: Option<i8>,
    max_temperature: Option<i8>,
    total_moving_time: Option<u32>,
    avg_pos_vertical_speed: Option<i16>,
    avg_neg_vertical_speed: Option<i16>,
    max_pos_vertical_speed: Option<i16>,
    max_neg_vertical_speed: Option<i16>,
    time_in_hr_zone: Option<Vec<u32>>,
    time_in_speed_zone: Option<Vec<u32>>,
    time_in_cadence_zone: Option<Vec<u32>>,
    time_in_power_zone: Option<Vec<u32>>,
    repetition_num: Option<u16>,
    min_altitude: Option<u16>,
    min_heart_rate: Option<u8>,
    active_time: Option<u32>,
    wkt_step_index: Option<enums::MessageIndex>,
    sport_event: Option<enums::SportEvent>,
    avg_left_torque_effectiveness: Option<u8>,
    avg_right_torque_effectiveness: Option<u8>,
    avg_left_pedal_smoothness: Option<u8>,
    avg_right_pedal_smoothness: Option<u8>,
    avg_combined_pedal_smoothness: Option<u8>,
    status: Option<enums::SegmentLapStatus>,
    uuid: Option<String>,
    avg_fractional_cadence: Option<u8>,
    max_fractional_cadence: Option<u8>,
    total_fractional_cycles: Option<u8>,
    front_gear_shift_count: Option<u16>,
    rear_gear_shift_count: Option<u16>,
    time_standing: Option<u32>,
    stand_count: Option<u16>,
    avg_left_pco: Option<i8>,
    avg_right_pco: Option<i8>,
    avg_left_power_phase: Option<Vec<u8>>,
    avg_left_power_phase_peak: Option<Vec<u8>>,
    avg_right_power_phase: Option<Vec<u8>>,
    avg_right_power_phase_peak: Option<Vec<u8>>,
    avg_power_position: Option<Vec<u16>>,
    max_power_position: Option<Vec<u16>>,
    avg_cadence_position: Option<Vec<u8>>,
    max_cadence_position: Option<Vec<u8>>,
    manufacturer: Option<enums::Manufacturer>,
}

impl From<Vec<(u8, Field)>> for SegmentLap {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.event = field.one().map(<enums::Event>::from),
                1 => msg.event_type = field.one().map(<enums::EventType>::from),
                2 => msg.start_time = field.one().map(<enums::DateTime>::from),
                3 => msg.start_position_lat = field.one().map(<i32>::from),
                4 => msg.start_position_long = field.one().map(<i32>::from),
                5 => msg.end_position_lat = field.one().map(<i32>::from),
                6 => msg.end_position_long = field.one().map(<i32>::from),
                7 => msg.total_elapsed_time = field.one().map(<u32>::from),
                8 => msg.total_timer_time = field.one().map(<u32>::from),
                9 => msg.total_distance = field.one().map(<u32>::from),
                10 => msg.total_cycles = field.one().map(<u32>::from),
                11 => msg.total_calories = field.one().map(<u16>::from),
                12 => msg.total_fat_calories = field.one().map(<u16>::from),
                13 => msg.avg_speed = field.one().map(<u16>::from),
                14 => msg.max_speed = field.one().map(<u16>::from),
                15 => msg.avg_heart_rate = field.one().map(<u8>::from),
                16 => msg.max_heart_rate = field.one().map(<u8>::from),
                17 => msg.avg_cadence = field.one().map(<u8>::from),
                18 => msg.max_cadence = field.one().map(<u8>::from),
                19 => msg.avg_power = field.one().map(<u16>::from),
                20 => msg.max_power = field.one().map(<u16>::from),
                21 => msg.total_ascent = field.one().map(<u16>::from),
                22 => msg.total_descent = field.one().map(<u16>::from),
                23 => msg.sport = field.one().map(<enums::Sport>::from),
                24 => msg.event_group = field.one().map(<u8>::from),
                25 => msg.nec_lat = field.one().map(<i32>::from),
                26 => msg.nec_long = field.one().map(<i32>::from),
                27 => msg.swc_lat = field.one().map(<i32>::from),
                28 => msg.swc_long = field.one().map(<i32>::from),
                29 => msg.name = field.one().map(<String>::from),
                30 => msg.normalized_power = field.one().map(<u16>::from),
                31 => msg.left_right_balance = field.one().map(<enums::LeftRightBalance100>::from),
                32 => msg.sub_sport = field.one().map(<enums::SubSport>::from),
                33 => msg.total_work = field.one().map(<u32>::from),
                34 => msg.avg_altitude = field.one().map(<u16>::from),
                35 => msg.max_altitude = field.one().map(<u16>::from),
                36 => msg.gps_accuracy = field.one().map(<u8>::from),
                37 => msg.avg_grade = field.one().map(<i16>::from),
                38 => msg.avg_pos_grade = field.one().map(<i16>::from),
                39 => msg.avg_neg_grade = field.one().map(<i16>::from),
                40 => msg.max_pos_grade = field.one().map(<i16>::from),
                41 => msg.max_neg_grade = field.one().map(<i16>::from),
                42 => msg.avg_temperature = field.one().map(<i8>::from),
                43 => msg.max_temperature = field.one().map(<i8>::from),
                44 => msg.total_moving_time = field.one().map(<u32>::from),
                45 => msg.avg_pos_vertical_speed = field.one().map(<i16>::from),
                46 => msg.avg_neg_vertical_speed = field.one().map(<i16>::from),
                47 => msg.max_pos_vertical_speed = field.one().map(<i16>::from),
                48 => msg.max_neg_vertical_speed = field.one().map(<i16>::from),
                49 => msg.time_in_hr_zone = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                50 => msg.time_in_speed_zone = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                51 => msg.time_in_cadence_zone = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                52 => msg.time_in_power_zone = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                53 => msg.repetition_num = field.one().map(<u16>::from),
                54 => msg.min_altitude = field.one().map(<u16>::from),
                55 => msg.min_heart_rate = field.one().map(<u8>::from),
                56 => msg.active_time = field.one().map(<u32>::from),
                57 => msg.wkt_step_index = field.one().map(<enums::MessageIndex>::from),
                58 => msg.sport_event = field.one().map(<enums::SportEvent>::from),
                59 => msg.avg_left_torque_effectiveness = field.one().map(<u8>::from),
                60 => msg.avg_right_torque_effectiveness = field.one().map(<u8>::from),
                61 => msg.avg_left_pedal_smoothness = field.one().map(<u8>::from),
                62 => msg.avg_right_pedal_smoothness = field.one().map(<u8>::from),
                63 => msg.avg_combined_pedal_smoothness = field.one().map(<u8>::from),
                64 => msg.status = field.one().map(<enums::SegmentLapStatus>::from),
                65 => msg.uuid = field.one().map(<String>::from),
                66 => msg.avg_fractional_cadence = field.one().map(<u8>::from),
                67 => msg.max_fractional_cadence = field.one().map(<u8>::from),
                68 => msg.total_fractional_cycles = field.one().map(<u8>::from),
                69 => msg.front_gear_shift_count = field.one().map(<u16>::from),
                70 => msg.rear_gear_shift_count = field.one().map(<u16>::from),
                71 => msg.time_standing = field.one().map(<u32>::from),
                72 => msg.stand_count = field.one().map(<u16>::from),
                73 => msg.avg_left_pco = field.one().map(<i8>::from),
                74 => msg.avg_right_pco = field.one().map(<i8>::from),
                75 => msg.avg_left_power_phase = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                76 => msg.avg_left_power_phase_peak = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                77 => msg.avg_right_power_phase = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                78 => msg.avg_right_power_phase_peak = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                79 => msg.avg_power_position = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                80 => msg.max_power_position = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                81 => msg.avg_cadence_position = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                82 => msg.max_cadence_position = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                83 => msg.manufacturer = field.one().map(<enums::Manufacturer>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct SegmentFile {
    message_index: Option<enums::MessageIndex>,
    file_uuid: Option<String>,
    enabled: Option<bool>,
    user_profile_primary_key: Option<u32>,
    leader_type: Option<Vec<enums::SegmentLeaderboardType>>,
    leader_group_primary_key: Option<Vec<u32>>,
    leader_activity_id: Option<Vec<u32>>,
    leader_activity_id_string: Option<Vec<String>>,
    default_race_leader: Option<u8>,
}

impl From<Vec<(u8, Field)>> for SegmentFile {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                1 => msg.file_uuid = field.one().map(<String>::from),
                3 => msg.enabled = field.one().map(<bool>::from),
                4 => msg.user_profile_primary_key = field.one().map(<u32>::from),
                7 => msg.leader_type = field.many().map(|vec| vec.into_iter().map(<enums::SegmentLeaderboardType>::from).collect()),
                8 => msg.leader_group_primary_key = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                9 => msg.leader_activity_id = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                10 => msg.leader_activity_id_string = field.many().map(|vec| vec.into_iter().map(<String>::from).collect()),
                11 => msg.default_race_leader = field.one().map(<u8>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct Workout {
    sport: Option<enums::Sport>,
    capabilities: Option<enums::WorkoutCapabilities>,
    num_valid_steps: Option<u16>,
    wkt_name: Option<String>,
    sub_sport: Option<enums::SubSport>,
    pool_length: Option<u16>,
    pool_length_unit: Option<enums::DisplayMeasure>,
}

impl From<Vec<(u8, Field)>> for Workout {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                4 => msg.sport = field.one().map(<enums::Sport>::from),
                5 => msg.capabilities = field.one().map(<enums::WorkoutCapabilities>::from),
                6 => msg.num_valid_steps = field.one().map(<u16>::from),
                8 => msg.wkt_name = field.one().map(<String>::from),
                11 => msg.sub_sport = field.one().map(<enums::SubSport>::from),
                14 => msg.pool_length = field.one().map(<u16>::from),
                15 => msg.pool_length_unit = field.one().map(<enums::DisplayMeasure>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct WorkoutSession {
    message_index: Option<enums::MessageIndex>,
    sport: Option<enums::Sport>,
    sub_sport: Option<enums::SubSport>,
    num_valid_steps: Option<u16>,
    first_step_index: Option<u16>,
    pool_length: Option<u16>,
    pool_length_unit: Option<enums::DisplayMeasure>,
}

impl From<Vec<(u8, Field)>> for WorkoutSession {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.sport = field.one().map(<enums::Sport>::from),
                1 => msg.sub_sport = field.one().map(<enums::SubSport>::from),
                2 => msg.num_valid_steps = field.one().map(<u16>::from),
                3 => msg.first_step_index = field.one().map(<u16>::from),
                4 => msg.pool_length = field.one().map(<u16>::from),
                5 => msg.pool_length_unit = field.one().map(<enums::DisplayMeasure>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct ExerciseTitle {
    message_index: Option<enums::MessageIndex>,
    exercise_category: Option<enums::ExerciseCategory>,
    exercise_name: Option<u16>,
    wkt_step_name: Option<Vec<String>>,
}

impl From<Vec<(u8, Field)>> for ExerciseTitle {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                0 => msg.exercise_category = field.one().map(<enums::ExerciseCategory>::from),
                1 => msg.exercise_name = field.one().map(<u16>::from),
                2 => msg.wkt_step_name = field.many().map(|vec| vec.into_iter().map(<String>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct Schedule {
    manufacturer: Option<enums::Manufacturer>,
    product: Option<u16>,
    serial_number: Option<u32>,
    time_created: Option<enums::DateTime>,
    completed: Option<bool>,
    type_: Option<enums::Schedule>,
    scheduled_time: Option<enums::LocalDateTime>,
}

impl From<Vec<(u8, Field)>> for Schedule {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.manufacturer = field.one().map(<enums::Manufacturer>::from),
                1 => msg.product = field.one().map(<u16>::from),
                2 => msg.serial_number = field.one().map(<u32>::from),
                3 => msg.time_created = field.one().map(<enums::DateTime>::from),
                4 => msg.completed = field.one().map(<bool>::from),
                5 => msg.type_ = field.one().map(<enums::Schedule>::from),
                6 => msg.scheduled_time = field.one().map(<enums::LocalDateTime>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct Totals {
    message_index: Option<enums::MessageIndex>,
    timestamp: Option<enums::DateTime>,
    timer_time: Option<u32>,
    distance: Option<u32>,
    calories: Option<u32>,
    sport: Option<enums::Sport>,
    elapsed_time: Option<u32>,
    sessions: Option<u16>,
    active_time: Option<u32>,
    sport_index: Option<u8>,
}

impl From<Vec<(u8, Field)>> for Totals {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                254 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.timer_time = field.one().map(<u32>::from),
                1 => msg.distance = field.one().map(<u32>::from),
                2 => msg.calories = field.one().map(<u32>::from),
                3 => msg.sport = field.one().map(<enums::Sport>::from),
                4 => msg.elapsed_time = field.one().map(<u32>::from),
                5 => msg.sessions = field.one().map(<u16>::from),
                6 => msg.active_time = field.one().map(<u32>::from),
                9 => msg.sport_index = field.one().map(<u8>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct WeightScale {
    timestamp: Option<enums::DateTime>,
    weight: Option<enums::Weight>,
    percent_fat: Option<u16>,
    percent_hydration: Option<u16>,
    visceral_fat_mass: Option<u16>,
    bone_mass: Option<u16>,
    muscle_mass: Option<u16>,
    basal_met: Option<u16>,
    physique_rating: Option<u8>,
    active_met: Option<u16>,
    metabolic_age: Option<u8>,
    visceral_fat_rating: Option<u8>,
    user_profile_index: Option<enums::MessageIndex>,
}

impl From<Vec<(u8, Field)>> for WeightScale {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.weight = field.one().map(<enums::Weight>::from),
                1 => msg.percent_fat = field.one().map(<u16>::from),
                2 => msg.percent_hydration = field.one().map(<u16>::from),
                3 => msg.visceral_fat_mass = field.one().map(<u16>::from),
                4 => msg.bone_mass = field.one().map(<u16>::from),
                5 => msg.muscle_mass = field.one().map(<u16>::from),
                7 => msg.basal_met = field.one().map(<u16>::from),
                8 => msg.physique_rating = field.one().map(<u8>::from),
                9 => msg.active_met = field.one().map(<u16>::from),
                10 => msg.metabolic_age = field.one().map(<u8>::from),
                11 => msg.visceral_fat_rating = field.one().map(<u8>::from),
                12 => msg.user_profile_index = field.one().map(<enums::MessageIndex>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct BloodPressure {
    timestamp: Option<enums::DateTime>,
    systolic_pressure: Option<u16>,
    diastolic_pressure: Option<u16>,
    mean_arterial_pressure: Option<u16>,
    map_3_sample_mean: Option<u16>,
    map_morning_values: Option<u16>,
    map_evening_values: Option<u16>,
    heart_rate: Option<u8>,
    heart_rate_type: Option<enums::HrType>,
    status: Option<enums::BpStatus>,
    user_profile_index: Option<enums::MessageIndex>,
}

impl From<Vec<(u8, Field)>> for BloodPressure {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.systolic_pressure = field.one().map(<u16>::from),
                1 => msg.diastolic_pressure = field.one().map(<u16>::from),
                2 => msg.mean_arterial_pressure = field.one().map(<u16>::from),
                3 => msg.map_3_sample_mean = field.one().map(<u16>::from),
                4 => msg.map_morning_values = field.one().map(<u16>::from),
                5 => msg.map_evening_values = field.one().map(<u16>::from),
                6 => msg.heart_rate = field.one().map(<u8>::from),
                7 => msg.heart_rate_type = field.one().map(<enums::HrType>::from),
                8 => msg.status = field.one().map(<enums::BpStatus>::from),
                9 => msg.user_profile_index = field.one().map(<enums::MessageIndex>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct MonitoringInfo {
    timestamp: Option<enums::DateTime>,
    local_timestamp: Option<enums::LocalDateTime>,
    activity_type: Option<Vec<enums::ActivityType>>,
    cycles_to_distance: Option<Vec<u16>>,
    cycles_to_calories: Option<Vec<u16>>,
    resting_metabolic_rate: Option<u16>,
}

impl From<Vec<(u8, Field)>> for MonitoringInfo {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.local_timestamp = field.one().map(<enums::LocalDateTime>::from),
                1 => msg.activity_type = field.many().map(|vec| vec.into_iter().map(<enums::ActivityType>::from).collect()),
                3 => msg.cycles_to_distance = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                4 => msg.cycles_to_calories = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                5 => msg.resting_metabolic_rate = field.one().map(<u16>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct Monitoring {
    timestamp: Option<enums::DateTime>,
    device_index: Option<enums::DeviceIndex>,
    calories: Option<u16>,
    distance: Option<u32>,
    cycles: Option<u32>,
    active_time: Option<u32>,
    activity_type: Option<enums::ActivityType>,
    activity_subtype: Option<enums::ActivitySubtype>,
    activity_level: Option<enums::ActivityLevel>,
    distance_16: Option<u16>,
    cycles_16: Option<u16>,
    active_time_16: Option<u16>,
    local_timestamp: Option<enums::LocalDateTime>,
    temperature: Option<i16>,
    temperature_min: Option<i16>,
    temperature_max: Option<i16>,
    activity_time: Option<Vec<u16>>,
    active_calories: Option<u16>,
    current_activity_type_intensity: Option<u8>,
    timestamp_min_8: Option<u8>,
    timestamp_16: Option<u16>,
    heart_rate: Option<u8>,
    intensity: Option<u8>,
    duration_min: Option<u16>,
    duration: Option<u32>,
    ascent: Option<u32>,
    descent: Option<u32>,
    moderate_activity_minutes: Option<u16>,
    vigorous_activity_minutes: Option<u16>,
}

impl From<Vec<(u8, Field)>> for Monitoring {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.device_index = field.one().map(<enums::DeviceIndex>::from),
                1 => msg.calories = field.one().map(<u16>::from),
                2 => msg.distance = field.one().map(<u32>::from),
                3 => msg.cycles = field.one().map(<u32>::from),
                4 => msg.active_time = field.one().map(<u32>::from),
                5 => msg.activity_type = field.one().map(<enums::ActivityType>::from),
                6 => msg.activity_subtype = field.one().map(<enums::ActivitySubtype>::from),
                7 => msg.activity_level = field.one().map(<enums::ActivityLevel>::from),
                8 => msg.distance_16 = field.one().map(<u16>::from),
                9 => msg.cycles_16 = field.one().map(<u16>::from),
                10 => msg.active_time_16 = field.one().map(<u16>::from),
                11 => msg.local_timestamp = field.one().map(<enums::LocalDateTime>::from),
                12 => msg.temperature = field.one().map(<i16>::from),
                14 => msg.temperature_min = field.one().map(<i16>::from),
                15 => msg.temperature_max = field.one().map(<i16>::from),
                16 => msg.activity_time = field.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                19 => msg.active_calories = field.one().map(<u16>::from),
                24 => msg.current_activity_type_intensity = field.one().map(<u8>::from),
                25 => msg.timestamp_min_8 = field.one().map(<u8>::from),
                26 => msg.timestamp_16 = field.one().map(<u16>::from),
                27 => msg.heart_rate = field.one().map(<u8>::from),
                28 => msg.intensity = field.one().map(<u8>::from),
                29 => msg.duration_min = field.one().map(<u16>::from),
                30 => msg.duration = field.one().map(<u32>::from),
                31 => msg.ascent = field.one().map(<u32>::from),
                32 => msg.descent = field.one().map(<u32>::from),
                33 => msg.moderate_activity_minutes = field.one().map(<u16>::from),
                34 => msg.vigorous_activity_minutes = field.one().map(<u16>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct Hr {
    timestamp: Option<enums::DateTime>,
    fractional_timestamp: Option<u16>,
    time256: Option<u8>,
    filtered_bpm: Option<Vec<u8>>,
    event_timestamp: Option<Vec<u32>>,
    event_timestamp_12: Option<Vec<u8>>,
}

impl From<Vec<(u8, Field)>> for Hr {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.fractional_timestamp = field.one().map(<u16>::from),
                1 => msg.time256 = field.one().map(<u8>::from),
                6 => msg.filtered_bpm = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                9 => msg.event_timestamp = field.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                10 => msg.event_timestamp_12 = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct MemoGlob {
    part_index: Option<u32>,
    memo: Option<Vec<u8>>,
    message_number: Option<u16>,
    message_index: Option<enums::MessageIndex>,
}

impl From<Vec<(u8, Field)>> for MemoGlob {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                250 => msg.part_index = field.one().map(<u32>::from),
                0 => msg.memo = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                1 => msg.message_number = field.one().map(<u16>::from),
                2 => msg.message_index = field.one().map(<enums::MessageIndex>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct AntChannelId {
    channel_number: Option<u8>,
    device_type: Option<u8>,
    device_number: Option<u16>,
    transmission_type: Option<u8>,
    device_index: Option<enums::DeviceIndex>,
}

impl From<Vec<(u8, Field)>> for AntChannelId {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.channel_number = field.one().map(<u8>::from),
                1 => msg.device_type = field.one().map(<u8>::from),
                2 => msg.device_number = field.one().map(<u16>::from),
                3 => msg.transmission_type = field.one().map(<u8>::from),
                4 => msg.device_index = field.one().map(<enums::DeviceIndex>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct AntTx {
    timestamp: Option<enums::DateTime>,
    fractional_timestamp: Option<u16>,
    mesg_id: Option<u8>,
    mesg_data: Option<Vec<u8>>,
    channel_number: Option<u8>,
    data: Option<Vec<u8>>,
}

impl From<Vec<(u8, Field)>> for AntTx {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                253 => msg.timestamp = field.one().map(<enums::DateTime>::from),
                0 => msg.fractional_timestamp = field.one().map(<u16>::from),
                1 => msg.mesg_id = field.one().map(<u8>::from),
                2 => msg.mesg_data = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                3 => msg.channel_number = field.one().map(<u8>::from),
                4 => msg.data = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct ExdDataFieldConfiguration {
    screen_index: Option<u8>,
    concept_field: Option<u8>,
    field_id: Option<u8>,
    concept_count: Option<u8>,
    display_type: Option<enums::ExdDisplayType>,
    title: Option<Vec<String>>,
}

impl From<Vec<(u8, Field)>> for ExdDataFieldConfiguration {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.screen_index = field.one().map(<u8>::from),
                1 => msg.concept_field = field.one().map(<u8>::from),
                2 => msg.field_id = field.one().map(<u8>::from),
                3 => msg.concept_count = field.one().map(<u8>::from),
                4 => msg.display_type = field.one().map(<enums::ExdDisplayType>::from),
                5 => msg.title = field.many().map(|vec| vec.into_iter().map(<String>::from).collect()),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct ExdDataConceptConfiguration {
    screen_index: Option<u8>,
    concept_field: Option<u8>,
    field_id: Option<u8>,
    concept_index: Option<u8>,
    data_page: Option<u8>,
    concept_key: Option<u8>,
    scaling: Option<u8>,
    data_units: Option<enums::ExdDataUnits>,
    qualifier: Option<enums::ExdQualifiers>,
    descriptor: Option<enums::ExdDescriptors>,
    is_signed: Option<bool>,
}

impl From<Vec<(u8, Field)>> for ExdDataConceptConfiguration {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.screen_index = field.one().map(<u8>::from),
                1 => msg.concept_field = field.one().map(<u8>::from),
                2 => msg.field_id = field.one().map(<u8>::from),
                3 => msg.concept_index = field.one().map(<u8>::from),
                4 => msg.data_page = field.one().map(<u8>::from),
                5 => msg.concept_key = field.one().map(<u8>::from),
                6 => msg.scaling = field.one().map(<u8>::from),
                8 => msg.data_units = field.one().map(<enums::ExdDataUnits>::from),
                9 => msg.qualifier = field.one().map(<enums::ExdQualifiers>::from),
                10 => msg.descriptor = field.one().map(<enums::ExdDescriptors>::from),
                11 => msg.is_signed = field.one().map(<bool>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

#[derive(Debug, Default)]
pub struct DeveloperDataId {
    developer_id: Option<Vec<u8>>,
    application_id: Option<Vec<u8>>,
    manufacturer_id: Option<enums::Manufacturer>,
    developer_data_index: Option<u8>,
    application_version: Option<u32>,
}

impl From<Vec<(u8, Field)>> for DeveloperDataId {
    fn from(fields: Vec<(u8, Field)>) -> Self {
        let mut msg: Self = Default::default();
        for (number, field) in fields {
            match number {
                0 => msg.developer_id = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                1 => msg.application_id = field.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                2 => msg.manufacturer_id = field.one().map(<enums::Manufacturer>::from),
                3 => msg.developer_data_index = field.one().map(<u8>::from),
                4 => msg.application_version = field.one().map(<u32>::from),
                v => panic!("unknown field number: {}", v)
            };
        }
        msg
    }
}

