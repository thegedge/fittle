use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CameraEventType {
    PhotoTaken,
    VideoEnd,
    VideoPause,
    VideoResume,
    VideoSecondStreamEnd,
    VideoSecondStreamPause,
    VideoSecondStreamResume,
    VideoSecondStreamSplit,
    VideoSecondStreamSplitStart,
    VideoSecondStreamStart,
    VideoSplit,
    VideoSplitStart,
    VideoStart,
    UnknownValue(u64),
}

impl From<FieldContent> for CameraEventType {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => CameraEventType::VideoStart,
                1 => CameraEventType::VideoSplit,
                2 => CameraEventType::VideoEnd,
                3 => CameraEventType::PhotoTaken,
                4 => CameraEventType::VideoSecondStreamStart,
                5 => CameraEventType::VideoSecondStreamSplit,
                6 => CameraEventType::VideoSecondStreamEnd,
                7 => CameraEventType::VideoSplitStart,
                8 => CameraEventType::VideoSecondStreamSplitStart,
                11 => CameraEventType::VideoPause,
                12 => CameraEventType::VideoSecondStreamPause,
                13 => CameraEventType::VideoResume,
                14 => CameraEventType::VideoSecondStreamResume,
                n => CameraEventType::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert CameraEventType to {:?}", field);
        }
    }
}
