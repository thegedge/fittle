use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SegmentLeaderboardType {
    Challenger,
    ClubLeader,
    Connections,
    Goal,
    Group,
    Kom,
    Overall,
    PersonalBest,
    Pr,
    Qom,
    Rival,
    UnknownValue(u64),
}

impl From<FieldContent> for SegmentLeaderboardType {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => SegmentLeaderboardType::Overall,
                1 => SegmentLeaderboardType::PersonalBest,
                2 => SegmentLeaderboardType::Connections,
                3 => SegmentLeaderboardType::Group,
                4 => SegmentLeaderboardType::Challenger,
                5 => SegmentLeaderboardType::Kom,
                6 => SegmentLeaderboardType::Qom,
                7 => SegmentLeaderboardType::Pr,
                8 => SegmentLeaderboardType::Goal,
                9 => SegmentLeaderboardType::Rival,
                10 => SegmentLeaderboardType::ClubLeader,
                n => SegmentLeaderboardType::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert SegmentLeaderboardType to {:?}", field);
        }
    }
}
