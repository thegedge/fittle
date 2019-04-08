// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

#[allow(unused_imports)]
use crate::bits::BitReader;

#[allow(unused_imports)]
use crate::fields::{
    Field,
    FieldContent,
    FieldDefinition,
};

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
    autosync_min_time: Option<crate::fields::Time>,

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
    time_zone_offset: Option<Vec<crate::fields::Time>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    utc_offset: Option<u32>,
}

impl DeviceSettings {
    pub fn from_fields<Order, Reader>(reader: &mut Reader, field_defs: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut msg: Self = Default::default();
        for field_def in field_defs {
            let (number, field) = field_def.content_from::<Order, Reader>(reader)?;
            msg.from_content(number, field);
        }

        Ok(msg)
    }

    fn from_content(&mut self, number: u8, field: Field) {
        match number {
            0 => {
                self.active_time_zone =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            1 => {
                self.utc_offset =field.one().map(|v| {
                    let value = u32::from(v);
                    value
                })
            },

            2 => {
                self.time_offset =field.many().map(|v| {
                    let value = v.into_iter().map(u32::from).collect::<Vec<_>>();
                    value.into_iter().map(crate::fields::Time::new::<uom::si::time::second, u32>).collect()
                })
            },

            4 => {
                self.time_mode =field.many().map(|v| {
                    let value = v.into_iter().map(crate::profile::enums::TimeMode::from).collect::<Vec<_>>();
                    value
                })
            },

            5 => {
                self.time_zone_offset =field.many().map(|v| {
                    let value = v.into_iter().map(i8::from).collect::<Vec<_>>();
                    value.into_iter().map(|v| crate::fields::Time::new::<uom::si::time::hour, f64>((|v| { f64::from(v) / 4.0 - 0.0 })(v))).collect()
                })
            },

            12 => {
                self.backlight_mode =field.one().map(|v| {
                    let value = crate::profile::enums::BacklightMode::from(v);
                    value
                })
            },

            36 => {
                self.activity_tracker_enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            39 => {
                self.clock_time =field.one().map(|v| {
                    let value = crate::fields::DateTime::from(v);
                    value
                })
            },

            40 => {
                self.pages_enabled =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value
                })
            },

            46 => {
                self.move_alert_enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            47 => {
                self.date_mode =field.one().map(|v| {
                    let value = crate::profile::enums::DateMode::from(v);
                    value
                })
            },

            55 => {
                self.display_orientation =field.one().map(|v| {
                    let value = crate::profile::enums::DisplayOrientation::from(v);
                    value
                })
            },

            56 => {
                self.mounting_side =field.one().map(|v| {
                    let value = crate::profile::enums::Side::from(v);
                    value
                })
            },

            57 => {
                self.default_page =field.many().map(|v| {
                    let value = v.into_iter().map(u16::from).collect::<Vec<_>>();
                    value
                })
            },

            58 => {
                self.autosync_min_steps =field.one().map(|v| {
                    let value = u16::from(v);
                    value
                })
            },

            59 => {
                self.autosync_min_time =field.one().map(|v| {
                    let value = u16::from(v);
                    (crate::fields::Time::new::<uom::si::time::minute, u16>)(value)
                })
            },

            80 => {
                self.lactate_threshold_autodetect_enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            86 => {
                self.ble_auto_upload_enabled =field.one().map(|v| {
                    let value = bool::from(v);
                    value
                })
            },

            89 => {
                self.auto_sync_frequency =field.one().map(|v| {
                    let value = crate::profile::enums::AutoSyncFrequency::from(v);
                    value
                })
            },

            90 => {
                self.auto_activity_detect =field.one().map(|v| {
                    let value = crate::profile::enums::AutoActivityDetect::from(v);
                    value
                })
            },

            94 => {
                self.number_of_screens =field.one().map(|v| {
                    let value = u8::from(v);
                    value
                })
            },

            95 => {
                self.smart_notification_display_orientation =field.one().map(|v| {
                    let value = crate::profile::enums::DisplayOrientation::from(v);
                    value
                })
            },

            134 => {
                self.tap_interface =field.one().map(|v| {
                    let value = crate::profile::enums::Switch::from(v);
                    value
                })
            },

            _ => (),
        }
    }
}
