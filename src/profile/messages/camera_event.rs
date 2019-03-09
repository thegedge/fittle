// DO NOT EDIT -- generated code

#[allow(unused_imports)]
use crate::profile::enums;
use crate::fields::Field;

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

