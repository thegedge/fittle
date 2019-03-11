use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CoursePoint {
    Danger,
    FirstAid,
    FirstCategory,
    Food,
    FourthCategory,
    Generic,
    HorsCategory,
    Left,
    LeftFork,
    MiddleFork,
    Right,
    RightFork,
    SecondCategory,
    SegmentEnd,
    SegmentStart,
    SharpLeft,
    SharpRight,
    SlightLeft,
    SlightRight,
    Sprint,
    Straight,
    Summit,
    ThirdCategory,
    UTurn,
    Valley,
    Water,
    UnknownValue(u64),
}

impl From<FieldContent> for CoursePoint {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => CoursePoint::Generic,
                1 => CoursePoint::Summit,
                2 => CoursePoint::Valley,
                3 => CoursePoint::Water,
                4 => CoursePoint::Food,
                5 => CoursePoint::Danger,
                6 => CoursePoint::Left,
                7 => CoursePoint::Right,
                8 => CoursePoint::Straight,
                9 => CoursePoint::FirstAid,
                10 => CoursePoint::FourthCategory,
                11 => CoursePoint::ThirdCategory,
                12 => CoursePoint::SecondCategory,
                13 => CoursePoint::FirstCategory,
                14 => CoursePoint::HorsCategory,
                15 => CoursePoint::Sprint,
                16 => CoursePoint::LeftFork,
                17 => CoursePoint::RightFork,
                18 => CoursePoint::MiddleFork,
                19 => CoursePoint::SlightLeft,
                20 => CoursePoint::SharpLeft,
                21 => CoursePoint::SlightRight,
                22 => CoursePoint::SharpRight,
                23 => CoursePoint::UTurn,
                24 => CoursePoint::SegmentStart,
                25 => CoursePoint::SegmentEnd,
                n => CoursePoint::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert CoursePoint to {:?}", field);
        }
    }
}
