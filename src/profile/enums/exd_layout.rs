use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExdLayout {
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

impl From<FieldContent> for ExdLayout {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => ExdLayout::FullScreen,
                1 => ExdLayout::HalfVertical,
                2 => ExdLayout::HalfHorizontal,
                3 => ExdLayout::HalfVerticalRightSplit,
                4 => ExdLayout::HalfHorizontalBottomSplit,
                5 => ExdLayout::FullQuarterSplit,
                6 => ExdLayout::HalfVerticalLeftSplit,
                7 => ExdLayout::HalfHorizontalTopSplit,
                n => ExdLayout::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert ExdLayout to {:?}", field);
        }
    }
}
