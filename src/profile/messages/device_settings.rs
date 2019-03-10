// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::FieldDefinition;

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

impl DeviceSettings {
    pub fn from_fields<'i, Order, Reader>(reader: &mut Reader, fields: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut msg: Self = Default::default();
        for field in fields {
            let (number, content) = field.content_from::<Order, Reader>(reader)?;
            match number {
                0 => msg.active_time_zone = content.one().map(<u8>::from),
                1 => msg.utc_offset = content.one().map(<u32>::from),
                2 => msg.time_offset = content.many().map(|vec| vec.into_iter().map(<u32>::from).collect()),
                4 => msg.time_mode = content.many().map(|vec| vec.into_iter().map(<enums::TimeMode>::from).collect()),
                5 => msg.time_zone_offset = content.many().map(|vec| vec.into_iter().map(<i8>::from).collect()),
                12 => msg.backlight_mode = content.one().map(<enums::BacklightMode>::from),
                36 => msg.activity_tracker_enabled = content.one().map(<bool>::from),
                39 => msg.clock_time = content.one().map(<enums::DateTime>::from),
                40 => msg.pages_enabled = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                46 => msg.move_alert_enabled = content.one().map(<bool>::from),
                47 => msg.date_mode = content.one().map(<enums::DateMode>::from),
                55 => msg.display_orientation = content.one().map(<enums::DisplayOrientation>::from),
                56 => msg.mounting_side = content.one().map(<enums::Side>::from),
                57 => msg.default_page = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                58 => msg.autosync_min_steps = content.one().map(<u16>::from),
                59 => msg.autosync_min_time = content.one().map(<u16>::from),
                80 => msg.lactate_threshold_autodetect_enabled = content.one().map(<bool>::from),
                86 => msg.ble_auto_upload_enabled = content.one().map(<bool>::from),
                89 => msg.auto_sync_frequency = content.one().map(<enums::AutoSyncFrequency>::from),
                90 => msg.auto_activity_detect = content.one().map(<enums::AutoActivityDetect>::from),
                94 => msg.number_of_screens = content.one().map(<u8>::from),
                95 => msg.smart_notification_display_orientation = content.one().map(<enums::DisplayOrientation>::from),
                134 => msg.tap_interface = content.one().map(<enums::Switch>::from),
                _ => (),
            };
        }
        Ok(msg)
    }
}

