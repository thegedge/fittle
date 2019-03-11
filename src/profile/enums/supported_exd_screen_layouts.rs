use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportedExdScreenLayouts {
    FullQuarterSplit,
    FullScreen,
    HalfHorizontal,
    HalfHorizontalBottomSplit,
    HalfHorizontalTopSplit,
    HalfVertical,
    HalfVerticalLeftSplit,
    HalfVerticalRightSplit,
    UnknownValue(u64),
}

impl From<FieldContent> for SupportedExdScreenLayouts {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::UnsignedInt32z(enum_value) = field {
            match enum_value {
                1 => SupportedExdScreenLayouts::FullScreen,
                2 => SupportedExdScreenLayouts::HalfVertical,
                4 => SupportedExdScreenLayouts::HalfHorizontal,
                8 => SupportedExdScreenLayouts::HalfVerticalRightSplit,
                16 => SupportedExdScreenLayouts::HalfHorizontalBottomSplit,
                32 => SupportedExdScreenLayouts::FullQuarterSplit,
                64 => SupportedExdScreenLayouts::HalfVerticalLeftSplit,
                128 => SupportedExdScreenLayouts::HalfHorizontalTopSplit,
                n => SupportedExdScreenLayouts::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert SupportedExdScreenLayouts to {:?}", field);
        }
    }
}
