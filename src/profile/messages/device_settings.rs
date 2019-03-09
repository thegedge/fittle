// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

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

