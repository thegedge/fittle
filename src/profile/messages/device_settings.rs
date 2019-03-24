// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct DeviceSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    active_time_zone: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    activity_tracker_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    auto_activity_detect: Option<crate::profile::enums::AutoActivityDetect>,

    #[serde(skip_serializing_if = "Option::is_none")]
    auto_sync_frequency: Option<crate::profile::enums::AutoSyncFrequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    autosync_min_steps: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    autosync_min_time: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    backlight_mode: Option<crate::profile::enums::BacklightMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ble_auto_upload_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    clock_time: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    date_mode: Option<crate::profile::enums::DateMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    default_page: Option<Vec<u16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    display_orientation: Option<crate::profile::enums::DisplayOrientation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    lactate_threshold_autodetect_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mounting_side: Option<crate::profile::enums::Side>,

    #[serde(skip_serializing_if = "Option::is_none")]
    move_alert_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    number_of_screens: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pages_enabled: Option<Vec<u16>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    smart_notification_display_orientation: Option<crate::profile::enums::DisplayOrientation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tap_interface: Option<crate::profile::enums::Switch>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_mode: Option<Vec<crate::profile::enums::TimeMode>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_offset: Option<Vec<crate::fields::Time>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone_offset: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    utc_offset: Option<u32>,
}

impl DeviceSettings {
    pub fn from_fields<Order, Reader>(reader: &mut Reader, fields: &Vec<FieldDefinition>)
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
                2 => msg.time_offset = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::second, u32>((<u32>::from)(v))).collect()),
                4 => msg.time_mode = content.many().map(|vec| vec.into_iter().map(<crate::profile::enums::TimeMode>::from).collect()),
                5 => msg.time_zone_offset = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<i8>::from(v)) / 4.0 - 0.0 }).collect()),
                12 => msg.backlight_mode = content.one().map(<crate::profile::enums::BacklightMode>::from),
                36 => msg.activity_tracker_enabled = content.one().map(<bool>::from),
                39 => msg.clock_time = content.one().map(<crate::fields::DateTime>::from),
                40 => msg.pages_enabled = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                46 => msg.move_alert_enabled = content.one().map(<bool>::from),
                47 => msg.date_mode = content.one().map(<crate::profile::enums::DateMode>::from),
                55 => msg.display_orientation = content.one().map(<crate::profile::enums::DisplayOrientation>::from),
                56 => msg.mounting_side = content.one().map(<crate::profile::enums::Side>::from),
                57 => msg.default_page = content.many().map(|vec| vec.into_iter().map(<u16>::from).collect()),
                58 => msg.autosync_min_steps = content.one().map(<u16>::from),
                59 => msg.autosync_min_time = content.one().map(<u16>::from),
                80 => msg.lactate_threshold_autodetect_enabled = content.one().map(<bool>::from),
                86 => msg.ble_auto_upload_enabled = content.one().map(<bool>::from),
                89 => msg.auto_sync_frequency = content.one().map(<crate::profile::enums::AutoSyncFrequency>::from),
                90 => msg.auto_activity_detect = content.one().map(<crate::profile::enums::AutoActivityDetect>::from),
                94 => msg.number_of_screens = content.one().map(<u8>::from),
                95 => msg.smart_notification_display_orientation = content.one().map(<crate::profile::enums::DisplayOrientation>::from),
                134 => msg.tap_interface = content.one().map(<crate::profile::enums::Switch>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
