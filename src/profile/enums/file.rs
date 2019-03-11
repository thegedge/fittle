use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum File {
    Activity,
    ActivitySummary,
    BloodPressure,
    Course,
    Device,
    ExdConfiguration,
    Goals,
    MonitoringA,
    MonitoringB,
    MonitoringDaily,
    Schedules,
    Segment,
    SegmentList,
    Settings,
    Sport,
    Totals,
    Weight,
    Workout,
    UnknownValue(u64),
}

impl From<FieldContent> for File {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                1 => File::Device,
                2 => File::Settings,
                3 => File::Sport,
                4 => File::Activity,
                5 => File::Workout,
                6 => File::Course,
                7 => File::Schedules,
                9 => File::Weight,
                10 => File::Totals,
                11 => File::Goals,
                14 => File::BloodPressure,
                15 => File::MonitoringA,
                20 => File::ActivitySummary,
                28 => File::MonitoringDaily,
                32 => File::MonitoringB,
                34 => File::Segment,
                35 => File::SegmentList,
                40 => File::ExdConfiguration,
                n => File::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert File to {:?}", field);
        }
    }
}
