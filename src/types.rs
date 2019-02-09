// DO NOT EDIT -- generated code

#[derive(Clone, Debug)]
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

impl<N> From<N> for File
    where N: Into<u64>
{
    fn from(number: N) -> File {
        match number.into() {
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
            n => File::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum FileFlags {
    Erase,
    Read,
    Write,
    UnknownValue(u64),
}

impl<N> From<N> for FileFlags
    where N: Into<u64>
{
    fn from(number: N) -> FileFlags {
        match number.into() {
            2 => FileFlags::Read,
            4 => FileFlags::Write,
            8 => FileFlags::Erase,
            n => FileFlags::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum MesgCount {
    MaxPerFile,
    MaxPerFileType,
    NumPerFile,
    UnknownValue(u64),
}

impl<N> From<N> for MesgCount
    where N: Into<u64>
{
    fn from(number: N) -> MesgCount {
        match number.into() {
            0 => MesgCount::NumPerFile,
            1 => MesgCount::MaxPerFile,
            2 => MesgCount::MaxPerFileType,
            n => MesgCount::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum DateTime {
    Min,
    UnknownValue(u64),
}

impl<N> From<N> for DateTime
    where N: Into<u64>
{
    fn from(number: N) -> DateTime {
        match number.into() {
            268435456 => DateTime::Min,
            n => DateTime::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum LocalDateTime {
    Min,
    UnknownValue(u64),
}

impl<N> From<N> for LocalDateTime
    where N: Into<u64>
{
    fn from(number: N) -> LocalDateTime {
        match number.into() {
            268435456 => LocalDateTime::Min,
            n => LocalDateTime::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum MessageIndex {
    Mask,
    Reserved,
    Selected,
    UnknownValue(u64),
}

impl<N> From<N> for MessageIndex
    where N: Into<u64>
{
    fn from(number: N) -> MessageIndex {
        match number.into() {
            4095 => MessageIndex::Mask,
            28672 => MessageIndex::Reserved,
            32768 => MessageIndex::Selected,
            n => MessageIndex::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum DeviceIndex {
    Creator,
    UnknownValue(u64),
}

impl<N> From<N> for DeviceIndex
    where N: Into<u64>
{
    fn from(number: N) -> DeviceIndex {
        match number.into() {
            0 => DeviceIndex::Creator,
            n => DeviceIndex::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum Gender {
    Female,
    Male,
    UnknownValue(u64),
}

impl<N> From<N> for Gender
    where N: Into<u64>
{
    fn from(number: N) -> Gender {
        match number.into() {
            0 => Gender::Female,
            1 => Gender::Male,
            n => Gender::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum Language {
    Arabic,
    BrazilianPortuguese,
    Bulgarian,
    Burmese,
    Chinese,
    Croatian,
    Custom,
    Czech,
    Danish,
    Dutch,
    English,
    Farsi,
    Finnish,
    French,
    German,
    Greek,
    Hebrew,
    Hungarian,
    Indonesian,
    Italian,
    Japanese,
    Korean,
    Latvian,
    Malaysian,
    Mongolian,
    Norwegian,
    Polish,
    Portuguese,
    Romanian,
    Russian,
    Slovakian,
    Slovenian,
    Spanish,
    Swedish,
    Taiwanese,
    Thai,
    Turkish,
    Ukrainian,
    Vietnamese,
    UnknownValue(u64),
}

impl<N> From<N> for Language
    where N: Into<u64>
{
    fn from(number: N) -> Language {
        match number.into() {
            0 => Language::English,
            1 => Language::French,
            2 => Language::Italian,
            3 => Language::German,
            4 => Language::Spanish,
            5 => Language::Croatian,
            6 => Language::Czech,
            7 => Language::Danish,
            8 => Language::Dutch,
            9 => Language::Finnish,
            10 => Language::Greek,
            11 => Language::Hungarian,
            12 => Language::Norwegian,
            13 => Language::Polish,
            14 => Language::Portuguese,
            15 => Language::Slovakian,
            16 => Language::Slovenian,
            17 => Language::Swedish,
            18 => Language::Russian,
            19 => Language::Turkish,
            20 => Language::Latvian,
            21 => Language::Ukrainian,
            22 => Language::Arabic,
            23 => Language::Farsi,
            24 => Language::Bulgarian,
            25 => Language::Romanian,
            26 => Language::Chinese,
            27 => Language::Japanese,
            28 => Language::Korean,
            29 => Language::Taiwanese,
            30 => Language::Thai,
            31 => Language::Hebrew,
            32 => Language::BrazilianPortuguese,
            33 => Language::Indonesian,
            34 => Language::Malaysian,
            35 => Language::Vietnamese,
            36 => Language::Burmese,
            37 => Language::Mongolian,
            254 => Language::Custom,
            n => Language::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum LanguageBits0 {
    Croatian,
    Czech,
    Danish,
    English,
    French,
    German,
    Italian,
    Spanish,
    UnknownValue(u64),
}

impl<N> From<N> for LanguageBits0
    where N: Into<u64>
{
    fn from(number: N) -> LanguageBits0 {
        match number.into() {
            1 => LanguageBits0::English,
            2 => LanguageBits0::French,
            4 => LanguageBits0::Italian,
            8 => LanguageBits0::German,
            16 => LanguageBits0::Spanish,
            32 => LanguageBits0::Croatian,
            64 => LanguageBits0::Czech,
            128 => LanguageBits0::Danish,
            n => LanguageBits0::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum LanguageBits1 {
    Dutch,
    Finnish,
    Greek,
    Hungarian,
    Norwegian,
    Polish,
    Portuguese,
    Slovakian,
    UnknownValue(u64),
}

impl<N> From<N> for LanguageBits1
    where N: Into<u64>
{
    fn from(number: N) -> LanguageBits1 {
        match number.into() {
            1 => LanguageBits1::Dutch,
            2 => LanguageBits1::Finnish,
            4 => LanguageBits1::Greek,
            8 => LanguageBits1::Hungarian,
            16 => LanguageBits1::Norwegian,
            32 => LanguageBits1::Polish,
            64 => LanguageBits1::Portuguese,
            128 => LanguageBits1::Slovakian,
            n => LanguageBits1::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum LanguageBits2 {
    Arabic,
    Farsi,
    Latvian,
    Russian,
    Slovenian,
    Swedish,
    Turkish,
    Ukrainian,
    UnknownValue(u64),
}

impl<N> From<N> for LanguageBits2
    where N: Into<u64>
{
    fn from(number: N) -> LanguageBits2 {
        match number.into() {
            1 => LanguageBits2::Slovenian,
            2 => LanguageBits2::Swedish,
            4 => LanguageBits2::Russian,
            8 => LanguageBits2::Turkish,
            16 => LanguageBits2::Latvian,
            32 => LanguageBits2::Ukrainian,
            64 => LanguageBits2::Arabic,
            128 => LanguageBits2::Farsi,
            n => LanguageBits2::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum LanguageBits3 {
    Bulgarian,
    Chinese,
    Hebrew,
    Japanese,
    Korean,
    Romanian,
    Taiwanese,
    Thai,
    UnknownValue(u64),
}

impl<N> From<N> for LanguageBits3
    where N: Into<u64>
{
    fn from(number: N) -> LanguageBits3 {
        match number.into() {
            1 => LanguageBits3::Bulgarian,
            2 => LanguageBits3::Romanian,
            4 => LanguageBits3::Chinese,
            8 => LanguageBits3::Japanese,
            16 => LanguageBits3::Korean,
            32 => LanguageBits3::Taiwanese,
            64 => LanguageBits3::Thai,
            128 => LanguageBits3::Hebrew,
            n => LanguageBits3::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum LanguageBits4 {
    BrazilianPortuguese,
    Burmese,
    Indonesian,
    Malaysian,
    Mongolian,
    Vietnamese,
    UnknownValue(u64),
}

impl<N> From<N> for LanguageBits4
    where N: Into<u64>
{
    fn from(number: N) -> LanguageBits4 {
        match number.into() {
            1 => LanguageBits4::BrazilianPortuguese,
            2 => LanguageBits4::Indonesian,
            4 => LanguageBits4::Malaysian,
            8 => LanguageBits4::Vietnamese,
            16 => LanguageBits4::Burmese,
            32 => LanguageBits4::Mongolian,
            n => LanguageBits4::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum DisplayMeasure {
    Metric,
    Nautical,
    Statute,
    UnknownValue(u64),
}

impl<N> From<N> for DisplayMeasure
    where N: Into<u64>
{
    fn from(number: N) -> DisplayMeasure {
        match number.into() {
            0 => DisplayMeasure::Metric,
            1 => DisplayMeasure::Statute,
            2 => DisplayMeasure::Nautical,
            n => DisplayMeasure::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum DisplayHeart {
    Bpm,
    Max,
    Reserve,
    UnknownValue(u64),
}

impl<N> From<N> for DisplayHeart
    where N: Into<u64>
{
    fn from(number: N) -> DisplayHeart {
        match number.into() {
            0 => DisplayHeart::Bpm,
            1 => DisplayHeart::Max,
            2 => DisplayHeart::Reserve,
            n => DisplayHeart::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum DisplayPower {
    PercentFtp,
    Watts,
    UnknownValue(u64),
}

impl<N> From<N> for DisplayPower
    where N: Into<u64>
{
    fn from(number: N) -> DisplayPower {
        match number.into() {
            0 => DisplayPower::Watts,
            1 => DisplayPower::PercentFtp,
            n => DisplayPower::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum DisplayPosition {
    AustrianGrid,
    BorneoRso,
    BritishGrid,
    Degree,
    DegreeMinute,
    DegreeMinuteSecond,
    DutchGrid,
    EstonianGrid,
    FinnishGrid,
    GermanGrid,
    HungarianGrid,
    IcelandicGrid,
    IndiaZone0,
    IndiaZoneIA,
    IndiaZoneIB,
    IndiaZoneIIA,
    IndiaZoneIIB,
    IndiaZoneIIIA,
    IndiaZoneIIIB,
    IndiaZoneIVA,
    IndiaZoneIVB,
    IndonesianEquatorial,
    IndonesianIrian,
    IndonesianSouthern,
    IrishGrid,
    IrishTransverse,
    LatvianGrid,
    Loran,
    MaidenheadGrid,
    MgrsGrid,
    ModifiedSwedishGrid,
    NewZealandGrid,
    NewZealandTransverse,
    QatarGrid,
    SouthAfricanGrid,
    SwedishGrid,
    SwedishRef99Grid,
    SwissGrid,
    TaiwanGrid,
    UnitedStatesGrid,
    UtmUpsGrid,
    WestMalayan,
    UnknownValue(u64),
}

impl<N> From<N> for DisplayPosition
    where N: Into<u64>
{
    fn from(number: N) -> DisplayPosition {
        match number.into() {
            0 => DisplayPosition::Degree,
            1 => DisplayPosition::DegreeMinute,
            2 => DisplayPosition::DegreeMinuteSecond,
            3 => DisplayPosition::AustrianGrid,
            4 => DisplayPosition::BritishGrid,
            5 => DisplayPosition::DutchGrid,
            6 => DisplayPosition::HungarianGrid,
            7 => DisplayPosition::FinnishGrid,
            8 => DisplayPosition::GermanGrid,
            9 => DisplayPosition::IcelandicGrid,
            10 => DisplayPosition::IndonesianEquatorial,
            11 => DisplayPosition::IndonesianIrian,
            12 => DisplayPosition::IndonesianSouthern,
            13 => DisplayPosition::IndiaZone0,
            14 => DisplayPosition::IndiaZoneIA,
            15 => DisplayPosition::IndiaZoneIB,
            16 => DisplayPosition::IndiaZoneIIA,
            17 => DisplayPosition::IndiaZoneIIB,
            18 => DisplayPosition::IndiaZoneIIIA,
            19 => DisplayPosition::IndiaZoneIIIB,
            20 => DisplayPosition::IndiaZoneIVA,
            21 => DisplayPosition::IndiaZoneIVB,
            22 => DisplayPosition::IrishTransverse,
            23 => DisplayPosition::IrishGrid,
            24 => DisplayPosition::Loran,
            25 => DisplayPosition::MaidenheadGrid,
            26 => DisplayPosition::MgrsGrid,
            27 => DisplayPosition::NewZealandGrid,
            28 => DisplayPosition::NewZealandTransverse,
            29 => DisplayPosition::QatarGrid,
            30 => DisplayPosition::ModifiedSwedishGrid,
            31 => DisplayPosition::SwedishGrid,
            32 => DisplayPosition::SouthAfricanGrid,
            33 => DisplayPosition::SwissGrid,
            34 => DisplayPosition::TaiwanGrid,
            35 => DisplayPosition::UnitedStatesGrid,
            36 => DisplayPosition::UtmUpsGrid,
            37 => DisplayPosition::WestMalayan,
            38 => DisplayPosition::BorneoRso,
            39 => DisplayPosition::EstonianGrid,
            40 => DisplayPosition::LatvianGrid,
            41 => DisplayPosition::SwedishRef99Grid,
            n => DisplayPosition::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum Switch {
    Auto,
    Off,
    On,
    UnknownValue(u64),
}

impl<N> From<N> for Switch
    where N: Into<u64>
{
    fn from(number: N) -> Switch {
        match number.into() {
            0 => Switch::Off,
            1 => Switch::On,
            2 => Switch::Auto,
            n => Switch::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum SportBits0 {
    Basketball,
    Cycling,
    FitnessEquipment,
    Generic,
    Running,
    Soccer,
    Swimming,
    Transition,
    UnknownValue(u64),
}

impl<N> From<N> for SportBits0
    where N: Into<u64>
{
    fn from(number: N) -> SportBits0 {
        match number.into() {
            1 => SportBits0::Generic,
            2 => SportBits0::Running,
            4 => SportBits0::Cycling,
            8 => SportBits0::Transition,
            16 => SportBits0::FitnessEquipment,
            32 => SportBits0::Swimming,
            64 => SportBits0::Basketball,
            128 => SportBits0::Soccer,
            n => SportBits0::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum SportBits1 {
    AlpineSkiing,
    AmericanFootball,
    CrossCountrySkiing,
    Rowing,
    Snowboarding,
    Tennis,
    Training,
    Walking,
    UnknownValue(u64),
}

impl<N> From<N> for SportBits1
    where N: Into<u64>
{
    fn from(number: N) -> SportBits1 {
        match number.into() {
            1 => SportBits1::Tennis,
            2 => SportBits1::AmericanFootball,
            4 => SportBits1::Training,
            8 => SportBits1::Walking,
            16 => SportBits1::CrossCountrySkiing,
            32 => SportBits1::AlpineSkiing,
            64 => SportBits1::Snowboarding,
            128 => SportBits1::Rowing,
            n => SportBits1::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum SportBits2 {
    Boating,
    EBiking,
    Flying,
    Hiking,
    Motorcycling,
    Mountaineering,
    Multisport,
    Paddling,
    UnknownValue(u64),
}

impl<N> From<N> for SportBits2
    where N: Into<u64>
{
    fn from(number: N) -> SportBits2 {
        match number.into() {
            1 => SportBits2::Mountaineering,
            2 => SportBits2::Hiking,
            4 => SportBits2::Multisport,
            8 => SportBits2::Paddling,
            16 => SportBits2::Flying,
            32 => SportBits2::EBiking,
            64 => SportBits2::Motorcycling,
            128 => SportBits2::Boating,
            n => SportBits2::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum SportBits3 {
    Driving,
    Fishing,
    Golf,
    HangGliding,
    HorsebackRiding,
    Hunting,
    InlineSkating,
    RockClimbing,
    UnknownValue(u64),
}

impl<N> From<N> for SportBits3
    where N: Into<u64>
{
    fn from(number: N) -> SportBits3 {
        match number.into() {
            1 => SportBits3::Driving,
            2 => SportBits3::Golf,
            4 => SportBits3::HangGliding,
            8 => SportBits3::HorsebackRiding,
            16 => SportBits3::Hunting,
            32 => SportBits3::Fishing,
            64 => SportBits3::InlineSkating,
            128 => SportBits3::RockClimbing,
            n => SportBits3::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum SportBits4 {
    IceSkating,
    Sailing,
    SkyDiving,
    Snowmobiling,
    Snowshoeing,
    StandUpPaddleboarding,
    Surfing,
    Wakeboarding,
    UnknownValue(u64),
}

impl<N> From<N> for SportBits4
    where N: Into<u64>
{
    fn from(number: N) -> SportBits4 {
        match number.into() {
            1 => SportBits4::Sailing,
            2 => SportBits4::IceSkating,
            4 => SportBits4::SkyDiving,
            8 => SportBits4::Snowshoeing,
            16 => SportBits4::Snowmobiling,
            32 => SportBits4::StandUpPaddleboarding,
            64 => SportBits4::Surfing,
            128 => SportBits4::Wakeboarding,
            n => SportBits4::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum SportBits5 {
    Boxing,
    Jumpmaster,
    Kayaking,
    Kitesurfing,
    Rafting,
    Tactical,
    WaterSkiing,
    Windsurfing,
    UnknownValue(u64),
}

impl<N> From<N> for SportBits5
    where N: Into<u64>
{
    fn from(number: N) -> SportBits5 {
        match number.into() {
            1 => SportBits5::WaterSkiing,
            2 => SportBits5::Kayaking,
            4 => SportBits5::Rafting,
            8 => SportBits5::Windsurfing,
            16 => SportBits5::Kitesurfing,
            32 => SportBits5::Tactical,
            64 => SportBits5::Jumpmaster,
            128 => SportBits5::Boxing,
            n => SportBits5::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum SportBits6 {
    FloorClimbing,
    UnknownValue(u64),
}

impl<N> From<N> for SportBits6
    where N: Into<u64>
{
    fn from(number: N) -> SportBits6 {
        match number.into() {
            1 => SportBits6::FloorClimbing,
            n => SportBits6::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum SubSport {
    All,
    ApneaDiving,
    ApneaHunting,
    Atv,
    Backcountry,
    BikeToRunTransition,
    Bmx,
    CardioTraining,
    CasualWalking,
    Challenge,
    Commuting,
    Cyclocross,
    Downhill,
    EBikeFitness,
    EBikeMountain,
    Elliptical,
    Exercise,
    FlexibilityTraining,
    GaugeDiving,
    Generic,
    GravelCycling,
    HandCycling,
    IndoorCycling,
    IndoorRowing,
    IndoorRunning,
    IndoorSkiing,
    IndoorWalking,
    LapSwimming,
    Map,
    Match,
    MixedSurface,
    Motocross,
    Mountain,
    MultiGasDiving,
    Navigate,
    Obstacle,
    OpenWater,
    Pilates,
    RcDrone,
    Recumbent,
    Resort,
    Road,
    RunToBikeTransition,
    SingleGasDiving,
    SkateSkiing,
    SpeedWalking,
    Spin,
    StairClimbing,
    Street,
    StrengthTraining,
    SwimToBikeTransition,
    Track,
    TrackCycling,
    TrackMe,
    Trail,
    Treadmill,
    VirtualActivity,
    WarmUp,
    Whitewater,
    Wingsuit,
    Yoga,
    UnknownValue(u64),
}

impl<N> From<N> for SubSport
    where N: Into<u64>
{
    fn from(number: N) -> SubSport {
        match number.into() {
            0 => SubSport::Generic,
            1 => SubSport::Treadmill,
            2 => SubSport::Street,
            3 => SubSport::Trail,
            4 => SubSport::Track,
            5 => SubSport::Spin,
            6 => SubSport::IndoorCycling,
            7 => SubSport::Road,
            8 => SubSport::Mountain,
            9 => SubSport::Downhill,
            10 => SubSport::Recumbent,
            11 => SubSport::Cyclocross,
            12 => SubSport::HandCycling,
            13 => SubSport::TrackCycling,
            14 => SubSport::IndoorRowing,
            15 => SubSport::Elliptical,
            16 => SubSport::StairClimbing,
            17 => SubSport::LapSwimming,
            18 => SubSport::OpenWater,
            19 => SubSport::FlexibilityTraining,
            20 => SubSport::StrengthTraining,
            21 => SubSport::WarmUp,
            22 => SubSport::Match,
            23 => SubSport::Exercise,
            24 => SubSport::Challenge,
            25 => SubSport::IndoorSkiing,
            26 => SubSport::CardioTraining,
            27 => SubSport::IndoorWalking,
            28 => SubSport::EBikeFitness,
            29 => SubSport::Bmx,
            30 => SubSport::CasualWalking,
            31 => SubSport::SpeedWalking,
            32 => SubSport::BikeToRunTransition,
            33 => SubSport::RunToBikeTransition,
            34 => SubSport::SwimToBikeTransition,
            35 => SubSport::Atv,
            36 => SubSport::Motocross,
            37 => SubSport::Backcountry,
            38 => SubSport::Resort,
            39 => SubSport::RcDrone,
            40 => SubSport::Wingsuit,
            41 => SubSport::Whitewater,
            42 => SubSport::SkateSkiing,
            43 => SubSport::Yoga,
            44 => SubSport::Pilates,
            45 => SubSport::IndoorRunning,
            46 => SubSport::GravelCycling,
            47 => SubSport::EBikeMountain,
            48 => SubSport::Commuting,
            49 => SubSport::MixedSurface,
            50 => SubSport::Navigate,
            51 => SubSport::TrackMe,
            52 => SubSport::Map,
            53 => SubSport::SingleGasDiving,
            54 => SubSport::MultiGasDiving,
            55 => SubSport::GaugeDiving,
            56 => SubSport::ApneaDiving,
            57 => SubSport::ApneaHunting,
            58 => SubSport::VirtualActivity,
            59 => SubSport::Obstacle,
            254 => SubSport::All,
            n => SubSport::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum SportEvent {
    Fitness,
    Geocaching,
    Race,
    Recreation,
    SpecialEvent,
    Touring,
    Training,
    Transportation,
    Uncategorized,
    UnknownValue(u64),
}

impl<N> From<N> for SportEvent
    where N: Into<u64>
{
    fn from(number: N) -> SportEvent {
        match number.into() {
            0 => SportEvent::Uncategorized,
            1 => SportEvent::Geocaching,
            2 => SportEvent::Fitness,
            3 => SportEvent::Recreation,
            4 => SportEvent::Race,
            5 => SportEvent::SpecialEvent,
            6 => SportEvent::Training,
            7 => SportEvent::Transportation,
            8 => SportEvent::Touring,
            n => SportEvent::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum Activity {
    AutoMultiSport,
    Manual,
    UnknownValue(u64),
}

impl<N> From<N> for Activity
    where N: Into<u64>
{
    fn from(number: N) -> Activity {
        match number.into() {
            0 => Activity::Manual,
            1 => Activity::AutoMultiSport,
            n => Activity::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum Intensity {
    Active,
    Cooldown,
    Rest,
    Warmup,
    UnknownValue(u64),
}

impl<N> From<N> for Intensity
    where N: Into<u64>
{
    fn from(number: N) -> Intensity {
        match number.into() {
            0 => Intensity::Active,
            1 => Intensity::Rest,
            2 => Intensity::Warmup,
            3 => Intensity::Cooldown,
            n => Intensity::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum SessionTrigger {
    ActivityEnd,
    AutoMultiSport,
    FitnessEquipment,
    Manual,
    UnknownValue(u64),
}

impl<N> From<N> for SessionTrigger
    where N: Into<u64>
{
    fn from(number: N) -> SessionTrigger {
        match number.into() {
            0 => SessionTrigger::ActivityEnd,
            1 => SessionTrigger::Manual,
            2 => SessionTrigger::AutoMultiSport,
            3 => SessionTrigger::FitnessEquipment,
            n => SessionTrigger::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum AutolapTrigger {
    Distance,
    Off,
    PositionLap,
    PositionMarked,
    PositionStart,
    PositionWaypoint,
    Time,
    UnknownValue(u64),
}

impl<N> From<N> for AutolapTrigger
    where N: Into<u64>
{
    fn from(number: N) -> AutolapTrigger {
        match number.into() {
            0 => AutolapTrigger::Time,
            1 => AutolapTrigger::Distance,
            2 => AutolapTrigger::PositionStart,
            3 => AutolapTrigger::PositionLap,
            4 => AutolapTrigger::PositionWaypoint,
            5 => AutolapTrigger::PositionMarked,
            6 => AutolapTrigger::Off,
            n => AutolapTrigger::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum LapTrigger {
    Distance,
    FitnessEquipment,
    Manual,
    PositionLap,
    PositionMarked,
    PositionStart,
    PositionWaypoint,
    SessionEnd,
    Time,
    UnknownValue(u64),
}

impl<N> From<N> for LapTrigger
    where N: Into<u64>
{
    fn from(number: N) -> LapTrigger {
        match number.into() {
            0 => LapTrigger::Manual,
            1 => LapTrigger::Time,
            2 => LapTrigger::Distance,
            3 => LapTrigger::PositionStart,
            4 => LapTrigger::PositionLap,
            5 => LapTrigger::PositionWaypoint,
            6 => LapTrigger::PositionMarked,
            7 => LapTrigger::SessionEnd,
            8 => LapTrigger::FitnessEquipment,
            n => LapTrigger::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum TimeMode {
    Hour12,
    Hour12WithSeconds,
    Hour24,
    Hour24WithSeconds,
    Military,
    Utc,
    UnknownValue(u64),
}

impl<N> From<N> for TimeMode
    where N: Into<u64>
{
    fn from(number: N) -> TimeMode {
        match number.into() {
            0 => TimeMode::Hour12,
            1 => TimeMode::Hour24,
            2 => TimeMode::Military,
            3 => TimeMode::Hour12WithSeconds,
            4 => TimeMode::Hour24WithSeconds,
            5 => TimeMode::Utc,
            n => TimeMode::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum DateMode {
    DayMonth,
    MonthDay,
    UnknownValue(u64),
}

impl<N> From<N> for DateMode
    where N: Into<u64>
{
    fn from(number: N) -> DateMode {
        match number.into() {
            0 => DateMode::DayMonth,
            1 => DateMode::MonthDay,
            n => DateMode::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum Event {
    Activity,
    Battery,
    BatteryLow,
    CadHighAlert,
    CadLowAlert,
    Calibration,
    CalorieDurationAlert,
    CommTimeout,
    CoursePoint,
    DistanceDurationAlert,
    ElevHighAlert,
    ElevLowAlert,
    FitnessEquipment,
    FrontGearChange,
    HrHighAlert,
    HrLowAlert,
    Lap,
    Length,
    OffCourse,
    PowerDown,
    PowerHighAlert,
    PowerLowAlert,
    PowerUp,
    RearGearChange,
    RecoveryHr,
    RiderPositionChange,
    Session,
    SpeedHighAlert,
    SpeedLowAlert,
    SportPoint,
    TimeDurationAlert,
    Timer,
    UserMarker,
    VirtualPartnerPace,
    Workout,
    WorkoutStep,
    UnknownValue(u64),
}

impl<N> From<N> for Event
    where N: Into<u64>
{
    fn from(number: N) -> Event {
        match number.into() {
            0 => Event::Timer,
            3 => Event::Workout,
            4 => Event::WorkoutStep,
            5 => Event::PowerDown,
            6 => Event::PowerUp,
            7 => Event::OffCourse,
            8 => Event::Session,
            9 => Event::Lap,
            10 => Event::CoursePoint,
            11 => Event::Battery,
            12 => Event::VirtualPartnerPace,
            13 => Event::HrHighAlert,
            14 => Event::HrLowAlert,
            15 => Event::SpeedHighAlert,
            16 => Event::SpeedLowAlert,
            17 => Event::CadHighAlert,
            18 => Event::CadLowAlert,
            19 => Event::PowerHighAlert,
            20 => Event::PowerLowAlert,
            21 => Event::RecoveryHr,
            22 => Event::BatteryLow,
            23 => Event::TimeDurationAlert,
            24 => Event::DistanceDurationAlert,
            25 => Event::CalorieDurationAlert,
            26 => Event::Activity,
            27 => Event::FitnessEquipment,
            28 => Event::Length,
            32 => Event::UserMarker,
            33 => Event::SportPoint,
            36 => Event::Calibration,
            42 => Event::FrontGearChange,
            43 => Event::RearGearChange,
            44 => Event::RiderPositionChange,
            45 => Event::ElevHighAlert,
            46 => Event::ElevLowAlert,
            47 => Event::CommTimeout,
            n => Event::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum EventType {
    BeginDepreciated,
    ConsecutiveDepreciated,
    EndAllDepreciated,
    EndDepreciated,
    Marker,
    Start,
    Stop,
    StopAll,
    StopDisable,
    StopDisableAll,
    UnknownValue(u64),
}

impl<N> From<N> for EventType
    where N: Into<u64>
{
    fn from(number: N) -> EventType {
        match number.into() {
            0 => EventType::Start,
            1 => EventType::Stop,
            2 => EventType::ConsecutiveDepreciated,
            3 => EventType::Marker,
            4 => EventType::StopAll,
            5 => EventType::BeginDepreciated,
            6 => EventType::EndDepreciated,
            7 => EventType::EndAllDepreciated,
            8 => EventType::StopDisable,
            9 => EventType::StopDisableAll,
            n => EventType::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum TimerTrigger {
    Auto,
    FitnessEquipment,
    Manual,
    UnknownValue(u64),
}

impl<N> From<N> for TimerTrigger
    where N: Into<u64>
{
    fn from(number: N) -> TimerTrigger {
        match number.into() {
            0 => TimerTrigger::Manual,
            1 => TimerTrigger::Auto,
            2 => TimerTrigger::FitnessEquipment,
            n => TimerTrigger::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum FitnessEquipmentState {
    InUse,
    Paused,
    Ready,
    Unknown,
    UnknownValue(u64),
}

impl<N> From<N> for FitnessEquipmentState
    where N: Into<u64>
{
    fn from(number: N) -> FitnessEquipmentState {
        match number.into() {
            0 => FitnessEquipmentState::Ready,
            1 => FitnessEquipmentState::InUse,
            2 => FitnessEquipmentState::Paused,
            3 => FitnessEquipmentState::Unknown,
            n => FitnessEquipmentState::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum Tone {
    Off,
    Tone,
    ToneAndVibrate,
    Vibrate,
    UnknownValue(u64),
}

impl<N> From<N> for Tone
    where N: Into<u64>
{
    fn from(number: N) -> Tone {
        match number.into() {
            0 => Tone::Off,
            1 => Tone::Tone,
            2 => Tone::Vibrate,
            3 => Tone::ToneAndVibrate,
            n => Tone::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum ActivityClass {
    Athlete,
    Level,
    LevelMax,
    UnknownValue(u64),
}

impl<N> From<N> for ActivityClass
    where N: Into<u64>
{
    fn from(number: N) -> ActivityClass {
        match number.into() {
            100 => ActivityClass::LevelMax,
            127 => ActivityClass::Level,
            128 => ActivityClass::Athlete,
            n => ActivityClass::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum HrZoneCalc {
    Custom,
    PercentHrr,
    PercentMaxHr,
    UnknownValue(u64),
}

impl<N> From<N> for HrZoneCalc
    where N: Into<u64>
{
    fn from(number: N) -> HrZoneCalc {
        match number.into() {
            0 => HrZoneCalc::Custom,
            1 => HrZoneCalc::PercentMaxHr,
            2 => HrZoneCalc::PercentHrr,
            n => HrZoneCalc::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum PwrZoneCalc {
    Custom,
    PercentFtp,
    UnknownValue(u64),
}

impl<N> From<N> for PwrZoneCalc
    where N: Into<u64>
{
    fn from(number: N) -> PwrZoneCalc {
        match number.into() {
            0 => PwrZoneCalc::Custom,
            1 => PwrZoneCalc::PercentFtp,
            n => PwrZoneCalc::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum WktStepDuration {
    Calories,
    Distance,
    HrGreaterThan,
    HrLessThan,
    Open,
    Power10SGreaterThan,
    Power10SLessThan,
    Power30SGreaterThan,
    Power30SLessThan,
    Power3SGreaterThan,
    Power3SLessThan,
    PowerGreaterThan,
    PowerLapGreaterThan,
    PowerLapLessThan,
    PowerLessThan,
    RepeatUntilCalories,
    RepeatUntilDistance,
    RepeatUntilHrGreaterThan,
    RepeatUntilHrLessThan,
    RepeatUntilMaxPowerLastLapLessThan,
    RepeatUntilPowerGreaterThan,
    RepeatUntilPowerLastLapLessThan,
    RepeatUntilPowerLessThan,
    RepeatUntilStepsCmplt,
    RepeatUntilTime,
    RepeatUntilTrainingPeaksTss,
    RepetitionTime,
    Reps,
    Time,
    TrainingPeaksTss,
    UnknownValue(u64),
}

impl<N> From<N> for WktStepDuration
    where N: Into<u64>
{
    fn from(number: N) -> WktStepDuration {
        match number.into() {
            0 => WktStepDuration::Time,
            1 => WktStepDuration::Distance,
            2 => WktStepDuration::HrLessThan,
            3 => WktStepDuration::HrGreaterThan,
            4 => WktStepDuration::Calories,
            5 => WktStepDuration::Open,
            6 => WktStepDuration::RepeatUntilStepsCmplt,
            7 => WktStepDuration::RepeatUntilTime,
            8 => WktStepDuration::RepeatUntilDistance,
            9 => WktStepDuration::RepeatUntilCalories,
            10 => WktStepDuration::RepeatUntilHrLessThan,
            11 => WktStepDuration::RepeatUntilHrGreaterThan,
            12 => WktStepDuration::RepeatUntilPowerLessThan,
            13 => WktStepDuration::RepeatUntilPowerGreaterThan,
            14 => WktStepDuration::PowerLessThan,
            15 => WktStepDuration::PowerGreaterThan,
            16 => WktStepDuration::TrainingPeaksTss,
            17 => WktStepDuration::RepeatUntilPowerLastLapLessThan,
            18 => WktStepDuration::RepeatUntilMaxPowerLastLapLessThan,
            19 => WktStepDuration::Power3SLessThan,
            20 => WktStepDuration::Power10SLessThan,
            21 => WktStepDuration::Power30SLessThan,
            22 => WktStepDuration::Power3SGreaterThan,
            23 => WktStepDuration::Power10SGreaterThan,
            24 => WktStepDuration::Power30SGreaterThan,
            25 => WktStepDuration::PowerLapLessThan,
            26 => WktStepDuration::PowerLapGreaterThan,
            27 => WktStepDuration::RepeatUntilTrainingPeaksTss,
            28 => WktStepDuration::RepetitionTime,
            29 => WktStepDuration::Reps,
            n => WktStepDuration::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum WktStepTarget {
    Cadence,
    Grade,
    HeartRate,
    HeartRateLap,
    Open,
    Power,
    Power10S,
    Power30S,
    Power3S,
    PowerLap,
    Resistance,
    Speed,
    SpeedLap,
    SwimStroke,
    UnknownValue(u64),
}

impl<N> From<N> for WktStepTarget
    where N: Into<u64>
{
    fn from(number: N) -> WktStepTarget {
        match number.into() {
            0 => WktStepTarget::Speed,
            1 => WktStepTarget::HeartRate,
            2 => WktStepTarget::Open,
            3 => WktStepTarget::Cadence,
            4 => WktStepTarget::Power,
            5 => WktStepTarget::Grade,
            6 => WktStepTarget::Resistance,
            7 => WktStepTarget::Power3S,
            8 => WktStepTarget::Power10S,
            9 => WktStepTarget::Power30S,
            10 => WktStepTarget::PowerLap,
            11 => WktStepTarget::SwimStroke,
            12 => WktStepTarget::SpeedLap,
            13 => WktStepTarget::HeartRateLap,
            n => WktStepTarget::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum Goal {
    ActiveMinutes,
    Ascent,
    Calories,
    Distance,
    Frequency,
    Steps,
    Time,
    UnknownValue(u64),
}

impl<N> From<N> for Goal
    where N: Into<u64>
{
    fn from(number: N) -> Goal {
        match number.into() {
            0 => Goal::Time,
            1 => Goal::Distance,
            2 => Goal::Calories,
            3 => Goal::Frequency,
            4 => Goal::Steps,
            5 => Goal::Ascent,
            6 => Goal::ActiveMinutes,
            n => Goal::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum GoalRecurrence {
    Custom,
    Daily,
    Monthly,
    Off,
    Weekly,
    Yearly,
    UnknownValue(u64),
}

impl<N> From<N> for GoalRecurrence
    where N: Into<u64>
{
    fn from(number: N) -> GoalRecurrence {
        match number.into() {
            0 => GoalRecurrence::Off,
            1 => GoalRecurrence::Daily,
            2 => GoalRecurrence::Weekly,
            3 => GoalRecurrence::Monthly,
            4 => GoalRecurrence::Yearly,
            5 => GoalRecurrence::Custom,
            n => GoalRecurrence::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum GoalSource {
    Auto,
    Community,
    User,
    UnknownValue(u64),
}

impl<N> From<N> for GoalSource
    where N: Into<u64>
{
    fn from(number: N) -> GoalSource {
        match number.into() {
            0 => GoalSource::Auto,
            1 => GoalSource::Community,
            2 => GoalSource::User,
            n => GoalSource::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
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

impl<N> From<N> for CoursePoint
    where N: Into<u64>
{
    fn from(number: N) -> CoursePoint {
        match number.into() {
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
            n => CoursePoint::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum Manufacturer {
    AAndD,
    AceSensor,
    AcornProjectsAps,
    Actigraphcorp,
    AlatechTechnologyLtd,
    Archinoetics,
    Beurer,
    Bf1Systems,
    Bkool,
    BodyBikeSmart,
    Bontrager,
    Breakaway,
    BrimBrothers,
    Bryton,
    BrytonSensors,
    BsxAthletics,
    CampagnoloSrl,
    Cardiosport,
    Cateye,
    Ciclosport,
    CitizenSystems,
    CleanMobile,
    Cobi,
    Concept2,
    Coros,
    Cosinuss,
    Cycligentinc,
    Cycliq,
    Dabuziduo,
    Dayton,
    Development,
    Dexcom,
    DirectionTechnology,
    DkCity,
    Dynastream,
    DynastreamOem,
    Dynovelo,
    Echowell,
    Elite,
    Evesports,
    FalcoEMotors,
    FaveroElectronics,
    Feedbacksports,
    Fitcare,
    FourIIII,
    Fullspeedahead,
    Garmin,
    GarminFr405Antfs,
    Geonaute,
    GiantManufacturingCo,
    GopherSport,
    Gpulse,
    Hammerhead,
    Healthandlife,
    Hmm,
    Holux,
    Ibike,
    Icg,
    IdBike,
    Idt,
    IforPowell,
    Igpsport,
    Inpeak,
    InsideRideTechnologies,
    Jetblack,
    JohnsonHealthTech,
    Kinetic,
    KineticByKurt,
    LatitudeLimited,
    LemondFitness,
    Lezyne,
    LifeTimeFitness,
    Lifebeam,
    LimitsTechnology,
    Look,
    Luxottica,
    Magellan,
    Magene,
    Magneticdays,
    Magtonic,
    Magura,
    MaxwellGuider,
    Metalogics,
    Metrigear,
    MiPulse,
    Minoura,
    MioMagellan,
    MioTechnologyEurope,
    Moxy,
    Nautilus,
    Navman,
    NciTechnology,
    NielsenKellerman,
    NorthPoleEngineering,
    OctaneFitness,
    Omata,
    OneGiantLeap,
    OnePartCarbon,
    Orangetheory,
    Osynce,
    Peaksware,
    PedalBrain,
    PerceptionDigital,
    Peripedal,
    PhysicalEnterprises,
    Pioneer,
    Podoon,
    PolarElectro,
    Powerbahn,
    Praxisworks,
    Precor,
    Quarq,
    Recon,
    Rotor,
    Salutron,
    Saris,
    Saxonar,
    Scosche,
    ScribeLabs,
    Seesense,
    SeikoEpson,
    SeikoEpsonOem,
    SensitivusGauge,
    Shapelog,
    Sigmasport,
    SoaringTechnology,
    SoundOfMotion,
    Spantec,
    SparkHk,
    Specialized,
    Spivi,
    Sram,
    Srm,
    StagesCycling,
    StarTrac,
    Strava,
    Stryd,
    Suunto,
    Tacx,
    Tanita,
    Technogym,
    TheHurtBox,
    TheSufferfest,
    Thinkrider,
    ThitaElektronik,
    Tigrasport,
    Timex,
    Tomtom,
    TopactionTechnology,
    TrainerRoad,
    Vdo,
    Velosense,
    Virtualtraining,
    Virtugo,
    WahooFitness,
    Waterrower,
    Wattbike,
    Watteam,
    Wellgo,
    Woodway,
    Wtek,
    Xelic,
    Xplova,
    Zephyr,
    Zwift,
    UnknownValue(u64),
}

impl<N> From<N> for Manufacturer
    where N: Into<u64>
{
    fn from(number: N) -> Manufacturer {
        match number.into() {
            1 => Manufacturer::Garmin,
            2 => Manufacturer::GarminFr405Antfs,
            3 => Manufacturer::Zephyr,
            4 => Manufacturer::Dayton,
            5 => Manufacturer::Idt,
            6 => Manufacturer::Srm,
            7 => Manufacturer::Quarq,
            8 => Manufacturer::Ibike,
            9 => Manufacturer::Saris,
            10 => Manufacturer::SparkHk,
            11 => Manufacturer::Tanita,
            12 => Manufacturer::Echowell,
            13 => Manufacturer::DynastreamOem,
            14 => Manufacturer::Nautilus,
            15 => Manufacturer::Dynastream,
            16 => Manufacturer::Timex,
            17 => Manufacturer::Metrigear,
            18 => Manufacturer::Xelic,
            19 => Manufacturer::Beurer,
            20 => Manufacturer::Cardiosport,
            21 => Manufacturer::AAndD,
            22 => Manufacturer::Hmm,
            23 => Manufacturer::Suunto,
            24 => Manufacturer::ThitaElektronik,
            25 => Manufacturer::Gpulse,
            26 => Manufacturer::CleanMobile,
            27 => Manufacturer::PedalBrain,
            28 => Manufacturer::Peaksware,
            29 => Manufacturer::Saxonar,
            30 => Manufacturer::LemondFitness,
            31 => Manufacturer::Dexcom,
            32 => Manufacturer::WahooFitness,
            33 => Manufacturer::OctaneFitness,
            34 => Manufacturer::Archinoetics,
            35 => Manufacturer::TheHurtBox,
            36 => Manufacturer::CitizenSystems,
            37 => Manufacturer::Magellan,
            38 => Manufacturer::Osynce,
            39 => Manufacturer::Holux,
            40 => Manufacturer::Concept2,
            42 => Manufacturer::OneGiantLeap,
            43 => Manufacturer::AceSensor,
            44 => Manufacturer::BrimBrothers,
            45 => Manufacturer::Xplova,
            46 => Manufacturer::PerceptionDigital,
            47 => Manufacturer::Bf1Systems,
            48 => Manufacturer::Pioneer,
            49 => Manufacturer::Spantec,
            50 => Manufacturer::Metalogics,
            51 => Manufacturer::FourIIII,
            52 => Manufacturer::SeikoEpson,
            53 => Manufacturer::SeikoEpsonOem,
            54 => Manufacturer::IforPowell,
            55 => Manufacturer::MaxwellGuider,
            56 => Manufacturer::StarTrac,
            57 => Manufacturer::Breakaway,
            58 => Manufacturer::AlatechTechnologyLtd,
            59 => Manufacturer::MioTechnologyEurope,
            60 => Manufacturer::Rotor,
            61 => Manufacturer::Geonaute,
            62 => Manufacturer::IdBike,
            63 => Manufacturer::Specialized,
            64 => Manufacturer::Wtek,
            65 => Manufacturer::PhysicalEnterprises,
            66 => Manufacturer::NorthPoleEngineering,
            67 => Manufacturer::Bkool,
            68 => Manufacturer::Cateye,
            69 => Manufacturer::StagesCycling,
            70 => Manufacturer::Sigmasport,
            71 => Manufacturer::Tomtom,
            72 => Manufacturer::Peripedal,
            73 => Manufacturer::Wattbike,
            76 => Manufacturer::Moxy,
            77 => Manufacturer::Ciclosport,
            78 => Manufacturer::Powerbahn,
            79 => Manufacturer::AcornProjectsAps,
            80 => Manufacturer::Lifebeam,
            81 => Manufacturer::Bontrager,
            82 => Manufacturer::Wellgo,
            83 => Manufacturer::Scosche,
            84 => Manufacturer::Magura,
            85 => Manufacturer::Woodway,
            86 => Manufacturer::Elite,
            87 => Manufacturer::NielsenKellerman,
            88 => Manufacturer::DkCity,
            89 => Manufacturer::Tacx,
            90 => Manufacturer::DirectionTechnology,
            91 => Manufacturer::Magtonic,
            92 => Manufacturer::OnePartCarbon,
            93 => Manufacturer::InsideRideTechnologies,
            94 => Manufacturer::SoundOfMotion,
            95 => Manufacturer::Stryd,
            96 => Manufacturer::Icg,
            97 => Manufacturer::MiPulse,
            98 => Manufacturer::BsxAthletics,
            99 => Manufacturer::Look,
            100 => Manufacturer::CampagnoloSrl,
            101 => Manufacturer::BodyBikeSmart,
            102 => Manufacturer::Praxisworks,
            103 => Manufacturer::LimitsTechnology,
            104 => Manufacturer::TopactionTechnology,
            105 => Manufacturer::Cosinuss,
            106 => Manufacturer::Fitcare,
            107 => Manufacturer::Magene,
            108 => Manufacturer::GiantManufacturingCo,
            109 => Manufacturer::Tigrasport,
            110 => Manufacturer::Salutron,
            111 => Manufacturer::Technogym,
            112 => Manufacturer::BrytonSensors,
            113 => Manufacturer::LatitudeLimited,
            114 => Manufacturer::SoaringTechnology,
            115 => Manufacturer::Igpsport,
            116 => Manufacturer::Thinkrider,
            117 => Manufacturer::GopherSport,
            118 => Manufacturer::Waterrower,
            119 => Manufacturer::Orangetheory,
            120 => Manufacturer::Inpeak,
            121 => Manufacturer::Kinetic,
            122 => Manufacturer::JohnsonHealthTech,
            123 => Manufacturer::PolarElectro,
            124 => Manufacturer::Seesense,
            125 => Manufacturer::NciTechnology,
            255 => Manufacturer::Development,
            257 => Manufacturer::Healthandlife,
            258 => Manufacturer::Lezyne,
            259 => Manufacturer::ScribeLabs,
            260 => Manufacturer::Zwift,
            261 => Manufacturer::Watteam,
            262 => Manufacturer::Recon,
            263 => Manufacturer::FaveroElectronics,
            264 => Manufacturer::Dynovelo,
            265 => Manufacturer::Strava,
            266 => Manufacturer::Precor,
            267 => Manufacturer::Bryton,
            268 => Manufacturer::Sram,
            269 => Manufacturer::Navman,
            270 => Manufacturer::Cobi,
            271 => Manufacturer::Spivi,
            272 => Manufacturer::MioMagellan,
            273 => Manufacturer::Evesports,
            274 => Manufacturer::SensitivusGauge,
            275 => Manufacturer::Podoon,
            276 => Manufacturer::LifeTimeFitness,
            277 => Manufacturer::FalcoEMotors,
            278 => Manufacturer::Minoura,
            279 => Manufacturer::Cycliq,
            280 => Manufacturer::Luxottica,
            281 => Manufacturer::TrainerRoad,
            282 => Manufacturer::TheSufferfest,
            283 => Manufacturer::Fullspeedahead,
            284 => Manufacturer::Virtualtraining,
            285 => Manufacturer::Feedbacksports,
            286 => Manufacturer::Omata,
            287 => Manufacturer::Vdo,
            288 => Manufacturer::Magneticdays,
            289 => Manufacturer::Hammerhead,
            290 => Manufacturer::KineticByKurt,
            291 => Manufacturer::Shapelog,
            292 => Manufacturer::Dabuziduo,
            293 => Manufacturer::Jetblack,
            294 => Manufacturer::Coros,
            295 => Manufacturer::Virtugo,
            296 => Manufacturer::Velosense,
            297 => Manufacturer::Cycligentinc,
            5759 => Manufacturer::Actigraphcorp,
            n => Manufacturer::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum GarminProduct {
    Alf04,
    Amx,
    AndroidAntplusPlugin,
    ApproachS20,
    ApproachS6,
    Axb01,
    Axb02,
    Axh01,
    Axs01,
    Bcm,
    Bsm,
    Chirp,
    Connect,
    ConnectiqSimulator,
    D2Bravo,
    D2BravoTitanium,
    DsiAlf01,
    DsiAlf02,
    Edge1000,
    Edge1000China,
    Edge1000Japan,
    Edge1000Korea,
    Edge1000Taiwan,
    Edge20,
    Edge200,
    Edge200Taiwan,
    Edge25,
    Edge500,
    Edge500China,
    Edge500Japan,
    Edge500Korea,
    Edge500Taiwan,
    Edge510,
    Edge510Asia,
    Edge510Japan,
    Edge510Korea,
    Edge520,
    Edge800,
    Edge800China,
    Edge800Japan,
    Edge800Korea,
    Edge800Taiwan,
    Edge810,
    Edge810China,
    Edge810Japan,
    Edge810Taiwan,
    Edge820,
    EdgeExplore1000,
    EdgeExplore820,
    EdgeRemote,
    EdgeTouring,
    Epix,
    EtrexTouch,
    Fenix,
    Fenix2,
    Fenix3,
    Fenix3China,
    Fenix3Chronos,
    Fenix3Hr,
    Fenix3Twn,
    Fenix5,
    Fenix5S,
    Fenix5X,
    Fr10,
    Fr10Japan,
    Fr110,
    Fr110Japan,
    Fr15,
    Fr15Japan,
    Fr210Japan,
    Fr220,
    Fr220China,
    Fr220Japan,
    Fr220Russia,
    Fr220Taiwan,
    Fr225,
    Fr225Asia,
    Fr225SingleByteProductId,
    Fr230,
    Fr235,
    Fr25,
    Fr301China,
    Fr301Japan,
    Fr301Korea,
    Fr301Taiwan,
    Fr310Xt,
    Fr310Xt4T,
    Fr405,
    Fr405Japan,
    Fr50,
    Fr60,
    Fr610,
    Fr610Japan,
    Fr620,
    Fr620China,
    Fr620Japan,
    Fr620Russia,
    Fr620Taiwan,
    Fr630,
    Fr70,
    Fr910Xt,
    Fr910XtChina,
    Fr910XtJapan,
    Fr910XtKorea,
    Fr920Xt,
    Fr920XtChina,
    Fr920XtJapan,
    Fr920XtTaiwan,
    Fr935,
    Hrm1,
    Hrm2Ss,
    Hrm3Ss,
    Hrm4Run,
    HrmRun,
    HrmRunSingleByteProductId,
    HrmTri,
    HrmTriSingleByteProductId,
    IndexSmartScale,
    Nautix,
    Oregon7Xx,
    Rino7Xx,
    RunningDynamicsPod,
    Sdm4,
    Swim,
    Tempe,
    TrainingCenter,
    Truswing,
    VariaHeadlight,
    VariaRadarDisplay,
    VariaRadarTaillight,
    VariaRemote,
    VariaTaillightOld,
    VariaUt800,
    VariaVision,
    Vector2,
    Vector2S,
    VectorCp,
    VectorS,
    VectorSs,
    VirbElite,
    VirbRemote,
    VirbUltra30,
    Virbx,
    Virbxe,
    VivoActive,
    VivoActiveApac,
    VivoActiveHr,
    VivoFit,
    VivoFit2,
    VivoFit3,
    VivoFitJr,
    VivoKi,
    VivoMove,
    VivoSmart,
    VivoSmartApac,
    VivoSmartGpsHr,
    VivoSmartHr,
    UnknownValue(u64),
}

impl<N> From<N> for GarminProduct
    where N: Into<u64>
{
    fn from(number: N) -> GarminProduct {
        match number.into() {
            1 => GarminProduct::Hrm1,
            2 => GarminProduct::Axh01,
            3 => GarminProduct::Axb01,
            4 => GarminProduct::Axb02,
            5 => GarminProduct::Hrm2Ss,
            6 => GarminProduct::DsiAlf02,
            7 => GarminProduct::Hrm3Ss,
            8 => GarminProduct::HrmRunSingleByteProductId,
            9 => GarminProduct::Bsm,
            10 => GarminProduct::Bcm,
            11 => GarminProduct::Axs01,
            12 => GarminProduct::HrmTriSingleByteProductId,
            14 => GarminProduct::Fr225SingleByteProductId,
            473 => GarminProduct::Fr301China,
            474 => GarminProduct::Fr301Japan,
            475 => GarminProduct::Fr301Korea,
            494 => GarminProduct::Fr301Taiwan,
            717 => GarminProduct::Fr405,
            782 => GarminProduct::Fr50,
            987 => GarminProduct::Fr405Japan,
            988 => GarminProduct::Fr60,
            1011 => GarminProduct::DsiAlf01,
            1018 => GarminProduct::Fr310Xt,
            1036 => GarminProduct::Edge500,
            1124 => GarminProduct::Fr110,
            1169 => GarminProduct::Edge800,
            1199 => GarminProduct::Edge500Taiwan,
            1213 => GarminProduct::Edge500Japan,
            1253 => GarminProduct::Chirp,
            1274 => GarminProduct::Fr110Japan,
            1325 => GarminProduct::Edge200,
            1328 => GarminProduct::Fr910Xt,
            1333 => GarminProduct::Edge800Taiwan,
            1334 => GarminProduct::Edge800Japan,
            1341 => GarminProduct::Alf04,
            1345 => GarminProduct::Fr610,
            1360 => GarminProduct::Fr210Japan,
            1380 => GarminProduct::VectorSs,
            1381 => GarminProduct::VectorCp,
            1386 => GarminProduct::Edge800China,
            1387 => GarminProduct::Edge500China,
            1410 => GarminProduct::Fr610Japan,
            1422 => GarminProduct::Edge500Korea,
            1436 => GarminProduct::Fr70,
            1446 => GarminProduct::Fr310Xt4T,
            1461 => GarminProduct::Amx,
            1482 => GarminProduct::Fr10,
            1497 => GarminProduct::Edge800Korea,
            1499 => GarminProduct::Swim,
            1537 => GarminProduct::Fr910XtChina,
            1551 => GarminProduct::Fenix,
            1555 => GarminProduct::Edge200Taiwan,
            1561 => GarminProduct::Edge510,
            1567 => GarminProduct::Edge810,
            1570 => GarminProduct::Tempe,
            1600 => GarminProduct::Fr910XtJapan,
            1623 => GarminProduct::Fr620,
            1632 => GarminProduct::Fr220,
            1664 => GarminProduct::Fr910XtKorea,
            1688 => GarminProduct::Fr10Japan,
            1721 => GarminProduct::Edge810Japan,
            1735 => GarminProduct::VirbElite,
            1736 => GarminProduct::EdgeTouring,
            1742 => GarminProduct::Edge510Japan,
            1743 => GarminProduct::HrmTri,
            1752 => GarminProduct::HrmRun,
            1765 => GarminProduct::Fr920Xt,
            1821 => GarminProduct::Edge510Asia,
            1822 => GarminProduct::Edge810China,
            1823 => GarminProduct::Edge810Taiwan,
            1836 => GarminProduct::Edge1000,
            1837 => GarminProduct::VivoFit,
            1853 => GarminProduct::VirbRemote,
            1885 => GarminProduct::VivoKi,
            1903 => GarminProduct::Fr15,
            1907 => GarminProduct::VivoActive,
            1918 => GarminProduct::Edge510Korea,
            1928 => GarminProduct::Fr620Japan,
            1929 => GarminProduct::Fr620China,
            1930 => GarminProduct::Fr220Japan,
            1931 => GarminProduct::Fr220China,
            1936 => GarminProduct::ApproachS6,
            1956 => GarminProduct::VivoSmart,
            1967 => GarminProduct::Fenix2,
            1988 => GarminProduct::Epix,
            2050 => GarminProduct::Fenix3,
            2052 => GarminProduct::Edge1000Taiwan,
            2053 => GarminProduct::Edge1000Japan,
            2061 => GarminProduct::Fr15Japan,
            2067 => GarminProduct::Edge520,
            2070 => GarminProduct::Edge1000China,
            2072 => GarminProduct::Fr620Russia,
            2073 => GarminProduct::Fr220Russia,
            2079 => GarminProduct::VectorS,
            2100 => GarminProduct::Edge1000Korea,
            2130 => GarminProduct::Fr920XtTaiwan,
            2131 => GarminProduct::Fr920XtChina,
            2132 => GarminProduct::Fr920XtJapan,
            2134 => GarminProduct::Virbx,
            2135 => GarminProduct::VivoSmartApac,
            2140 => GarminProduct::EtrexTouch,
            2147 => GarminProduct::Edge25,
            2148 => GarminProduct::Fr25,
            2150 => GarminProduct::VivoFit2,
            2153 => GarminProduct::Fr225,
            2156 => GarminProduct::Fr630,
            2157 => GarminProduct::Fr230,
            2160 => GarminProduct::VivoActiveApac,
            2161 => GarminProduct::Vector2,
            2162 => GarminProduct::Vector2S,
            2172 => GarminProduct::Virbxe,
            2173 => GarminProduct::Fr620Taiwan,
            2174 => GarminProduct::Fr220Taiwan,
            2175 => GarminProduct::Truswing,
            2188 => GarminProduct::Fenix3China,
            2189 => GarminProduct::Fenix3Twn,
            2192 => GarminProduct::VariaHeadlight,
            2193 => GarminProduct::VariaTaillightOld,
            2204 => GarminProduct::EdgeExplore1000,
            2219 => GarminProduct::Fr225Asia,
            2225 => GarminProduct::VariaRadarTaillight,
            2226 => GarminProduct::VariaRadarDisplay,
            2238 => GarminProduct::Edge20,
            2262 => GarminProduct::D2Bravo,
            2266 => GarminProduct::ApproachS20,
            2276 => GarminProduct::VariaRemote,
            2327 => GarminProduct::Hrm4Run,
            2337 => GarminProduct::VivoActiveHr,
            2347 => GarminProduct::VivoSmartGpsHr,
            2348 => GarminProduct::VivoSmartHr,
            2368 => GarminProduct::VivoMove,
            2398 => GarminProduct::VariaVision,
            2406 => GarminProduct::VivoFit3,
            2413 => GarminProduct::Fenix3Hr,
            2417 => GarminProduct::VirbUltra30,
            2429 => GarminProduct::IndexSmartScale,
            2431 => GarminProduct::Fr235,
            2432 => GarminProduct::Fenix3Chronos,
            2441 => GarminProduct::Oregon7Xx,
            2444 => GarminProduct::Rino7Xx,
            2496 => GarminProduct::Nautix,
            2530 => GarminProduct::Edge820,
            2531 => GarminProduct::EdgeExplore820,
            2544 => GarminProduct::Fenix5S,
            2547 => GarminProduct::D2BravoTitanium,
            2567 => GarminProduct::VariaUt800,
            2593 => GarminProduct::RunningDynamicsPod,
            2604 => GarminProduct::Fenix5X,
            2606 => GarminProduct::VivoFitJr,
            2691 => GarminProduct::Fr935,
            2697 => GarminProduct::Fenix5,
            10007 => GarminProduct::Sdm4,
            10014 => GarminProduct::EdgeRemote,
            20119 => GarminProduct::TrainingCenter,
            65531 => GarminProduct::ConnectiqSimulator,
            65532 => GarminProduct::AndroidAntplusPlugin,
            65534 => GarminProduct::Connect,
            n => GarminProduct::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum AntplusDeviceType {
    Antfs,
    BikeAero,
    BikeCadence,
    BikeLightMain,
    BikeLightShared,
    BikePower,
    BikeRadar,
    BikeSpeed,
    BikeSpeedCadence,
    BloodPressure,
    Control,
    ControlHub,
    EnvSensor,
    EnvironmentSensorLegacy,
    Exd,
    FitnessEquipment,
    GeocacheNode,
    HeartRate,
    LightElectricVehicle,
    MultiSportSpeedDistance,
    MuscleOxygen,
    Racquet,
    StrideSpeedDistance,
    WeightScale,
    UnknownValue(u64),
}

impl<N> From<N> for AntplusDeviceType
    where N: Into<u64>
{
    fn from(number: N) -> AntplusDeviceType {
        match number.into() {
            1 => AntplusDeviceType::Antfs,
            11 => AntplusDeviceType::BikePower,
            12 => AntplusDeviceType::EnvironmentSensorLegacy,
            15 => AntplusDeviceType::MultiSportSpeedDistance,
            16 => AntplusDeviceType::Control,
            17 => AntplusDeviceType::FitnessEquipment,
            18 => AntplusDeviceType::BloodPressure,
            19 => AntplusDeviceType::GeocacheNode,
            20 => AntplusDeviceType::LightElectricVehicle,
            25 => AntplusDeviceType::EnvSensor,
            26 => AntplusDeviceType::Racquet,
            27 => AntplusDeviceType::ControlHub,
            31 => AntplusDeviceType::MuscleOxygen,
            35 => AntplusDeviceType::BikeLightMain,
            36 => AntplusDeviceType::BikeLightShared,
            38 => AntplusDeviceType::Exd,
            40 => AntplusDeviceType::BikeRadar,
            46 => AntplusDeviceType::BikeAero,
            119 => AntplusDeviceType::WeightScale,
            120 => AntplusDeviceType::HeartRate,
            121 => AntplusDeviceType::BikeSpeedCadence,
            122 => AntplusDeviceType::BikeCadence,
            123 => AntplusDeviceType::BikeSpeed,
            124 => AntplusDeviceType::StrideSpeedDistance,
            n => AntplusDeviceType::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum AntNetwork {
    Antfs,
    Antplus,
    Private,
    Public,
    UnknownValue(u64),
}

impl<N> From<N> for AntNetwork
    where N: Into<u64>
{
    fn from(number: N) -> AntNetwork {
        match number.into() {
            0 => AntNetwork::Public,
            1 => AntNetwork::Antplus,
            2 => AntNetwork::Antfs,
            3 => AntNetwork::Private,
            n => AntNetwork::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum WorkoutCapabilities {
    Cadence,
    Custom,
    Distance,
    Firstbeat,
    FitnessEquipment,
    Grade,
    HeartRate,
    Interval,
    NewLeaf,
    Power,
    Protected,
    Resistance,
    Speed,
    Tcx,
    UnknownValue(u64),
}

impl<N> From<N> for WorkoutCapabilities
    where N: Into<u64>
{
    fn from(number: N) -> WorkoutCapabilities {
        match number.into() {
            1 => WorkoutCapabilities::Interval,
            2 => WorkoutCapabilities::Custom,
            4 => WorkoutCapabilities::FitnessEquipment,
            8 => WorkoutCapabilities::Firstbeat,
            16 => WorkoutCapabilities::NewLeaf,
            32 => WorkoutCapabilities::Tcx,
            128 => WorkoutCapabilities::Speed,
            256 => WorkoutCapabilities::HeartRate,
            512 => WorkoutCapabilities::Distance,
            1024 => WorkoutCapabilities::Cadence,
            2048 => WorkoutCapabilities::Power,
            4096 => WorkoutCapabilities::Grade,
            8192 => WorkoutCapabilities::Resistance,
            16384 => WorkoutCapabilities::Protected,
            n => WorkoutCapabilities::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum BatteryStatus {
    Charging,
    Critical,
    Good,
    Low,
    New,
    Ok,
    Unknown,
    UnknownValue(u64),
}

impl<N> From<N> for BatteryStatus
    where N: Into<u64>
{
    fn from(number: N) -> BatteryStatus {
        match number.into() {
            1 => BatteryStatus::New,
            2 => BatteryStatus::Good,
            3 => BatteryStatus::Ok,
            4 => BatteryStatus::Low,
            5 => BatteryStatus::Critical,
            6 => BatteryStatus::Charging,
            7 => BatteryStatus::Unknown,
            n => BatteryStatus::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum HrType {
    Irregular,
    Normal,
    UnknownValue(u64),
}

impl<N> From<N> for HrType
    where N: Into<u64>
{
    fn from(number: N) -> HrType {
        match number.into() {
            0 => HrType::Normal,
            1 => HrType::Irregular,
            n => HrType::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum CourseCapabilities {
    Bikeway,
    Cadence,
    Distance,
    HeartRate,
    Navigation,
    Position,
    Power,
    Processed,
    Time,
    Training,
    Valid,
    UnknownValue(u64),
}

impl<N> From<N> for CourseCapabilities
    where N: Into<u64>
{
    fn from(number: N) -> CourseCapabilities {
        match number.into() {
            1 => CourseCapabilities::Processed,
            2 => CourseCapabilities::Valid,
            4 => CourseCapabilities::Time,
            8 => CourseCapabilities::Distance,
            16 => CourseCapabilities::Position,
            32 => CourseCapabilities::HeartRate,
            64 => CourseCapabilities::Power,
            128 => CourseCapabilities::Cadence,
            256 => CourseCapabilities::Training,
            512 => CourseCapabilities::Navigation,
            1024 => CourseCapabilities::Bikeway,
            n => CourseCapabilities::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum Weight {
    Calculating,
    UnknownValue(u64),
}

impl<N> From<N> for Weight
    where N: Into<u64>
{
    fn from(number: N) -> Weight {
        match number.into() {
            65534 => Weight::Calculating,
            n => Weight::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum WorkoutHr {
    BpmOffset,
    UnknownValue(u64),
}

impl<N> From<N> for WorkoutHr
    where N: Into<u64>
{
    fn from(number: N) -> WorkoutHr {
        match number.into() {
            100 => WorkoutHr::BpmOffset,
            n => WorkoutHr::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum WorkoutPower {
    WattsOffset,
    UnknownValue(u64),
}

impl<N> From<N> for WorkoutPower
    where N: Into<u64>
{
    fn from(number: N) -> WorkoutPower {
        match number.into() {
            1000 => WorkoutPower::WattsOffset,
            n => WorkoutPower::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum BpStatus {
    ErrorDataOutOfRange,
    ErrorIncompleteData,
    ErrorIrregularHeartRate,
    ErrorNoMeasurement,
    NoError,
    UnknownValue(u64),
}

impl<N> From<N> for BpStatus
    where N: Into<u64>
{
    fn from(number: N) -> BpStatus {
        match number.into() {
            0 => BpStatus::NoError,
            1 => BpStatus::ErrorIncompleteData,
            2 => BpStatus::ErrorNoMeasurement,
            3 => BpStatus::ErrorDataOutOfRange,
            4 => BpStatus::ErrorIrregularHeartRate,
            n => BpStatus::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum UserLocalId {
    LocalMax,
    LocalMin,
    PortableMax,
    PortableMin,
    StationaryMax,
    StationaryMin,
    UnknownValue(u64),
}

impl<N> From<N> for UserLocalId
    where N: Into<u64>
{
    fn from(number: N) -> UserLocalId {
        match number.into() {
            0 => UserLocalId::LocalMin,
            15 => UserLocalId::LocalMax,
            16 => UserLocalId::StationaryMin,
            255 => UserLocalId::StationaryMax,
            256 => UserLocalId::PortableMin,
            65534 => UserLocalId::PortableMax,
            n => UserLocalId::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum SwimStroke {
    Backstroke,
    Breaststroke,
    Butterfly,
    Drill,
    Freestyle,
    Im,
    Mixed,
    UnknownValue(u64),
}

impl<N> From<N> for SwimStroke
    where N: Into<u64>
{
    fn from(number: N) -> SwimStroke {
        match number.into() {
            0 => SwimStroke::Freestyle,
            1 => SwimStroke::Backstroke,
            2 => SwimStroke::Breaststroke,
            3 => SwimStroke::Butterfly,
            4 => SwimStroke::Drill,
            5 => SwimStroke::Mixed,
            6 => SwimStroke::Im,
            n => SwimStroke::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum ActivityType {
    All,
    Cycling,
    FitnessEquipment,
    Generic,
    Running,
    Sedentary,
    Swimming,
    Transition,
    Walking,
    UnknownValue(u64),
}

impl<N> From<N> for ActivityType
    where N: Into<u64>
{
    fn from(number: N) -> ActivityType {
        match number.into() {
            0 => ActivityType::Generic,
            1 => ActivityType::Running,
            2 => ActivityType::Cycling,
            3 => ActivityType::Transition,
            4 => ActivityType::FitnessEquipment,
            5 => ActivityType::Swimming,
            6 => ActivityType::Walking,
            8 => ActivityType::Sedentary,
            254 => ActivityType::All,
            n => ActivityType::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum ActivitySubtype {
    All,
    Cyclocross,
    Downhill,
    Elliptical,
    Generic,
    HandCycling,
    IndoorCycling,
    IndoorRowing,
    LapSwimming,
    Mountain,
    OpenWater,
    Recumbent,
    Road,
    Spin,
    StairClimbing,
    Street,
    Track,
    TrackCycling,
    Trail,
    Treadmill,
    UnknownValue(u64),
}

impl<N> From<N> for ActivitySubtype
    where N: Into<u64>
{
    fn from(number: N) -> ActivitySubtype {
        match number.into() {
            0 => ActivitySubtype::Generic,
            1 => ActivitySubtype::Treadmill,
            2 => ActivitySubtype::Street,
            3 => ActivitySubtype::Trail,
            4 => ActivitySubtype::Track,
            5 => ActivitySubtype::Spin,
            6 => ActivitySubtype::IndoorCycling,
            7 => ActivitySubtype::Road,
            8 => ActivitySubtype::Mountain,
            9 => ActivitySubtype::Downhill,
            10 => ActivitySubtype::Recumbent,
            11 => ActivitySubtype::Cyclocross,
            12 => ActivitySubtype::HandCycling,
            13 => ActivitySubtype::TrackCycling,
            14 => ActivitySubtype::IndoorRowing,
            15 => ActivitySubtype::Elliptical,
            16 => ActivitySubtype::StairClimbing,
            17 => ActivitySubtype::LapSwimming,
            18 => ActivitySubtype::OpenWater,
            254 => ActivitySubtype::All,
            n => ActivitySubtype::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum ActivityLevel {
    High,
    Low,
    Medium,
    UnknownValue(u64),
}

impl<N> From<N> for ActivityLevel
    where N: Into<u64>
{
    fn from(number: N) -> ActivityLevel {
        match number.into() {
            0 => ActivityLevel::Low,
            1 => ActivityLevel::Medium,
            2 => ActivityLevel::High,
            n => ActivityLevel::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum Side {
    Left,
    Right,
    UnknownValue(u64),
}

impl<N> From<N> for Side
    where N: Into<u64>
{
    fn from(number: N) -> Side {
        match number.into() {
            0 => Side::Right,
            1 => Side::Left,
            n => Side::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum LeftRightBalance100 {
    Mask,
    Right,
    UnknownValue(u64),
}

impl<N> From<N> for LeftRightBalance100
    where N: Into<u64>
{
    fn from(number: N) -> LeftRightBalance100 {
        match number.into() {
            16383 => LeftRightBalance100::Mask,
            32768 => LeftRightBalance100::Right,
            n => LeftRightBalance100::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum LengthType {
    Active,
    Idle,
    UnknownValue(u64),
}

impl<N> From<N> for LengthType
    where N: Into<u64>
{
    fn from(number: N) -> LengthType {
        match number.into() {
            0 => LengthType::Idle,
            1 => LengthType::Active,
            n => LengthType::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum DayOfWeek {
    Friday,
    Monday,
    Saturday,
    Sunday,
    Thursday,
    Tuesday,
    Wednesday,
    UnknownValue(u64),
}

impl<N> From<N> for DayOfWeek
    where N: Into<u64>
{
    fn from(number: N) -> DayOfWeek {
        match number.into() {
            0 => DayOfWeek::Sunday,
            1 => DayOfWeek::Monday,
            2 => DayOfWeek::Tuesday,
            3 => DayOfWeek::Wednesday,
            4 => DayOfWeek::Thursday,
            5 => DayOfWeek::Friday,
            6 => DayOfWeek::Saturday,
            n => DayOfWeek::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum WeatherReport {
    Current,
    DailyForecast,
    HourlyForecast,
    UnknownValue(u64),
}

impl<N> From<N> for WeatherReport
    where N: Into<u64>
{
    fn from(number: N) -> WeatherReport {
        match number.into() {
            0 => WeatherReport::Current,
            1 => WeatherReport::HourlyForecast,
            2 => WeatherReport::DailyForecast,
            n => WeatherReport::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum WeatherStatus {
    Clear,
    Cloudy,
    Fog,
    Hail,
    Hazy,
    HeavyRain,
    HeavyRainSnow,
    HeavySnow,
    LightRain,
    LightRainSnow,
    LightSnow,
    MostlyCloudy,
    PartlyCloudy,
    Rain,
    ScatteredShowers,
    ScatteredThunderstorms,
    Snow,
    Thunderstorms,
    UnknownPrecipitation,
    Windy,
    WintryMix,
    UnknownValue(u64),
}

impl<N> From<N> for WeatherStatus
    where N: Into<u64>
{
    fn from(number: N) -> WeatherStatus {
        match number.into() {
            0 => WeatherStatus::Clear,
            1 => WeatherStatus::PartlyCloudy,
            2 => WeatherStatus::MostlyCloudy,
            3 => WeatherStatus::Rain,
            4 => WeatherStatus::Snow,
            5 => WeatherStatus::Windy,
            6 => WeatherStatus::Thunderstorms,
            7 => WeatherStatus::WintryMix,
            8 => WeatherStatus::Fog,
            11 => WeatherStatus::Hazy,
            12 => WeatherStatus::Hail,
            13 => WeatherStatus::ScatteredShowers,
            14 => WeatherStatus::ScatteredThunderstorms,
            15 => WeatherStatus::UnknownPrecipitation,
            16 => WeatherStatus::LightRain,
            17 => WeatherStatus::HeavyRain,
            18 => WeatherStatus::LightSnow,
            19 => WeatherStatus::HeavySnow,
            20 => WeatherStatus::LightRainSnow,
            21 => WeatherStatus::HeavyRainSnow,
            22 => WeatherStatus::Cloudy,
            n => WeatherStatus::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum WeatherSeverity {
    Advisory,
    Statement,
    Unknown,
    Warning,
    Watch,
    UnknownValue(u64),
}

impl<N> From<N> for WeatherSeverity
    where N: Into<u64>
{
    fn from(number: N) -> WeatherSeverity {
        match number.into() {
            0 => WeatherSeverity::Unknown,
            1 => WeatherSeverity::Warning,
            2 => WeatherSeverity::Watch,
            3 => WeatherSeverity::Advisory,
            4 => WeatherSeverity::Statement,
            n => WeatherSeverity::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum WeatherSevereType {
    AirQuality,
    AirStagnation,
    ArcticOutflow,
    ArealFlood,
    Ashfall,
    Avalanche,
    Blizzard,
    BlowingDust,
    BlowingSnow,
    BriskWind,
    CoastalFlood,
    ColdWave,
    DebrisFlow,
    DenseFog,
    DenseSmoke,
    DustStorm,
    ExcessiveHeat,
    ExtremeCold,
    ExtremeWind,
    FireWeather,
    FlashFlood,
    FlashFreeze,
    Flood,
    Freeze,
    FreezingDrizzle,
    FreezingFog,
    FreezingRain,
    FreezingSpray,
    Frost,
    Gale,
    HardFreeze,
    HazardousSeas,
    Heat,
    HeavyFreezingSpray,
    HeavySnowAlert,
    HighHeatAndHumidity,
    HighSurf,
    HighWaterLevel,
    HighWind,
    Humidex,
    HumidexAndHealth,
    Hurricane,
    HurricaneForceWind,
    Hydrological,
    IceStorm,
    InlandHurricane,
    InlandTropicalStorm,
    LakeEffectBlowingSnow,
    LakeEffectSnow,
    LakeWind,
    LakeshoreFlood,
    LesSuetesWind,
    LowWater,
    MarineWeather,
    Rainfall,
    RipTide,
    SevereThunderstorm,
    Sleet,
    SmallCraft,
    SmallCraftHazardousSeas,
    SmallCraftRoughBar,
    SmallCraftWinds,
    Smog,
    SnowAlert,
    SnowAndBlowingSnow,
    SnowSquall,
    Snowfall,
    SpecialMarine,
    SpecialWeather,
    Squall,
    Storm,
    StormSurge,
    StrongWind,
    Tornado,
    TropicalStorm,
    Tsunami,
    Typhoon,
    Unspecified,
    Waterspout,
    Weather,
    Wind,
    WindChill,
    WinterStorm,
    WinterWeather,
    WreckhouseWinds,
    UnknownValue(u64),
}

impl<N> From<N> for WeatherSevereType
    where N: Into<u64>
{
    fn from(number: N) -> WeatherSevereType {
        match number.into() {
            0 => WeatherSevereType::Unspecified,
            1 => WeatherSevereType::Tornado,
            2 => WeatherSevereType::Tsunami,
            3 => WeatherSevereType::Hurricane,
            4 => WeatherSevereType::ExtremeWind,
            5 => WeatherSevereType::Typhoon,
            6 => WeatherSevereType::InlandHurricane,
            7 => WeatherSevereType::HurricaneForceWind,
            8 => WeatherSevereType::Waterspout,
            9 => WeatherSevereType::SevereThunderstorm,
            10 => WeatherSevereType::WreckhouseWinds,
            11 => WeatherSevereType::LesSuetesWind,
            12 => WeatherSevereType::Avalanche,
            13 => WeatherSevereType::FlashFlood,
            14 => WeatherSevereType::TropicalStorm,
            15 => WeatherSevereType::InlandTropicalStorm,
            16 => WeatherSevereType::Blizzard,
            17 => WeatherSevereType::IceStorm,
            18 => WeatherSevereType::FreezingRain,
            19 => WeatherSevereType::DebrisFlow,
            20 => WeatherSevereType::FlashFreeze,
            21 => WeatherSevereType::DustStorm,
            22 => WeatherSevereType::HighWind,
            23 => WeatherSevereType::WinterStorm,
            24 => WeatherSevereType::HeavyFreezingSpray,
            25 => WeatherSevereType::ExtremeCold,
            26 => WeatherSevereType::WindChill,
            27 => WeatherSevereType::ColdWave,
            28 => WeatherSevereType::HeavySnowAlert,
            29 => WeatherSevereType::LakeEffectBlowingSnow,
            30 => WeatherSevereType::SnowSquall,
            31 => WeatherSevereType::LakeEffectSnow,
            32 => WeatherSevereType::WinterWeather,
            33 => WeatherSevereType::Sleet,
            34 => WeatherSevereType::Snowfall,
            35 => WeatherSevereType::SnowAndBlowingSnow,
            36 => WeatherSevereType::BlowingSnow,
            37 => WeatherSevereType::SnowAlert,
            38 => WeatherSevereType::ArcticOutflow,
            39 => WeatherSevereType::FreezingDrizzle,
            40 => WeatherSevereType::Storm,
            41 => WeatherSevereType::StormSurge,
            42 => WeatherSevereType::Rainfall,
            43 => WeatherSevereType::ArealFlood,
            44 => WeatherSevereType::CoastalFlood,
            45 => WeatherSevereType::LakeshoreFlood,
            46 => WeatherSevereType::ExcessiveHeat,
            47 => WeatherSevereType::Heat,
            48 => WeatherSevereType::Weather,
            49 => WeatherSevereType::HighHeatAndHumidity,
            50 => WeatherSevereType::HumidexAndHealth,
            51 => WeatherSevereType::Humidex,
            52 => WeatherSevereType::Gale,
            53 => WeatherSevereType::FreezingSpray,
            54 => WeatherSevereType::SpecialMarine,
            55 => WeatherSevereType::Squall,
            56 => WeatherSevereType::StrongWind,
            57 => WeatherSevereType::LakeWind,
            58 => WeatherSevereType::MarineWeather,
            59 => WeatherSevereType::Wind,
            60 => WeatherSevereType::SmallCraftHazardousSeas,
            61 => WeatherSevereType::HazardousSeas,
            62 => WeatherSevereType::SmallCraft,
            63 => WeatherSevereType::SmallCraftWinds,
            64 => WeatherSevereType::SmallCraftRoughBar,
            65 => WeatherSevereType::HighWaterLevel,
            66 => WeatherSevereType::Ashfall,
            67 => WeatherSevereType::FreezingFog,
            68 => WeatherSevereType::DenseFog,
            69 => WeatherSevereType::DenseSmoke,
            70 => WeatherSevereType::BlowingDust,
            71 => WeatherSevereType::HardFreeze,
            72 => WeatherSevereType::Freeze,
            73 => WeatherSevereType::Frost,
            74 => WeatherSevereType::FireWeather,
            75 => WeatherSevereType::Flood,
            76 => WeatherSevereType::RipTide,
            77 => WeatherSevereType::HighSurf,
            78 => WeatherSevereType::Smog,
            79 => WeatherSevereType::AirQuality,
            80 => WeatherSevereType::BriskWind,
            81 => WeatherSevereType::AirStagnation,
            82 => WeatherSevereType::LowWater,
            83 => WeatherSevereType::Hydrological,
            84 => WeatherSevereType::SpecialWeather,
            n => WeatherSevereType::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum TimeIntoDay {
    UnknownValue(u64),
}

impl<N> From<N> for TimeIntoDay
    where N: Into<u64>
{
    fn from(number: N) -> TimeIntoDay {
        match number.into() {
            n => TimeIntoDay::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum LocaltimeIntoDay {
    UnknownValue(u64),
}

impl<N> From<N> for LocaltimeIntoDay
    where N: Into<u64>
{
    fn from(number: N) -> LocaltimeIntoDay {
        match number.into() {
            n => LocaltimeIntoDay::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum BodyLocation {
    LeftAbdomen,
    LeftArm,
    LeftBicep,
    LeftBrachioradialis,
    LeftCalf,
    LeftChest,
    LeftForearmExtensors,
    LeftGlute,
    LeftHamstring,
    LeftLeg,
    LeftLowerBack,
    LeftQuad,
    LeftShin,
    LeftShoulder,
    LeftTricep,
    LeftUpperBack,
    Neck,
    RightAbdomen,
    RightArm,
    RightBicep,
    RightBrachioradialis,
    RightCalf,
    RightChest,
    RightForearmExtensors,
    RightGlute,
    RightHamstring,
    RightLeg,
    RightLowerBack,
    RightQuad,
    RightShin,
    RightShoulder,
    RightTricep,
    RightUpperBack,
    Throat,
    TorsoBack,
    TorsoFront,
    WaistFront,
    WaistLeft,
    WaistMidBack,
    WaistRight,
    UnknownValue(u64),
}

impl<N> From<N> for BodyLocation
    where N: Into<u64>
{
    fn from(number: N) -> BodyLocation {
        match number.into() {
            0 => BodyLocation::LeftLeg,
            1 => BodyLocation::LeftCalf,
            2 => BodyLocation::LeftShin,
            3 => BodyLocation::LeftHamstring,
            4 => BodyLocation::LeftQuad,
            5 => BodyLocation::LeftGlute,
            6 => BodyLocation::RightLeg,
            7 => BodyLocation::RightCalf,
            8 => BodyLocation::RightShin,
            9 => BodyLocation::RightHamstring,
            10 => BodyLocation::RightQuad,
            11 => BodyLocation::RightGlute,
            12 => BodyLocation::TorsoBack,
            13 => BodyLocation::LeftLowerBack,
            14 => BodyLocation::LeftUpperBack,
            15 => BodyLocation::RightLowerBack,
            16 => BodyLocation::RightUpperBack,
            17 => BodyLocation::TorsoFront,
            18 => BodyLocation::LeftAbdomen,
            19 => BodyLocation::LeftChest,
            20 => BodyLocation::RightAbdomen,
            21 => BodyLocation::RightChest,
            22 => BodyLocation::LeftArm,
            23 => BodyLocation::LeftShoulder,
            24 => BodyLocation::LeftBicep,
            25 => BodyLocation::LeftTricep,
            26 => BodyLocation::LeftBrachioradialis,
            27 => BodyLocation::LeftForearmExtensors,
            28 => BodyLocation::RightArm,
            29 => BodyLocation::RightShoulder,
            30 => BodyLocation::RightBicep,
            31 => BodyLocation::RightTricep,
            32 => BodyLocation::RightBrachioradialis,
            33 => BodyLocation::RightForearmExtensors,
            34 => BodyLocation::Neck,
            35 => BodyLocation::Throat,
            36 => BodyLocation::WaistMidBack,
            37 => BodyLocation::WaistFront,
            38 => BodyLocation::WaistLeft,
            39 => BodyLocation::WaistRight,
            n => BodyLocation::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum SegmentLapStatus {
    End,
    Fail,
    UnknownValue(u64),
}

impl<N> From<N> for SegmentLapStatus
    where N: Into<u64>
{
    fn from(number: N) -> SegmentLapStatus {
        match number.into() {
            0 => SegmentLapStatus::End,
            1 => SegmentLapStatus::Fail,
            n => SegmentLapStatus::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
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

impl<N> From<N> for SegmentLeaderboardType
    where N: Into<u64>
{
    fn from(number: N) -> SegmentLeaderboardType {
        match number.into() {
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
            n => SegmentLeaderboardType::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum SegmentDeleteStatus {
    DeleteAll,
    DeleteOne,
    DoNotDelete,
    UnknownValue(u64),
}

impl<N> From<N> for SegmentDeleteStatus
    where N: Into<u64>
{
    fn from(number: N) -> SegmentDeleteStatus {
        match number.into() {
            0 => SegmentDeleteStatus::DoNotDelete,
            1 => SegmentDeleteStatus::DeleteOne,
            2 => SegmentDeleteStatus::DeleteAll,
            n => SegmentDeleteStatus::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum SegmentSelectionType {
    Starred,
    Suggested,
    UnknownValue(u64),
}

impl<N> From<N> for SegmentSelectionType
    where N: Into<u64>
{
    fn from(number: N) -> SegmentSelectionType {
        match number.into() {
            0 => SegmentSelectionType::Starred,
            1 => SegmentSelectionType::Suggested,
            n => SegmentSelectionType::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum SourceType {
    Ant,
    Antplus,
    Bluetooth,
    BluetoothLowEnergy,
    Local,
    Wifi,
    UnknownValue(u64),
}

impl<N> From<N> for SourceType
    where N: Into<u64>
{
    fn from(number: N) -> SourceType {
        match number.into() {
            0 => SourceType::Ant,
            1 => SourceType::Antplus,
            2 => SourceType::Bluetooth,
            3 => SourceType::BluetoothLowEnergy,
            4 => SourceType::Wifi,
            5 => SourceType::Local,
            n => SourceType::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum LocalDeviceType {
    UnknownValue(u64),
}

impl<N> From<N> for LocalDeviceType
    where N: Into<u64>
{
    fn from(number: N) -> LocalDeviceType {
        match number.into() {
            n => LocalDeviceType::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum WorkoutEquipment {
    None,
    SwimFins,
    SwimKickboard,
    SwimPaddles,
    SwimPullBuoy,
    SwimSnorkel,
    UnknownValue(u64),
}

impl<N> From<N> for WorkoutEquipment
    where N: Into<u64>
{
    fn from(number: N) -> WorkoutEquipment {
        match number.into() {
            0 => WorkoutEquipment::None,
            1 => WorkoutEquipment::SwimFins,
            2 => WorkoutEquipment::SwimKickboard,
            3 => WorkoutEquipment::SwimPaddles,
            4 => WorkoutEquipment::SwimPullBuoy,
            5 => WorkoutEquipment::SwimSnorkel,
            n => WorkoutEquipment::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum DigitalWatchfaceLayout {
    Bold,
    Modern,
    Traditional,
    UnknownValue(u64),
}

impl<N> From<N> for DigitalWatchfaceLayout
    where N: Into<u64>
{
    fn from(number: N) -> DigitalWatchfaceLayout {
        match number.into() {
            0 => DigitalWatchfaceLayout::Traditional,
            1 => DigitalWatchfaceLayout::Modern,
            2 => DigitalWatchfaceLayout::Bold,
            n => DigitalWatchfaceLayout::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum RiderPositionType {
    Seated,
    Standing,
    TransitionToSeated,
    TransitionToStanding,
    UnknownValue(u64),
}

impl<N> From<N> for RiderPositionType
    where N: Into<u64>
{
    fn from(number: N) -> RiderPositionType {
        match number.into() {
            0 => RiderPositionType::Seated,
            1 => RiderPositionType::Standing,
            2 => RiderPositionType::TransitionToSeated,
            3 => RiderPositionType::TransitionToStanding,
            n => RiderPositionType::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum PowerPhaseType {
    PowerPhaseArcLength,
    PowerPhaseCenter,
    PowerPhaseEndAngle,
    PowerPhaseStartAngle,
    UnknownValue(u64),
}

impl<N> From<N> for PowerPhaseType
    where N: Into<u64>
{
    fn from(number: N) -> PowerPhaseType {
        match number.into() {
            0 => PowerPhaseType::PowerPhaseStartAngle,
            1 => PowerPhaseType::PowerPhaseEndAngle,
            2 => PowerPhaseType::PowerPhaseArcLength,
            3 => PowerPhaseType::PowerPhaseCenter,
            n => PowerPhaseType::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
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

impl<N> From<N> for CameraEventType
    where N: Into<u64>
{
    fn from(number: N) -> CameraEventType {
        match number.into() {
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
            n => CameraEventType::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum SensorType {
    Accelerometer,
    Barometer,
    Compass,
    Gyroscope,
    UnknownValue(u64),
}

impl<N> From<N> for SensorType
    where N: Into<u64>
{
    fn from(number: N) -> SensorType {
        match number.into() {
            0 => SensorType::Accelerometer,
            1 => SensorType::Gyroscope,
            2 => SensorType::Compass,
            3 => SensorType::Barometer,
            n => SensorType::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum BikeLightNetworkConfigType {
    Auto,
    HighVisibility,
    Individual,
    Trail,
    UnknownValue(u64),
}

impl<N> From<N> for BikeLightNetworkConfigType
    where N: Into<u64>
{
    fn from(number: N) -> BikeLightNetworkConfigType {
        match number.into() {
            0 => BikeLightNetworkConfigType::Auto,
            4 => BikeLightNetworkConfigType::Individual,
            5 => BikeLightNetworkConfigType::HighVisibility,
            6 => BikeLightNetworkConfigType::Trail,
            n => BikeLightNetworkConfigType::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum CameraOrientationType {
    CameraOrientation0,
    CameraOrientation180,
    CameraOrientation270,
    CameraOrientation90,
    UnknownValue(u64),
}

impl<N> From<N> for CameraOrientationType
    where N: Into<u64>
{
    fn from(number: N) -> CameraOrientationType {
        match number.into() {
            0 => CameraOrientationType::CameraOrientation0,
            1 => CameraOrientationType::CameraOrientation90,
            2 => CameraOrientationType::CameraOrientation180,
            3 => CameraOrientationType::CameraOrientation270,
            n => CameraOrientationType::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum AttitudeStage {
    Aligning,
    Degraded,
    Failed,
    Valid,
    UnknownValue(u64),
}

impl<N> From<N> for AttitudeStage
    where N: Into<u64>
{
    fn from(number: N) -> AttitudeStage {
        match number.into() {
            0 => AttitudeStage::Failed,
            1 => AttitudeStage::Aligning,
            2 => AttitudeStage::Degraded,
            3 => AttitudeStage::Valid,
            n => AttitudeStage::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum AttitudeValidity {
    GpsInvalid,
    HwFail,
    LateralBodyAccelValid,
    MagInvalid,
    MagneticHeading,
    NoGps,
    NormalBodyAccelValid,
    PitchValid,
    RollValid,
    SolutionCoasting,
    TrackAngleHeadingValid,
    TrueTrackAngle,
    TurnRateValid,
    UnknownValue(u64),
}

impl<N> From<N> for AttitudeValidity
    where N: Into<u64>
{
    fn from(number: N) -> AttitudeValidity {
        match number.into() {
            1 => AttitudeValidity::TrackAngleHeadingValid,
            2 => AttitudeValidity::PitchValid,
            4 => AttitudeValidity::RollValid,
            8 => AttitudeValidity::LateralBodyAccelValid,
            16 => AttitudeValidity::NormalBodyAccelValid,
            32 => AttitudeValidity::TurnRateValid,
            64 => AttitudeValidity::HwFail,
            128 => AttitudeValidity::MagInvalid,
            256 => AttitudeValidity::NoGps,
            512 => AttitudeValidity::GpsInvalid,
            1024 => AttitudeValidity::SolutionCoasting,
            2048 => AttitudeValidity::TrueTrackAngle,
            4096 => AttitudeValidity::MagneticHeading,
            n => AttitudeValidity::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum AutoSyncFrequency {
    Frequent,
    Never,
    Occasionally,
    OnceADay,
    Remote,
    UnknownValue(u64),
}

impl<N> From<N> for AutoSyncFrequency
    where N: Into<u64>
{
    fn from(number: N) -> AutoSyncFrequency {
        match number.into() {
            0 => AutoSyncFrequency::Never,
            1 => AutoSyncFrequency::Occasionally,
            2 => AutoSyncFrequency::Frequent,
            3 => AutoSyncFrequency::OnceADay,
            4 => AutoSyncFrequency::Remote,
            n => AutoSyncFrequency::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum ExdDisplayType {
    Balance,
    Bar,
    CircleGraph,
    Gauge,
    Graph,
    Numerical,
    Simple,
    SimpleDynamicIcon,
    String,
    StringList,
    VirtualPartner,
    UnknownValue(u64),
}

impl<N> From<N> for ExdDisplayType
    where N: Into<u64>
{
    fn from(number: N) -> ExdDisplayType {
        match number.into() {
            0 => ExdDisplayType::Numerical,
            1 => ExdDisplayType::Simple,
            2 => ExdDisplayType::Graph,
            3 => ExdDisplayType::Bar,
            4 => ExdDisplayType::CircleGraph,
            5 => ExdDisplayType::VirtualPartner,
            6 => ExdDisplayType::Balance,
            7 => ExdDisplayType::StringList,
            8 => ExdDisplayType::String,
            9 => ExdDisplayType::SimpleDynamicIcon,
            10 => ExdDisplayType::Gauge,
            n => ExdDisplayType::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum ExdDataUnits {
    Bpm,
    Bradians,
    Calories,
    Centimeter,
    Degrees,
    DegreesCelsius,
    DegreesFarenheit,
    EightCardinal,
    EnumBatteryStatus,
    EnumBikeLightBatteryStatus,
    EnumBikeLightBeamAngleMode,
    EnumBikeLightNetworkConfigType,
    EnumCoursePoint,
    EnumSport,
    EnumTurnType,
    Feet,
    FeetPerHour,
    FeetPerMin,
    Gear,
    HectoPascals,
    Hours,
    InchesHg,
    Kilofeet,
    Kilojoules,
    Kilometers,
    KilometersPerHour,
    Laps,
    Lights,
    Mbars,
    Meters,
    MetersPerHour,
    MetersPerMin,
    MetersPerSec,
    Miles,
    MilesPerHour,
    Millimeters,
    Milliseconds,
    Minutes,
    MmHg,
    NoUnits,
    Percent,
    Rpm,
    SecondPerKilometer,
    SecondPerMile,
    Seconds,
    Time,
    Watts,
    WattsPerKilogram,
    Yards,
    Zone,
    UnknownValue(u64),
}

impl<N> From<N> for ExdDataUnits
    where N: Into<u64>
{
    fn from(number: N) -> ExdDataUnits {
        match number.into() {
            0 => ExdDataUnits::NoUnits,
            1 => ExdDataUnits::Laps,
            2 => ExdDataUnits::MilesPerHour,
            3 => ExdDataUnits::KilometersPerHour,
            4 => ExdDataUnits::FeetPerHour,
            5 => ExdDataUnits::MetersPerHour,
            6 => ExdDataUnits::DegreesCelsius,
            7 => ExdDataUnits::DegreesFarenheit,
            8 => ExdDataUnits::Zone,
            9 => ExdDataUnits::Gear,
            10 => ExdDataUnits::Rpm,
            11 => ExdDataUnits::Bpm,
            12 => ExdDataUnits::Degrees,
            13 => ExdDataUnits::Millimeters,
            14 => ExdDataUnits::Meters,
            15 => ExdDataUnits::Kilometers,
            16 => ExdDataUnits::Feet,
            17 => ExdDataUnits::Yards,
            18 => ExdDataUnits::Kilofeet,
            19 => ExdDataUnits::Miles,
            20 => ExdDataUnits::Time,
            21 => ExdDataUnits::EnumTurnType,
            22 => ExdDataUnits::Percent,
            23 => ExdDataUnits::Watts,
            24 => ExdDataUnits::WattsPerKilogram,
            25 => ExdDataUnits::EnumBatteryStatus,
            26 => ExdDataUnits::EnumBikeLightBeamAngleMode,
            27 => ExdDataUnits::EnumBikeLightBatteryStatus,
            28 => ExdDataUnits::EnumBikeLightNetworkConfigType,
            29 => ExdDataUnits::Lights,
            30 => ExdDataUnits::Seconds,
            31 => ExdDataUnits::Minutes,
            32 => ExdDataUnits::Hours,
            33 => ExdDataUnits::Calories,
            34 => ExdDataUnits::Kilojoules,
            35 => ExdDataUnits::Milliseconds,
            36 => ExdDataUnits::SecondPerMile,
            37 => ExdDataUnits::SecondPerKilometer,
            38 => ExdDataUnits::Centimeter,
            39 => ExdDataUnits::EnumCoursePoint,
            40 => ExdDataUnits::Bradians,
            41 => ExdDataUnits::EnumSport,
            42 => ExdDataUnits::InchesHg,
            43 => ExdDataUnits::MmHg,
            44 => ExdDataUnits::Mbars,
            45 => ExdDataUnits::HectoPascals,
            46 => ExdDataUnits::FeetPerMin,
            47 => ExdDataUnits::MetersPerMin,
            48 => ExdDataUnits::MetersPerSec,
            49 => ExdDataUnits::EightCardinal,
            n => ExdDataUnits::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum ExdQualifiers {
    Average,
    AverageLap,
    ComparedToVirtualPartner,
    Elapsed,
    EstimatedTotal,
    First,
    Instantaneous,
    Lap,
    LapPercentMaximum,
    LastLap,
    LastSport,
    Maximum,
    Maximum24H,
    MaximumAverage,
    MaximumLap,
    Minimum,
    Minimum24H,
    Moving,
    NextCoursePoint,
    NoQualifier,
    PercentMaximum,
    PercentMaximumAverage,
    Second,
    Shifter,
    Stopped,
    Sunrise,
    Sunset,
    TenSecondAverage,
    Third,
    ThirtySecondAverage,
    ThreeSecondAverage,
    ToDestination,
    ToGo,
    ToNext,
    Total,
    Zone1,
    Zone2,
    Zone3,
    Zone4,
    Zone5,
    Zone6,
    Zone7,
    Zone8,
    Zone9,
    UnknownValue(u64),
}

impl<N> From<N> for ExdQualifiers
    where N: Into<u64>
{
    fn from(number: N) -> ExdQualifiers {
        match number.into() {
            0 => ExdQualifiers::NoQualifier,
            1 => ExdQualifiers::Instantaneous,
            2 => ExdQualifiers::Average,
            3 => ExdQualifiers::Lap,
            4 => ExdQualifiers::Maximum,
            5 => ExdQualifiers::MaximumAverage,
            6 => ExdQualifiers::MaximumLap,
            7 => ExdQualifiers::LastLap,
            8 => ExdQualifiers::AverageLap,
            9 => ExdQualifiers::ToDestination,
            10 => ExdQualifiers::ToGo,
            11 => ExdQualifiers::ToNext,
            12 => ExdQualifiers::NextCoursePoint,
            13 => ExdQualifiers::Total,
            14 => ExdQualifiers::ThreeSecondAverage,
            15 => ExdQualifiers::TenSecondAverage,
            16 => ExdQualifiers::ThirtySecondAverage,
            17 => ExdQualifiers::PercentMaximum,
            18 => ExdQualifiers::PercentMaximumAverage,
            19 => ExdQualifiers::LapPercentMaximum,
            20 => ExdQualifiers::Elapsed,
            21 => ExdQualifiers::Sunrise,
            22 => ExdQualifiers::Sunset,
            23 => ExdQualifiers::ComparedToVirtualPartner,
            24 => ExdQualifiers::Maximum24H,
            25 => ExdQualifiers::Minimum24H,
            26 => ExdQualifiers::Minimum,
            27 => ExdQualifiers::First,
            28 => ExdQualifiers::Second,
            29 => ExdQualifiers::Third,
            30 => ExdQualifiers::Shifter,
            31 => ExdQualifiers::LastSport,
            32 => ExdQualifiers::Moving,
            33 => ExdQualifiers::Stopped,
            34 => ExdQualifiers::EstimatedTotal,
            242 => ExdQualifiers::Zone9,
            243 => ExdQualifiers::Zone8,
            244 => ExdQualifiers::Zone7,
            245 => ExdQualifiers::Zone6,
            246 => ExdQualifiers::Zone5,
            247 => ExdQualifiers::Zone4,
            248 => ExdQualifiers::Zone3,
            249 => ExdQualifiers::Zone2,
            250 => ExdQualifiers::Zone1,
            n => ExdQualifiers::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum ExdDescriptors {
    AmbientPressure,
    AnaerobicTrainingEffect,
    Ascent,
    Balance,
    BateryLevel,
    BatteryLevel,
    BeamAngleStatus,
    BikeLightBatteryStatus,
    Cadence,
    Calories,
    Compass,
    CompassHeading,
    Course,
    CourseDistance,
    CourseEstimatedTimeOfArrival,
    CourseHeading,
    CourseLocation,
    CourseTime,
    CourseType,
    Descent,
    Di2BatteryLevel,
    Distance,
    Elevation,
    EstimatedTimeOfArrival,
    FrontGear,
    FunctionalThresholdPower,
    GearCombo,
    GearRatio,
    Gears,
    GlideRatio,
    GpsAccuracy,
    GpsElevation,
    GpsHeading,
    GpsSignalStrength,
    Grade,
    GroundContactTime,
    Heading,
    HeartRate,
    HeartRateReserve,
    HeartRateZone,
    Icon,
    IntensityFactor,
    Laps,
    LeftGroundContactTimeBalance,
    LeftPlatformCenterOffset,
    LeftPowerPhaseFinishAngle,
    LeftPowerPhaseStartAngle,
    LightNetworkMode,
    MuscleOxygen,
    NavigationDistance,
    NavigationEstimatedTimeOfArrival,
    NavigationHeading,
    NavigationLocation,
    NavigationTime,
    NavigationTurn,
    NormalizedPower,
    NumberLightsConnected,
    OffCourse,
    Pace,
    PedalSmoothness,
    PerformanceCondition,
    Power,
    PowerRatio,
    PowerWeightRatio,
    PowerZone,
    Pressure,
    RearGear,
    Reps,
    RightGroundContactTimeBalance,
    RightPlatformCenterOffset,
    RightPowerPhaseFinishAngle,
    RightPowerPhaseStartAngle,
    RunningCadence,
    Speed,
    StrideLength,
    Temperature,
    Time,
    TimeInHeartRateZone,
    TimeInPowerZone,
    TimeOfDay,
    TimeOnZone,
    TimeSeated,
    TimeStanding,
    TimerTime,
    TorqueEffectiveness,
    TrainerResistance,
    TrainerTargetPower,
    TrainingEffect,
    TrainingStressScore,
    Vam,
    VerticalDistance,
    VerticalOscillation,
    VerticalRatio,
    VerticalSpeed,
    Vmg,
    Work,
    WorkoutStep,
    UnknownValue(u64),
}

impl<N> From<N> for ExdDescriptors
    where N: Into<u64>
{
    fn from(number: N) -> ExdDescriptors {
        match number.into() {
            0 => ExdDescriptors::BikeLightBatteryStatus,
            1 => ExdDescriptors::BeamAngleStatus,
            2 => ExdDescriptors::BateryLevel,
            3 => ExdDescriptors::LightNetworkMode,
            4 => ExdDescriptors::NumberLightsConnected,
            5 => ExdDescriptors::Cadence,
            6 => ExdDescriptors::Distance,
            7 => ExdDescriptors::EstimatedTimeOfArrival,
            8 => ExdDescriptors::Heading,
            9 => ExdDescriptors::Time,
            10 => ExdDescriptors::BatteryLevel,
            11 => ExdDescriptors::TrainerResistance,
            12 => ExdDescriptors::TrainerTargetPower,
            13 => ExdDescriptors::TimeSeated,
            14 => ExdDescriptors::TimeStanding,
            15 => ExdDescriptors::Elevation,
            16 => ExdDescriptors::Grade,
            17 => ExdDescriptors::Ascent,
            18 => ExdDescriptors::Descent,
            19 => ExdDescriptors::VerticalSpeed,
            20 => ExdDescriptors::Di2BatteryLevel,
            21 => ExdDescriptors::FrontGear,
            22 => ExdDescriptors::RearGear,
            23 => ExdDescriptors::GearRatio,
            24 => ExdDescriptors::HeartRate,
            25 => ExdDescriptors::HeartRateZone,
            26 => ExdDescriptors::TimeInHeartRateZone,
            27 => ExdDescriptors::HeartRateReserve,
            28 => ExdDescriptors::Calories,
            29 => ExdDescriptors::GpsAccuracy,
            30 => ExdDescriptors::GpsSignalStrength,
            31 => ExdDescriptors::Temperature,
            32 => ExdDescriptors::TimeOfDay,
            33 => ExdDescriptors::Balance,
            34 => ExdDescriptors::PedalSmoothness,
            35 => ExdDescriptors::Power,
            36 => ExdDescriptors::FunctionalThresholdPower,
            37 => ExdDescriptors::IntensityFactor,
            38 => ExdDescriptors::Work,
            39 => ExdDescriptors::PowerRatio,
            40 => ExdDescriptors::NormalizedPower,
            41 => ExdDescriptors::TrainingStressScore,
            42 => ExdDescriptors::TimeOnZone,
            43 => ExdDescriptors::Speed,
            44 => ExdDescriptors::Laps,
            45 => ExdDescriptors::Reps,
            46 => ExdDescriptors::WorkoutStep,
            47 => ExdDescriptors::CourseDistance,
            48 => ExdDescriptors::NavigationDistance,
            49 => ExdDescriptors::CourseEstimatedTimeOfArrival,
            50 => ExdDescriptors::NavigationEstimatedTimeOfArrival,
            51 => ExdDescriptors::CourseTime,
            52 => ExdDescriptors::NavigationTime,
            53 => ExdDescriptors::CourseHeading,
            54 => ExdDescriptors::NavigationHeading,
            55 => ExdDescriptors::PowerZone,
            56 => ExdDescriptors::TorqueEffectiveness,
            57 => ExdDescriptors::TimerTime,
            58 => ExdDescriptors::PowerWeightRatio,
            59 => ExdDescriptors::LeftPlatformCenterOffset,
            60 => ExdDescriptors::RightPlatformCenterOffset,
            61 => ExdDescriptors::LeftPowerPhaseStartAngle,
            62 => ExdDescriptors::RightPowerPhaseStartAngle,
            63 => ExdDescriptors::LeftPowerPhaseFinishAngle,
            64 => ExdDescriptors::RightPowerPhaseFinishAngle,
            65 => ExdDescriptors::Gears,
            66 => ExdDescriptors::Pace,
            67 => ExdDescriptors::TrainingEffect,
            68 => ExdDescriptors::VerticalOscillation,
            69 => ExdDescriptors::VerticalRatio,
            70 => ExdDescriptors::GroundContactTime,
            71 => ExdDescriptors::LeftGroundContactTimeBalance,
            72 => ExdDescriptors::RightGroundContactTimeBalance,
            73 => ExdDescriptors::StrideLength,
            74 => ExdDescriptors::RunningCadence,
            75 => ExdDescriptors::PerformanceCondition,
            76 => ExdDescriptors::CourseType,
            77 => ExdDescriptors::TimeInPowerZone,
            78 => ExdDescriptors::NavigationTurn,
            79 => ExdDescriptors::CourseLocation,
            80 => ExdDescriptors::NavigationLocation,
            81 => ExdDescriptors::Compass,
            82 => ExdDescriptors::GearCombo,
            83 => ExdDescriptors::MuscleOxygen,
            84 => ExdDescriptors::Icon,
            85 => ExdDescriptors::CompassHeading,
            86 => ExdDescriptors::GpsHeading,
            87 => ExdDescriptors::GpsElevation,
            88 => ExdDescriptors::AnaerobicTrainingEffect,
            89 => ExdDescriptors::Course,
            90 => ExdDescriptors::OffCourse,
            91 => ExdDescriptors::GlideRatio,
            92 => ExdDescriptors::VerticalDistance,
            93 => ExdDescriptors::Vmg,
            94 => ExdDescriptors::AmbientPressure,
            95 => ExdDescriptors::Pressure,
            96 => ExdDescriptors::Vam,
            n => ExdDescriptors::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
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

impl<N> From<N> for SupportedExdScreenLayouts
    where N: Into<u64>
{
    fn from(number: N) -> SupportedExdScreenLayouts {
        match number.into() {
            1 => SupportedExdScreenLayouts::FullScreen,
            2 => SupportedExdScreenLayouts::HalfVertical,
            4 => SupportedExdScreenLayouts::HalfHorizontal,
            8 => SupportedExdScreenLayouts::HalfVerticalRightSplit,
            16 => SupportedExdScreenLayouts::HalfHorizontalBottomSplit,
            32 => SupportedExdScreenLayouts::FullQuarterSplit,
            64 => SupportedExdScreenLayouts::HalfVerticalLeftSplit,
            128 => SupportedExdScreenLayouts::HalfHorizontalTopSplit,
            n => SupportedExdScreenLayouts::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum FitBaseType {
    Byte,
    Enum,
    Float32,
    Float64,
    Sint16,
    Sint32,
    Sint64,
    Sint8,
    String,
    Uint16,
    Uint16Z,
    Uint32,
    Uint32Z,
    Uint64,
    Uint64Z,
    Uint8,
    Uint8Z,
    UnknownValue(u64),
}

impl<N> From<N> for FitBaseType
    where N: Into<u64>
{
    fn from(number: N) -> FitBaseType {
        match number.into() {
            0 => FitBaseType::Enum,
            1 => FitBaseType::Sint8,
            2 => FitBaseType::Uint8,
            7 => FitBaseType::String,
            10 => FitBaseType::Uint8Z,
            13 => FitBaseType::Byte,
            131 => FitBaseType::Sint16,
            132 => FitBaseType::Uint16,
            133 => FitBaseType::Sint32,
            134 => FitBaseType::Uint32,
            136 => FitBaseType::Float32,
            137 => FitBaseType::Float64,
            139 => FitBaseType::Uint16Z,
            140 => FitBaseType::Uint32Z,
            142 => FitBaseType::Sint64,
            143 => FitBaseType::Uint64,
            144 => FitBaseType::Uint64Z,
            n => FitBaseType::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum BikeLightBeamAngleMode {
    Auto,
    Manual,
    UnknownValue(u64),
}

impl<N> From<N> for BikeLightBeamAngleMode
    where N: Into<u64>
{
    fn from(number: N) -> BikeLightBeamAngleMode {
        match number.into() {
            0 => BikeLightBeamAngleMode::Manual,
            1 => BikeLightBeamAngleMode::Auto,
            n => BikeLightBeamAngleMode::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum FitBaseUnit {
    Kilogram,
    Other,
    Pound,
    UnknownValue(u64),
}

impl<N> From<N> for FitBaseUnit
    where N: Into<u64>
{
    fn from(number: N) -> FitBaseUnit {
        match number.into() {
            0 => FitBaseUnit::Other,
            1 => FitBaseUnit::Kilogram,
            2 => FitBaseUnit::Pound,
            n => FitBaseUnit::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum ExerciseCategory {
    BenchPress,
    CalfRaise,
    Cardio,
    Carry,
    Chop,
    Core,
    Crunch,
    Curl,
    Deadlift,
    Flye,
    HipRaise,
    HipStability,
    HipSwing,
    Hyperextension,
    LateralRaise,
    LegCurl,
    LegRaise,
    Lunge,
    OlympicLift,
    Plank,
    Plyo,
    PullUp,
    PushUp,
    Row,
    Run,
    ShoulderPress,
    ShoulderStability,
    Shrug,
    SitUp,
    Squat,
    TotalBody,
    TricepsExtension,
    Unknown,
    WarmUp,
    UnknownValue(u64),
}

impl<N> From<N> for ExerciseCategory
    where N: Into<u64>
{
    fn from(number: N) -> ExerciseCategory {
        match number.into() {
            0 => ExerciseCategory::BenchPress,
            1 => ExerciseCategory::CalfRaise,
            2 => ExerciseCategory::Cardio,
            3 => ExerciseCategory::Carry,
            4 => ExerciseCategory::Chop,
            5 => ExerciseCategory::Core,
            6 => ExerciseCategory::Crunch,
            7 => ExerciseCategory::Curl,
            8 => ExerciseCategory::Deadlift,
            9 => ExerciseCategory::Flye,
            10 => ExerciseCategory::HipRaise,
            11 => ExerciseCategory::HipStability,
            12 => ExerciseCategory::HipSwing,
            13 => ExerciseCategory::Hyperextension,
            14 => ExerciseCategory::LateralRaise,
            15 => ExerciseCategory::LegCurl,
            16 => ExerciseCategory::LegRaise,
            17 => ExerciseCategory::Lunge,
            18 => ExerciseCategory::OlympicLift,
            19 => ExerciseCategory::Plank,
            20 => ExerciseCategory::Plyo,
            21 => ExerciseCategory::PullUp,
            22 => ExerciseCategory::PushUp,
            23 => ExerciseCategory::Row,
            24 => ExerciseCategory::ShoulderPress,
            25 => ExerciseCategory::ShoulderStability,
            26 => ExerciseCategory::Shrug,
            27 => ExerciseCategory::SitUp,
            28 => ExerciseCategory::Squat,
            29 => ExerciseCategory::TotalBody,
            30 => ExerciseCategory::TricepsExtension,
            31 => ExerciseCategory::WarmUp,
            32 => ExerciseCategory::Run,
            65534 => ExerciseCategory::Unknown,
            n => ExerciseCategory::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum CalfRaiseExerciseName {
    DonkeyCalfRaise,
    SeatedCalfRaise,
    SeatedDumbbellToeRaise,
    SingleLegBentKneeCalfRaise,
    SingleLegDeclinePushUp,
    SingleLegDonkeyCalfRaise,
    SingleLegHipRaiseWithKneeHold,
    SingleLegStandingCalfRaise,
    SingleLegStandingDumbbellCalfRaise,
    StandingBarbellCalfRaise,
    StandingCalfRaise,
    StandingDumbbellCalfRaise,
    ThreeWayCalfRaise,
    ThreeWaySingleLegCalfRaise,
    ThreeWayWeightedCalfRaise,
    ThreeWayWeightedSingleLegCalfRaise,
    WeightedDonkeyCalfRaise,
    WeightedSeatedCalfRaise,
    WeightedSingleLegBentKneeCalfRaise,
    WeightedSingleLegDonkeyCalfRaise,
    WeightedStandingCalfRaise,
    UnknownValue(u64),
}

impl<N> From<N> for CalfRaiseExerciseName
    where N: Into<u64>
{
    fn from(number: N) -> CalfRaiseExerciseName {
        match number.into() {
            0 => CalfRaiseExerciseName::ThreeWayCalfRaise,
            1 => CalfRaiseExerciseName::ThreeWayWeightedCalfRaise,
            2 => CalfRaiseExerciseName::ThreeWaySingleLegCalfRaise,
            3 => CalfRaiseExerciseName::ThreeWayWeightedSingleLegCalfRaise,
            4 => CalfRaiseExerciseName::DonkeyCalfRaise,
            5 => CalfRaiseExerciseName::WeightedDonkeyCalfRaise,
            6 => CalfRaiseExerciseName::SeatedCalfRaise,
            7 => CalfRaiseExerciseName::WeightedSeatedCalfRaise,
            8 => CalfRaiseExerciseName::SeatedDumbbellToeRaise,
            9 => CalfRaiseExerciseName::SingleLegBentKneeCalfRaise,
            10 => CalfRaiseExerciseName::WeightedSingleLegBentKneeCalfRaise,
            11 => CalfRaiseExerciseName::SingleLegDeclinePushUp,
            12 => CalfRaiseExerciseName::SingleLegDonkeyCalfRaise,
            13 => CalfRaiseExerciseName::WeightedSingleLegDonkeyCalfRaise,
            14 => CalfRaiseExerciseName::SingleLegHipRaiseWithKneeHold,
            15 => CalfRaiseExerciseName::SingleLegStandingCalfRaise,
            16 => CalfRaiseExerciseName::SingleLegStandingDumbbellCalfRaise,
            17 => CalfRaiseExerciseName::StandingBarbellCalfRaise,
            18 => CalfRaiseExerciseName::StandingCalfRaise,
            19 => CalfRaiseExerciseName::WeightedStandingCalfRaise,
            20 => CalfRaiseExerciseName::StandingDumbbellCalfRaise,
            n => CalfRaiseExerciseName::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum CarryExerciseName {
    BarHolds,
    FarmersWalk,
    FarmersWalkOnToes,
    HexDumbbellHold,
    OverheadCarry,
    UnknownValue(u64),
}

impl<N> From<N> for CarryExerciseName
    where N: Into<u64>
{
    fn from(number: N) -> CarryExerciseName {
        match number.into() {
            0 => CarryExerciseName::BarHolds,
            1 => CarryExerciseName::FarmersWalk,
            2 => CarryExerciseName::FarmersWalkOnToes,
            3 => CarryExerciseName::HexDumbbellHold,
            4 => CarryExerciseName::OverheadCarry,
            n => CarryExerciseName::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum CoreExerciseName {
    AbdominalLegRotations,
    AbsJabs,
    AlternatingPlateReach,
    AlternatingSlideOut,
    ArmAndLegExtensionOnKnees,
    BarbellRollout,
    BicepCurlWithLegExtension,
    Bicycle,
    BodyBarObliqueTwist,
    CableCorePress,
    CableSideBend,
    CatCow,
    Corkscrew,
    CrescentCircle,
    CrissCross,
    CrissCrossWithBall,
    CyclingRussianTwist,
    DoubleLegStretch,
    ElevatedFeetRussianTwist,
    GhdBackExtensions,
    HalfTurkishGetUp,
    Inchworm,
    KettlebellWindmill,
    KneeFolds,
    KneelingAbWheel,
    LowerLift,
    ModifiedFrontLever,
    NeckPull,
    OpenKneeTucks,
    OverheadWalk,
    PelvicClocks,
    RollOver,
    RollUp,
    Rolling,
    Rowing1,
    Rowing2,
    RussianTwist,
    Scissors,
    SideAbsLegLift,
    SideBend,
    SingleLegCircles,
    SingleLegStretch,
    SnakeTwist1And2,
    Swan,
    Swimming,
    SwissBallJackknife,
    SwissBallPike,
    SwissBallRollout,
    Teaser,
    TheHundred,
    TriangleHipPress,
    TrxSuspendedJackknife,
    UBoat,
    WeightedAbsJabs,
    WeightedAlternatingSlideOut,
    WeightedBarbellRollout,
    WeightedCrescentCircle,
    WeightedCyclingRussianTwist,
    WeightedElevatedFeetRussianTwist,
    WeightedGhdBackExtensions,
    WeightedKneelingAbWheel,
    WeightedModifiedFrontLever,
    WeightedOpenKneeTucks,
    WeightedSideAbsLegLift,
    WeightedSideBend,
    WeightedSwissBallJackknife,
    WeightedSwissBallPike,
    WeightedSwissBallRollout,
    WeightedTriangleHipPress,
    WeightedTrxSuspendedJackknife,
    WeightedUBoat,
    WeightedWindmillSwitches,
    WindmillSwitches,
    UnknownValue(u64),
}

impl<N> From<N> for CoreExerciseName
    where N: Into<u64>
{
    fn from(number: N) -> CoreExerciseName {
        match number.into() {
            0 => CoreExerciseName::AbsJabs,
            1 => CoreExerciseName::WeightedAbsJabs,
            2 => CoreExerciseName::AlternatingPlateReach,
            3 => CoreExerciseName::BarbellRollout,
            4 => CoreExerciseName::WeightedBarbellRollout,
            5 => CoreExerciseName::BodyBarObliqueTwist,
            6 => CoreExerciseName::CableCorePress,
            7 => CoreExerciseName::CableSideBend,
            8 => CoreExerciseName::SideBend,
            9 => CoreExerciseName::WeightedSideBend,
            10 => CoreExerciseName::CrescentCircle,
            11 => CoreExerciseName::WeightedCrescentCircle,
            12 => CoreExerciseName::CyclingRussianTwist,
            13 => CoreExerciseName::WeightedCyclingRussianTwist,
            14 => CoreExerciseName::ElevatedFeetRussianTwist,
            15 => CoreExerciseName::WeightedElevatedFeetRussianTwist,
            16 => CoreExerciseName::HalfTurkishGetUp,
            17 => CoreExerciseName::KettlebellWindmill,
            18 => CoreExerciseName::KneelingAbWheel,
            19 => CoreExerciseName::WeightedKneelingAbWheel,
            20 => CoreExerciseName::ModifiedFrontLever,
            21 => CoreExerciseName::OpenKneeTucks,
            22 => CoreExerciseName::WeightedOpenKneeTucks,
            23 => CoreExerciseName::SideAbsLegLift,
            24 => CoreExerciseName::WeightedSideAbsLegLift,
            25 => CoreExerciseName::SwissBallJackknife,
            26 => CoreExerciseName::WeightedSwissBallJackknife,
            27 => CoreExerciseName::SwissBallPike,
            28 => CoreExerciseName::WeightedSwissBallPike,
            29 => CoreExerciseName::SwissBallRollout,
            30 => CoreExerciseName::WeightedSwissBallRollout,
            31 => CoreExerciseName::TriangleHipPress,
            32 => CoreExerciseName::WeightedTriangleHipPress,
            33 => CoreExerciseName::TrxSuspendedJackknife,
            34 => CoreExerciseName::WeightedTrxSuspendedJackknife,
            35 => CoreExerciseName::UBoat,
            36 => CoreExerciseName::WeightedUBoat,
            37 => CoreExerciseName::WindmillSwitches,
            38 => CoreExerciseName::WeightedWindmillSwitches,
            39 => CoreExerciseName::AlternatingSlideOut,
            40 => CoreExerciseName::WeightedAlternatingSlideOut,
            41 => CoreExerciseName::GhdBackExtensions,
            42 => CoreExerciseName::WeightedGhdBackExtensions,
            43 => CoreExerciseName::OverheadWalk,
            44 => CoreExerciseName::Inchworm,
            45 => CoreExerciseName::WeightedModifiedFrontLever,
            46 => CoreExerciseName::RussianTwist,
            47 => CoreExerciseName::AbdominalLegRotations,
            48 => CoreExerciseName::ArmAndLegExtensionOnKnees,
            49 => CoreExerciseName::Bicycle,
            50 => CoreExerciseName::BicepCurlWithLegExtension,
            51 => CoreExerciseName::CatCow,
            52 => CoreExerciseName::Corkscrew,
            53 => CoreExerciseName::CrissCross,
            54 => CoreExerciseName::CrissCrossWithBall,
            55 => CoreExerciseName::DoubleLegStretch,
            56 => CoreExerciseName::KneeFolds,
            57 => CoreExerciseName::LowerLift,
            58 => CoreExerciseName::NeckPull,
            59 => CoreExerciseName::PelvicClocks,
            60 => CoreExerciseName::RollOver,
            61 => CoreExerciseName::RollUp,
            62 => CoreExerciseName::Rolling,
            63 => CoreExerciseName::Rowing1,
            64 => CoreExerciseName::Rowing2,
            65 => CoreExerciseName::Scissors,
            66 => CoreExerciseName::SingleLegCircles,
            67 => CoreExerciseName::SingleLegStretch,
            68 => CoreExerciseName::SnakeTwist1And2,
            69 => CoreExerciseName::Swan,
            70 => CoreExerciseName::Swimming,
            71 => CoreExerciseName::Teaser,
            72 => CoreExerciseName::TheHundred,
            n => CoreExerciseName::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum CrunchExerciseName {
    BicycleCrunch,
    CableCrunch,
    CircularArmCrunch,
    CrossLegReverseCrunch,
    CrossedArmsCrunch,
    Crunch,
    CrunchChop,
    DoubleCrunch,
    ElbowToKneeCrunch,
    FlutterKicks,
    FoamRollerReverseCrunchOnBench,
    FoamRollerReverseCrunchWithDumbbell,
    FoamRollerReverseCrunchWithMedicineBall,
    FrogPress,
    HangingKneeRaiseObliqueCrunch,
    HipCrossover,
    HollowRock,
    InclineReverseCrunch,
    KneelingCableCrunch,
    KneelingCrossCrunch,
    KneelingObliqueCableCrunch,
    KneesToElbow,
    LegExtensions,
    LegLevers,
    McgillCurlUp,
    ModifiedPilatesRollUpWithBall,
    PilatesCrunch,
    PilatesRollUpWithBall,
    RaisedLegsCrunch,
    ReverseCrunch,
    ReverseCrunchOnABench,
    ReverseCurlAndLift,
    RotationalLift,
    SeatedAlternatingReverseCrunch,
    SeatedLegU,
    SideToSideCrunchAndWeave,
    SingleLegReverseCrunch,
    SkaterCrunchCross,
    StandingCableCrunch,
    StandingSideCrunch,
    StepClimb,
    StraightLegCrunchWithBall,
    SwissBallCrunch,
    SwissBallReverseCrunch,
    SwissBallRussianTwist,
    SwissBallSideCrunch,
    ThoracicCrunchesOnFoamRoller,
    ToesToBar,
    TricepsCrunch,
    WeightedBicycleCrunch,
    WeightedCrossLegReverseCrunch,
    WeightedCrossedArmsCrunch,
    WeightedCrunch,
    WeightedCrunchChop,
    WeightedDoubleCrunch,
    WeightedElbowToKneeCrunch,
    WeightedFlutterKicks,
    WeightedFoamRollerReverseCrunchOnBench,
    WeightedHangingKneeRaiseObliqueCrunch,
    WeightedHipCrossover,
    WeightedHollowRock,
    WeightedInclineReverseCrunch,
    WeightedKneelingCrossCrunch,
    WeightedLegExtensions,
    WeightedMcgillCurlUp,
    WeightedModifiedPilatesRollUpWithBall,
    WeightedPilatesCrunch,
    WeightedPilatesRollUpWithBall,
    WeightedRaisedLegsCrunch,
    WeightedReverseCrunch,
    WeightedReverseCrunchOnABench,
    WeightedReverseCurlAndLift,
    WeightedRotationalLift,
    WeightedSeatedAlternatingReverseCrunch,
    WeightedSeatedLegU,
    WeightedSideToSideCrunchAndWeave,
    WeightedSingleLegReverseCrunch,
    WeightedSkaterCrunchCross,
    WeightedStepClimb,
    WeightedSwissBallCrunch,
    WeightedSwissBallReverseCrunch,
    WeightedSwissBallRussianTwist,
    WeightedSwissBallSideCrunch,
    WeightedThoracicCrunchesOnFoamRoller,
    WeightedToesToBar,
    UnknownValue(u64),
}

impl<N> From<N> for CrunchExerciseName
    where N: Into<u64>
{
    fn from(number: N) -> CrunchExerciseName {
        match number.into() {
            0 => CrunchExerciseName::BicycleCrunch,
            1 => CrunchExerciseName::CableCrunch,
            2 => CrunchExerciseName::CircularArmCrunch,
            3 => CrunchExerciseName::CrossedArmsCrunch,
            4 => CrunchExerciseName::WeightedCrossedArmsCrunch,
            5 => CrunchExerciseName::CrossLegReverseCrunch,
            6 => CrunchExerciseName::WeightedCrossLegReverseCrunch,
            7 => CrunchExerciseName::CrunchChop,
            8 => CrunchExerciseName::WeightedCrunchChop,
            9 => CrunchExerciseName::DoubleCrunch,
            10 => CrunchExerciseName::WeightedDoubleCrunch,
            11 => CrunchExerciseName::ElbowToKneeCrunch,
            12 => CrunchExerciseName::WeightedElbowToKneeCrunch,
            13 => CrunchExerciseName::FlutterKicks,
            14 => CrunchExerciseName::WeightedFlutterKicks,
            15 => CrunchExerciseName::FoamRollerReverseCrunchOnBench,
            16 => CrunchExerciseName::WeightedFoamRollerReverseCrunchOnBench,
            17 => CrunchExerciseName::FoamRollerReverseCrunchWithDumbbell,
            18 => CrunchExerciseName::FoamRollerReverseCrunchWithMedicineBall,
            19 => CrunchExerciseName::FrogPress,
            20 => CrunchExerciseName::HangingKneeRaiseObliqueCrunch,
            21 => CrunchExerciseName::WeightedHangingKneeRaiseObliqueCrunch,
            22 => CrunchExerciseName::HipCrossover,
            23 => CrunchExerciseName::WeightedHipCrossover,
            24 => CrunchExerciseName::HollowRock,
            25 => CrunchExerciseName::WeightedHollowRock,
            26 => CrunchExerciseName::InclineReverseCrunch,
            27 => CrunchExerciseName::WeightedInclineReverseCrunch,
            28 => CrunchExerciseName::KneelingCableCrunch,
            29 => CrunchExerciseName::KneelingCrossCrunch,
            30 => CrunchExerciseName::WeightedKneelingCrossCrunch,
            31 => CrunchExerciseName::KneelingObliqueCableCrunch,
            32 => CrunchExerciseName::KneesToElbow,
            33 => CrunchExerciseName::LegExtensions,
            34 => CrunchExerciseName::WeightedLegExtensions,
            35 => CrunchExerciseName::LegLevers,
            36 => CrunchExerciseName::McgillCurlUp,
            37 => CrunchExerciseName::WeightedMcgillCurlUp,
            38 => CrunchExerciseName::ModifiedPilatesRollUpWithBall,
            39 => CrunchExerciseName::WeightedModifiedPilatesRollUpWithBall,
            40 => CrunchExerciseName::PilatesCrunch,
            41 => CrunchExerciseName::WeightedPilatesCrunch,
            42 => CrunchExerciseName::PilatesRollUpWithBall,
            43 => CrunchExerciseName::WeightedPilatesRollUpWithBall,
            44 => CrunchExerciseName::RaisedLegsCrunch,
            45 => CrunchExerciseName::WeightedRaisedLegsCrunch,
            46 => CrunchExerciseName::ReverseCrunch,
            47 => CrunchExerciseName::WeightedReverseCrunch,
            48 => CrunchExerciseName::ReverseCrunchOnABench,
            49 => CrunchExerciseName::WeightedReverseCrunchOnABench,
            50 => CrunchExerciseName::ReverseCurlAndLift,
            51 => CrunchExerciseName::WeightedReverseCurlAndLift,
            52 => CrunchExerciseName::RotationalLift,
            53 => CrunchExerciseName::WeightedRotationalLift,
            54 => CrunchExerciseName::SeatedAlternatingReverseCrunch,
            55 => CrunchExerciseName::WeightedSeatedAlternatingReverseCrunch,
            56 => CrunchExerciseName::SeatedLegU,
            57 => CrunchExerciseName::WeightedSeatedLegU,
            58 => CrunchExerciseName::SideToSideCrunchAndWeave,
            59 => CrunchExerciseName::WeightedSideToSideCrunchAndWeave,
            60 => CrunchExerciseName::SingleLegReverseCrunch,
            61 => CrunchExerciseName::WeightedSingleLegReverseCrunch,
            62 => CrunchExerciseName::SkaterCrunchCross,
            63 => CrunchExerciseName::WeightedSkaterCrunchCross,
            64 => CrunchExerciseName::StandingCableCrunch,
            65 => CrunchExerciseName::StandingSideCrunch,
            66 => CrunchExerciseName::StepClimb,
            67 => CrunchExerciseName::WeightedStepClimb,
            68 => CrunchExerciseName::SwissBallCrunch,
            69 => CrunchExerciseName::SwissBallReverseCrunch,
            70 => CrunchExerciseName::WeightedSwissBallReverseCrunch,
            71 => CrunchExerciseName::SwissBallRussianTwist,
            72 => CrunchExerciseName::WeightedSwissBallRussianTwist,
            73 => CrunchExerciseName::SwissBallSideCrunch,
            74 => CrunchExerciseName::WeightedSwissBallSideCrunch,
            75 => CrunchExerciseName::ThoracicCrunchesOnFoamRoller,
            76 => CrunchExerciseName::WeightedThoracicCrunchesOnFoamRoller,
            77 => CrunchExerciseName::TricepsCrunch,
            78 => CrunchExerciseName::WeightedBicycleCrunch,
            79 => CrunchExerciseName::WeightedCrunch,
            80 => CrunchExerciseName::WeightedSwissBallCrunch,
            81 => CrunchExerciseName::ToesToBar,
            82 => CrunchExerciseName::WeightedToesToBar,
            83 => CrunchExerciseName::Crunch,
            84 => CrunchExerciseName::StraightLegCrunchWithBall,
            n => CrunchExerciseName::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum CurlExerciseName {
    AlternatingDumbbellBicepsCurl,
    AlternatingDumbbellBicepsCurlOnSwissBall,
    AlternatingInclineDumbbellBicepsCurl,
    BarbellBicepsCurl,
    BarbellReverseWristCurl,
    BarbellWristCurl,
    BehindTheBackBarbellReverseWristCurl,
    BehindTheBackOneArmCableCurl,
    CableBicepsCurl,
    CableHammerCurl,
    CheatingBarbellBicepsCurl,
    CloseGripEzBarBicepsCurl,
    CrossBodyDumbbellHammerCurl,
    DeadHangBicepsCurl,
    DeclineHammerCurl,
    DumbbellBicepsCurlWithStaticHold,
    DumbbellHammerCurl,
    DumbbellReverseWristCurl,
    DumbbellWristCurl,
    EzBarPreacherCurl,
    ForwardBendBicepsCurl,
    HammerCurlToPress,
    InclineDumbbellBicepsCurl,
    InclineOffsetThumbDumbbellCurl,
    KettlebellBicepsCurl,
    LyingConcentrationCableCurl,
    OneArmPreacherCurl,
    PlatePinchCurl,
    PreacherCurlWithCable,
    ReverseEzBarCurl,
    ReverseGripBarbellBicepsCurl,
    ReverseGripWristCurl,
    SeatedAlternatingDumbbellBicepsCurl,
    SeatedDumbbellBicepsCurl,
    SeatedReverseDumbbellCurl,
    SplitStanceOffsetPinkyDumbbellCurl,
    StandingAlternatingDumbbellCurls,
    StandingDumbbellBicepsCurl,
    StandingEzBarBicepsCurl,
    StaticCurl,
    SwissBallDumbbellOverheadTricepsExtension,
    SwissBallEzBarPreacherCurl,
    TwistingStandingDumbbellBicepsCurl,
    WideGripEzBarBicepsCurl,
    UnknownValue(u64),
}

impl<N> From<N> for CurlExerciseName
    where N: Into<u64>
{
    fn from(number: N) -> CurlExerciseName {
        match number.into() {
            0 => CurlExerciseName::AlternatingDumbbellBicepsCurl,
            1 => CurlExerciseName::AlternatingDumbbellBicepsCurlOnSwissBall,
            2 => CurlExerciseName::AlternatingInclineDumbbellBicepsCurl,
            3 => CurlExerciseName::BarbellBicepsCurl,
            4 => CurlExerciseName::BarbellReverseWristCurl,
            5 => CurlExerciseName::BarbellWristCurl,
            6 => CurlExerciseName::BehindTheBackBarbellReverseWristCurl,
            7 => CurlExerciseName::BehindTheBackOneArmCableCurl,
            8 => CurlExerciseName::CableBicepsCurl,
            9 => CurlExerciseName::CableHammerCurl,
            10 => CurlExerciseName::CheatingBarbellBicepsCurl,
            11 => CurlExerciseName::CloseGripEzBarBicepsCurl,
            12 => CurlExerciseName::CrossBodyDumbbellHammerCurl,
            13 => CurlExerciseName::DeadHangBicepsCurl,
            14 => CurlExerciseName::DeclineHammerCurl,
            15 => CurlExerciseName::DumbbellBicepsCurlWithStaticHold,
            16 => CurlExerciseName::DumbbellHammerCurl,
            17 => CurlExerciseName::DumbbellReverseWristCurl,
            18 => CurlExerciseName::DumbbellWristCurl,
            19 => CurlExerciseName::EzBarPreacherCurl,
            20 => CurlExerciseName::ForwardBendBicepsCurl,
            21 => CurlExerciseName::HammerCurlToPress,
            22 => CurlExerciseName::InclineDumbbellBicepsCurl,
            23 => CurlExerciseName::InclineOffsetThumbDumbbellCurl,
            24 => CurlExerciseName::KettlebellBicepsCurl,
            25 => CurlExerciseName::LyingConcentrationCableCurl,
            26 => CurlExerciseName::OneArmPreacherCurl,
            27 => CurlExerciseName::PlatePinchCurl,
            28 => CurlExerciseName::PreacherCurlWithCable,
            29 => CurlExerciseName::ReverseEzBarCurl,
            30 => CurlExerciseName::ReverseGripWristCurl,
            31 => CurlExerciseName::ReverseGripBarbellBicepsCurl,
            32 => CurlExerciseName::SeatedAlternatingDumbbellBicepsCurl,
            33 => CurlExerciseName::SeatedDumbbellBicepsCurl,
            34 => CurlExerciseName::SeatedReverseDumbbellCurl,
            35 => CurlExerciseName::SplitStanceOffsetPinkyDumbbellCurl,
            36 => CurlExerciseName::StandingAlternatingDumbbellCurls,
            37 => CurlExerciseName::StandingDumbbellBicepsCurl,
            38 => CurlExerciseName::StandingEzBarBicepsCurl,
            39 => CurlExerciseName::StaticCurl,
            40 => CurlExerciseName::SwissBallDumbbellOverheadTricepsExtension,
            41 => CurlExerciseName::SwissBallEzBarPreacherCurl,
            42 => CurlExerciseName::TwistingStandingDumbbellBicepsCurl,
            43 => CurlExerciseName::WideGripEzBarBicepsCurl,
            n => CurlExerciseName::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum FlyeExerciseName {
    ArmRotations,
    CableCrossover,
    DeclineDumbbellFlye,
    DumbbellFlye,
    HugATree,
    InclineDumbbellFlye,
    KettlebellFlye,
    KneelingRearFlye,
    SingleArmStandingCableReverseFlye,
    SwissBallDumbbellFlye,
    UnknownValue(u64),
}

impl<N> From<N> for FlyeExerciseName
    where N: Into<u64>
{
    fn from(number: N) -> FlyeExerciseName {
        match number.into() {
            0 => FlyeExerciseName::CableCrossover,
            1 => FlyeExerciseName::DeclineDumbbellFlye,
            2 => FlyeExerciseName::DumbbellFlye,
            3 => FlyeExerciseName::InclineDumbbellFlye,
            4 => FlyeExerciseName::KettlebellFlye,
            5 => FlyeExerciseName::KneelingRearFlye,
            6 => FlyeExerciseName::SingleArmStandingCableReverseFlye,
            7 => FlyeExerciseName::SwissBallDumbbellFlye,
            8 => FlyeExerciseName::ArmRotations,
            9 => FlyeExerciseName::HugATree,
            n => FlyeExerciseName::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum HipRaiseExerciseName {
    BarbellHipThrustOnFloor,
    BarbellHipThrustWithBench,
    BentKneeSwissBallReverseHipRaise,
    BridgeWithLegExtension,
    ClamBridge,
    Clams,
    FrontKickTabletop,
    HipExtensionAndCross,
    HipRaise,
    HipRaiseWithFeetOnSwissBall,
    HipRaiseWithHeadOnBosuBall,
    HipRaiseWithHeadOnSwissBall,
    HipRaiseWithKneeSqueeze,
    InclineRearLegExtension,
    InnerThighCircles,
    InnerThighSideLift,
    KettlebellSwing,
    LegCircles,
    LegLift,
    LegLiftInExternalRotation,
    MarchingHipRaise,
    MarchingHipRaiseWithFeetOnASwissBall,
    ReverseHipRaise,
    SingleLegHipRaise,
    SingleLegHipRaiseWithFootOnBench,
    SingleLegHipRaiseWithFootOnBosuBall,
    SingleLegHipRaiseWithFootOnFoamRoller,
    SingleLegHipRaiseWithFootOnMedicineBall,
    SingleLegHipRaiseWithHeadOnBosuBall,
    SingleLegSwissBallHipRaiseAndLegCurl,
    WeightedBentKneeSwissBallReverseHipRaise,
    WeightedBridgeWithLegExtension,
    WeightedClamBridge,
    WeightedFrontKickTabletop,
    WeightedHipExtensionAndCross,
    WeightedHipRaise,
    WeightedHipRaiseWithFeetOnSwissBall,
    WeightedHipRaiseWithHeadOnBosuBall,
    WeightedHipRaiseWithHeadOnSwissBall,
    WeightedHipRaiseWithKneeSqueeze,
    WeightedInclineRearLegExtension,
    WeightedMarchingHipRaise,
    WeightedMarchingHipRaiseWithFeetOnASwissBall,
    WeightedReverseHipRaise,
    WeightedSingleLegHipRaise,
    WeightedSingleLegHipRaiseWithFootOnBench,
    WeightedSingleLegHipRaiseWithFootOnBosuBall,
    WeightedSingleLegHipRaiseWithFootOnFoamRoller,
    WeightedSingleLegHipRaiseWithFootOnMedicineBall,
    WeightedSingleLegHipRaiseWithHeadOnBosuBall,
    UnknownValue(u64),
}

impl<N> From<N> for HipRaiseExerciseName
    where N: Into<u64>
{
    fn from(number: N) -> HipRaiseExerciseName {
        match number.into() {
            0 => HipRaiseExerciseName::BarbellHipThrustOnFloor,
            1 => HipRaiseExerciseName::BarbellHipThrustWithBench,
            2 => HipRaiseExerciseName::BentKneeSwissBallReverseHipRaise,
            3 => HipRaiseExerciseName::WeightedBentKneeSwissBallReverseHipRaise,
            4 => HipRaiseExerciseName::BridgeWithLegExtension,
            5 => HipRaiseExerciseName::WeightedBridgeWithLegExtension,
            6 => HipRaiseExerciseName::ClamBridge,
            7 => HipRaiseExerciseName::FrontKickTabletop,
            8 => HipRaiseExerciseName::WeightedFrontKickTabletop,
            9 => HipRaiseExerciseName::HipExtensionAndCross,
            10 => HipRaiseExerciseName::WeightedHipExtensionAndCross,
            11 => HipRaiseExerciseName::HipRaise,
            12 => HipRaiseExerciseName::WeightedHipRaise,
            13 => HipRaiseExerciseName::HipRaiseWithFeetOnSwissBall,
            14 => HipRaiseExerciseName::WeightedHipRaiseWithFeetOnSwissBall,
            15 => HipRaiseExerciseName::HipRaiseWithHeadOnBosuBall,
            16 => HipRaiseExerciseName::WeightedHipRaiseWithHeadOnBosuBall,
            17 => HipRaiseExerciseName::HipRaiseWithHeadOnSwissBall,
            18 => HipRaiseExerciseName::WeightedHipRaiseWithHeadOnSwissBall,
            19 => HipRaiseExerciseName::HipRaiseWithKneeSqueeze,
            20 => HipRaiseExerciseName::WeightedHipRaiseWithKneeSqueeze,
            21 => HipRaiseExerciseName::InclineRearLegExtension,
            22 => HipRaiseExerciseName::WeightedInclineRearLegExtension,
            23 => HipRaiseExerciseName::KettlebellSwing,
            24 => HipRaiseExerciseName::MarchingHipRaise,
            25 => HipRaiseExerciseName::WeightedMarchingHipRaise,
            26 => HipRaiseExerciseName::MarchingHipRaiseWithFeetOnASwissBall,
            27 => HipRaiseExerciseName::WeightedMarchingHipRaiseWithFeetOnASwissBall,
            28 => HipRaiseExerciseName::ReverseHipRaise,
            29 => HipRaiseExerciseName::WeightedReverseHipRaise,
            30 => HipRaiseExerciseName::SingleLegHipRaise,
            31 => HipRaiseExerciseName::WeightedSingleLegHipRaise,
            32 => HipRaiseExerciseName::SingleLegHipRaiseWithFootOnBench,
            33 => HipRaiseExerciseName::WeightedSingleLegHipRaiseWithFootOnBench,
            34 => HipRaiseExerciseName::SingleLegHipRaiseWithFootOnBosuBall,
            35 => HipRaiseExerciseName::WeightedSingleLegHipRaiseWithFootOnBosuBall,
            36 => HipRaiseExerciseName::SingleLegHipRaiseWithFootOnFoamRoller,
            37 => HipRaiseExerciseName::WeightedSingleLegHipRaiseWithFootOnFoamRoller,
            38 => HipRaiseExerciseName::SingleLegHipRaiseWithFootOnMedicineBall,
            39 => HipRaiseExerciseName::WeightedSingleLegHipRaiseWithFootOnMedicineBall,
            40 => HipRaiseExerciseName::SingleLegHipRaiseWithHeadOnBosuBall,
            41 => HipRaiseExerciseName::WeightedSingleLegHipRaiseWithHeadOnBosuBall,
            42 => HipRaiseExerciseName::WeightedClamBridge,
            43 => HipRaiseExerciseName::SingleLegSwissBallHipRaiseAndLegCurl,
            44 => HipRaiseExerciseName::Clams,
            45 => HipRaiseExerciseName::InnerThighCircles,
            46 => HipRaiseExerciseName::InnerThighSideLift,
            47 => HipRaiseExerciseName::LegCircles,
            48 => HipRaiseExerciseName::LegLift,
            49 => HipRaiseExerciseName::LegLiftInExternalRotation,
            n => HipRaiseExerciseName::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum HipStabilityExerciseName {
    BandSideLyingLegRaise,
    DeadBug,
    ExternalHipRaise,
    FireHydrantKicks,
    HipCircles,
    InnerThighLift,
    LateralWalksWithBandAtAnkles,
    PretzelSideKick,
    ProneHipInternalRotation,
    Quadruped,
    QuadrupedHipExtension,
    QuadrupedWithLegLift,
    SideLyingLegRaise,
    SlidingHipAdduction,
    StandingAdduction,
    StandingCableHipAbduction,
    StandingHipAbduction,
    StandingRearLegRaise,
    SupineHipInternalRotation,
    WeightedDeadBug,
    WeightedExternalHipRaise,
    WeightedFireHydrantKicks,
    WeightedHipCircles,
    WeightedInnerThighLift,
    WeightedPretzelSideKick,
    WeightedProneHipInternalRotation,
    WeightedQuadrupedHipExtension,
    WeightedQuadrupedWithLegLift,
    WeightedSideLyingLegRaise,
    WeightedSlidingHipAdduction,
    WeightedStandingAdduction,
    WeightedStandingHipAbduction,
    WeightedStandingRearLegRaise,
    WeightedSupineHipInternalRotation,
    UnknownValue(u64),
}

impl<N> From<N> for HipStabilityExerciseName
    where N: Into<u64>
{
    fn from(number: N) -> HipStabilityExerciseName {
        match number.into() {
            0 => HipStabilityExerciseName::BandSideLyingLegRaise,
            1 => HipStabilityExerciseName::DeadBug,
            2 => HipStabilityExerciseName::WeightedDeadBug,
            3 => HipStabilityExerciseName::ExternalHipRaise,
            4 => HipStabilityExerciseName::WeightedExternalHipRaise,
            5 => HipStabilityExerciseName::FireHydrantKicks,
            6 => HipStabilityExerciseName::WeightedFireHydrantKicks,
            7 => HipStabilityExerciseName::HipCircles,
            8 => HipStabilityExerciseName::WeightedHipCircles,
            9 => HipStabilityExerciseName::InnerThighLift,
            10 => HipStabilityExerciseName::WeightedInnerThighLift,
            11 => HipStabilityExerciseName::LateralWalksWithBandAtAnkles,
            12 => HipStabilityExerciseName::PretzelSideKick,
            13 => HipStabilityExerciseName::WeightedPretzelSideKick,
            14 => HipStabilityExerciseName::ProneHipInternalRotation,
            15 => HipStabilityExerciseName::WeightedProneHipInternalRotation,
            16 => HipStabilityExerciseName::Quadruped,
            17 => HipStabilityExerciseName::QuadrupedHipExtension,
            18 => HipStabilityExerciseName::WeightedQuadrupedHipExtension,
            19 => HipStabilityExerciseName::QuadrupedWithLegLift,
            20 => HipStabilityExerciseName::WeightedQuadrupedWithLegLift,
            21 => HipStabilityExerciseName::SideLyingLegRaise,
            22 => HipStabilityExerciseName::WeightedSideLyingLegRaise,
            23 => HipStabilityExerciseName::SlidingHipAdduction,
            24 => HipStabilityExerciseName::WeightedSlidingHipAdduction,
            25 => HipStabilityExerciseName::StandingAdduction,
            26 => HipStabilityExerciseName::WeightedStandingAdduction,
            27 => HipStabilityExerciseName::StandingCableHipAbduction,
            28 => HipStabilityExerciseName::StandingHipAbduction,
            29 => HipStabilityExerciseName::WeightedStandingHipAbduction,
            30 => HipStabilityExerciseName::StandingRearLegRaise,
            31 => HipStabilityExerciseName::WeightedStandingRearLegRaise,
            32 => HipStabilityExerciseName::SupineHipInternalRotation,
            33 => HipStabilityExerciseName::WeightedSupineHipInternalRotation,
            n => HipStabilityExerciseName::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum HyperextensionExerciseName {
    BackExtensionWithOppositeArmAndLegReach,
    BaseRotations,
    BentKneeReverseHyperextension,
    Cobra,
    HollowHoldAndRoll,
    Kicks,
    KneeRaises,
    KneelingSuperman,
    LatPullDownWithRow,
    MedicineBallDeadliftToReach,
    OneArmOneLegRow,
    OneArmRowWithBand,
    OverheadLungeWithMedicineBall,
    PlankKneeTucks,
    SideStep,
    SingleLegBackExtension,
    SpineExtension,
    StaticBackExtension,
    SupermanFromFloor,
    SupermanOnSwissBall,
    SupineFloorBarre,
    SwissBallBackExtension,
    SwissBallHyperextension,
    SwissBallOppositeArmAndLegLift,
    WeightedBackExtensionWithOppositeArmAndLegReach,
    WeightedBaseRotations,
    WeightedBentKneeReverseHyperextension,
    WeightedHollowHoldAndRoll,
    WeightedKicks,
    WeightedKneeRaises,
    WeightedKneelingSuperman,
    WeightedPlankKneeTucks,
    WeightedSideStep,
    WeightedSingleLegBackExtension,
    WeightedSpineExtension,
    WeightedStaticBackExtension,
    WeightedSupermanFromFloor,
    WeightedSwissBallBackExtension,
    WeightedSwissBallHyperextension,
    WeightedSwissBallOppositeArmAndLegLift,
    UnknownValue(u64),
}

impl<N> From<N> for HyperextensionExerciseName
    where N: Into<u64>
{
    fn from(number: N) -> HyperextensionExerciseName {
        match number.into() {
            0 => HyperextensionExerciseName::BackExtensionWithOppositeArmAndLegReach,
            1 => HyperextensionExerciseName::WeightedBackExtensionWithOppositeArmAndLegReach,
            2 => HyperextensionExerciseName::BaseRotations,
            3 => HyperextensionExerciseName::WeightedBaseRotations,
            4 => HyperextensionExerciseName::BentKneeReverseHyperextension,
            5 => HyperextensionExerciseName::WeightedBentKneeReverseHyperextension,
            6 => HyperextensionExerciseName::HollowHoldAndRoll,
            7 => HyperextensionExerciseName::WeightedHollowHoldAndRoll,
            8 => HyperextensionExerciseName::Kicks,
            9 => HyperextensionExerciseName::WeightedKicks,
            10 => HyperextensionExerciseName::KneeRaises,
            11 => HyperextensionExerciseName::WeightedKneeRaises,
            12 => HyperextensionExerciseName::KneelingSuperman,
            13 => HyperextensionExerciseName::WeightedKneelingSuperman,
            14 => HyperextensionExerciseName::LatPullDownWithRow,
            15 => HyperextensionExerciseName::MedicineBallDeadliftToReach,
            16 => HyperextensionExerciseName::OneArmOneLegRow,
            17 => HyperextensionExerciseName::OneArmRowWithBand,
            18 => HyperextensionExerciseName::OverheadLungeWithMedicineBall,
            19 => HyperextensionExerciseName::PlankKneeTucks,
            20 => HyperextensionExerciseName::WeightedPlankKneeTucks,
            21 => HyperextensionExerciseName::SideStep,
            22 => HyperextensionExerciseName::WeightedSideStep,
            23 => HyperextensionExerciseName::SingleLegBackExtension,
            24 => HyperextensionExerciseName::WeightedSingleLegBackExtension,
            25 => HyperextensionExerciseName::SpineExtension,
            26 => HyperextensionExerciseName::WeightedSpineExtension,
            27 => HyperextensionExerciseName::StaticBackExtension,
            28 => HyperextensionExerciseName::WeightedStaticBackExtension,
            29 => HyperextensionExerciseName::SupermanFromFloor,
            30 => HyperextensionExerciseName::WeightedSupermanFromFloor,
            31 => HyperextensionExerciseName::SwissBallBackExtension,
            32 => HyperextensionExerciseName::WeightedSwissBallBackExtension,
            33 => HyperextensionExerciseName::SwissBallHyperextension,
            34 => HyperextensionExerciseName::WeightedSwissBallHyperextension,
            35 => HyperextensionExerciseName::SwissBallOppositeArmAndLegLift,
            36 => HyperextensionExerciseName::WeightedSwissBallOppositeArmAndLegLift,
            37 => HyperextensionExerciseName::SupermanOnSwissBall,
            38 => HyperextensionExerciseName::Cobra,
            39 => HyperextensionExerciseName::SupineFloorBarre,
            n => HyperextensionExerciseName::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum LegCurlExerciseName {
    GoodMorning,
    LegCurl,
    SeatedBarbellGoodMorning,
    SingleLegBarbellGoodMorning,
    SingleLegSlidingLegCurl,
    SlidingLegCurl,
    SplitBarbellGoodMorning,
    SplitStanceExtension,
    StaggeredStanceGoodMorning,
    SwissBallHipRaiseAndLegCurl,
    WeightedLegCurl,
    ZercherGoodMorning,
    UnknownValue(u64),
}

impl<N> From<N> for LegCurlExerciseName
    where N: Into<u64>
{
    fn from(number: N) -> LegCurlExerciseName {
        match number.into() {
            0 => LegCurlExerciseName::LegCurl,
            1 => LegCurlExerciseName::WeightedLegCurl,
            2 => LegCurlExerciseName::GoodMorning,
            3 => LegCurlExerciseName::SeatedBarbellGoodMorning,
            4 => LegCurlExerciseName::SingleLegBarbellGoodMorning,
            5 => LegCurlExerciseName::SingleLegSlidingLegCurl,
            6 => LegCurlExerciseName::SlidingLegCurl,
            7 => LegCurlExerciseName::SplitBarbellGoodMorning,
            8 => LegCurlExerciseName::SplitStanceExtension,
            9 => LegCurlExerciseName::StaggeredStanceGoodMorning,
            10 => LegCurlExerciseName::SwissBallHipRaiseAndLegCurl,
            11 => LegCurlExerciseName::ZercherGoodMorning,
            n => LegCurlExerciseName::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum LungeExerciseName {
    AlternatingBarbellForwardLunge,
    AlternatingDumbbellLungeWithReach,
    BackFootElevatedDumbbellSplitSquat,
    BarbellBoxLunge,
    BarbellBulgarianSplitSquat,
    BarbellCrossoverLunge,
    BarbellFrontSplitSquat,
    BarbellLunge,
    BarbellReverseLunge,
    BarbellSideLunge,
    BarbellSplitSquat,
    CoreControlRearLunge,
    DiagonalLunge,
    DropLunge,
    DumbbellBoxLunge,
    DumbbellBulgarianSplitSquat,
    DumbbellCrossoverLunge,
    DumbbellDiagonalLunge,
    DumbbellLunge,
    DumbbellLungeAndRotation,
    DumbbellOverheadBulgarianSplitSquat,
    DumbbellReverseLungeToHighKneeAndPress,
    DumbbellSideLunge,
    ElevatedFrontFootBarbellSplitSquat,
    FrontFootElevatedDumbbellSplitSquat,
    GunslingerLunge,
    LawnmowerLunge,
    LowLungeWithIsometricAdduction,
    LowSideToSideLunge,
    Lunge,
    LungeMatrix,
    LungeWithArmReach,
    LungeWithDiagonalReach,
    LungeWithSideBend,
    OffsetDumbbellLunge,
    OffsetDumbbellReverseLunge,
    OverheadBulgarianSplitSquat,
    OverheadDumbbellReverseLunge,
    OverheadDumbbellSplitSquat,
    OverheadLunge,
    OverheadLungeWithRotation,
    ReverseBarbellBoxLunge,
    ReverseBoxLunge,
    ReverseDumbbellBoxLunge,
    ReverseDumbbellCrossoverLunge,
    ReverseDumbbellDiagonalLunge,
    ReverseLungeWithReachBack,
    ReverseLungeWithTwistAndOverheadReach,
    ReverseSlidingBoxLunge,
    ReverseSlidingLunge,
    RunnersLungeToBalance,
    ShiftingSideLunge,
    SideAndCrossoverLunge,
    SideLunge,
    SideLungeAndPress,
    SideLungeJumpOff,
    SideLungeSweep,
    SideLungeToCrossoverTap,
    SideToSideLungeChops,
    SiffJumpLunge,
    SingleArmReverseLungeAndPress,
    SlidingLateralLunge,
    WalkingBarbellLunge,
    WalkingDumbbellLunge,
    WalkingLunge,
    WeightedLunge,
    WeightedLungeMatrix,
    WeightedReverseLungeWithReachBack,
    WeightedReverseLungeWithTwistAndOverheadReach,
    WeightedReverseSlidingBoxLunge,
    WeightedReverseSlidingLunge,
    WeightedRunnersLungeToBalance,
    WeightedSideAndCrossoverLunge,
    WeightedSideLunge,
    WeightedSideLungeSweep,
    WeightedSideLungeToCrossoverTap,
    WeightedSideToSideLungeChops,
    WeightedSiffJumpLunge,
    WeightedSlidingLateralLunge,
    WeightedWalkingLunge,
    WideGripOverheadBarbellSplitSquat,
    UnknownValue(u64),
}

impl<N> From<N> for LungeExerciseName
    where N: Into<u64>
{
    fn from(number: N) -> LungeExerciseName {
        match number.into() {
            0 => LungeExerciseName::OverheadLunge,
            1 => LungeExerciseName::LungeMatrix,
            2 => LungeExerciseName::WeightedLungeMatrix,
            3 => LungeExerciseName::AlternatingBarbellForwardLunge,
            4 => LungeExerciseName::AlternatingDumbbellLungeWithReach,
            5 => LungeExerciseName::BackFootElevatedDumbbellSplitSquat,
            6 => LungeExerciseName::BarbellBoxLunge,
            7 => LungeExerciseName::BarbellBulgarianSplitSquat,
            8 => LungeExerciseName::BarbellCrossoverLunge,
            9 => LungeExerciseName::BarbellFrontSplitSquat,
            10 => LungeExerciseName::BarbellLunge,
            11 => LungeExerciseName::BarbellReverseLunge,
            12 => LungeExerciseName::BarbellSideLunge,
            13 => LungeExerciseName::BarbellSplitSquat,
            14 => LungeExerciseName::CoreControlRearLunge,
            15 => LungeExerciseName::DiagonalLunge,
            16 => LungeExerciseName::DropLunge,
            17 => LungeExerciseName::DumbbellBoxLunge,
            18 => LungeExerciseName::DumbbellBulgarianSplitSquat,
            19 => LungeExerciseName::DumbbellCrossoverLunge,
            20 => LungeExerciseName::DumbbellDiagonalLunge,
            21 => LungeExerciseName::DumbbellLunge,
            22 => LungeExerciseName::DumbbellLungeAndRotation,
            23 => LungeExerciseName::DumbbellOverheadBulgarianSplitSquat,
            24 => LungeExerciseName::DumbbellReverseLungeToHighKneeAndPress,
            25 => LungeExerciseName::DumbbellSideLunge,
            26 => LungeExerciseName::ElevatedFrontFootBarbellSplitSquat,
            27 => LungeExerciseName::FrontFootElevatedDumbbellSplitSquat,
            28 => LungeExerciseName::GunslingerLunge,
            29 => LungeExerciseName::LawnmowerLunge,
            30 => LungeExerciseName::LowLungeWithIsometricAdduction,
            31 => LungeExerciseName::LowSideToSideLunge,
            32 => LungeExerciseName::Lunge,
            33 => LungeExerciseName::WeightedLunge,
            34 => LungeExerciseName::LungeWithArmReach,
            35 => LungeExerciseName::LungeWithDiagonalReach,
            36 => LungeExerciseName::LungeWithSideBend,
            37 => LungeExerciseName::OffsetDumbbellLunge,
            38 => LungeExerciseName::OffsetDumbbellReverseLunge,
            39 => LungeExerciseName::OverheadBulgarianSplitSquat,
            40 => LungeExerciseName::OverheadDumbbellReverseLunge,
            41 => LungeExerciseName::OverheadDumbbellSplitSquat,
            42 => LungeExerciseName::OverheadLungeWithRotation,
            43 => LungeExerciseName::ReverseBarbellBoxLunge,
            44 => LungeExerciseName::ReverseBoxLunge,
            45 => LungeExerciseName::ReverseDumbbellBoxLunge,
            46 => LungeExerciseName::ReverseDumbbellCrossoverLunge,
            47 => LungeExerciseName::ReverseDumbbellDiagonalLunge,
            48 => LungeExerciseName::ReverseLungeWithReachBack,
            49 => LungeExerciseName::WeightedReverseLungeWithReachBack,
            50 => LungeExerciseName::ReverseLungeWithTwistAndOverheadReach,
            51 => LungeExerciseName::WeightedReverseLungeWithTwistAndOverheadReach,
            52 => LungeExerciseName::ReverseSlidingBoxLunge,
            53 => LungeExerciseName::WeightedReverseSlidingBoxLunge,
            54 => LungeExerciseName::ReverseSlidingLunge,
            55 => LungeExerciseName::WeightedReverseSlidingLunge,
            56 => LungeExerciseName::RunnersLungeToBalance,
            57 => LungeExerciseName::WeightedRunnersLungeToBalance,
            58 => LungeExerciseName::ShiftingSideLunge,
            59 => LungeExerciseName::SideAndCrossoverLunge,
            60 => LungeExerciseName::WeightedSideAndCrossoverLunge,
            61 => LungeExerciseName::SideLunge,
            62 => LungeExerciseName::WeightedSideLunge,
            63 => LungeExerciseName::SideLungeAndPress,
            64 => LungeExerciseName::SideLungeJumpOff,
            65 => LungeExerciseName::SideLungeSweep,
            66 => LungeExerciseName::WeightedSideLungeSweep,
            67 => LungeExerciseName::SideLungeToCrossoverTap,
            68 => LungeExerciseName::WeightedSideLungeToCrossoverTap,
            69 => LungeExerciseName::SideToSideLungeChops,
            70 => LungeExerciseName::WeightedSideToSideLungeChops,
            71 => LungeExerciseName::SiffJumpLunge,
            72 => LungeExerciseName::WeightedSiffJumpLunge,
            73 => LungeExerciseName::SingleArmReverseLungeAndPress,
            74 => LungeExerciseName::SlidingLateralLunge,
            75 => LungeExerciseName::WeightedSlidingLateralLunge,
            76 => LungeExerciseName::WalkingBarbellLunge,
            77 => LungeExerciseName::WalkingDumbbellLunge,
            78 => LungeExerciseName::WalkingLunge,
            79 => LungeExerciseName::WeightedWalkingLunge,
            80 => LungeExerciseName::WideGripOverheadBarbellSplitSquat,
            n => LungeExerciseName::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum PlankExerciseName {
    BearCrawl,
    BridgeOneLegBridge,
    BridgeWithGluteLowerLift,
    CrossBodyMountainClimber,
    ElbowPlankPikeJacks,
    ElevatedFeetPlank,
    ElevatorAbs,
    ExtendedPlank,
    FourtyFiveDegreePlank,
    FullPlankPasseTwist,
    InchingElbowPlank,
    InchwormToSidePlank,
    KneelingPlank,
    KneelingSidePlankWithLegLift,
    LateralRoll,
    LyingReversePlank,
    MedicineBallMountainClimber,
    ModifiedMountainClimberAndExtension,
    MountainClimber,
    MountainClimberOnSlidingDiscs,
    MountainClimberWithFeetOnBosuBall,
    MountainClimberWithHandsOnBench,
    MountainClimberWithHandsOnSwissBall,
    NinetyDegreePlank,
    Plank,
    PlankJacksWithFeetOnSlidingDiscs,
    PlankKneeTwist,
    PlankPikeJumps,
    PlankPikes,
    PlankToStandUp,
    PlankWithArmRaise,
    PlankWithArmVariations,
    PlankWithFeetOnSwissBall,
    PlankWithKneeToElbow,
    PlankWithLegLift,
    PlankWithObliqueCrunch,
    PlyometricSidePlank,
    ReversePlankWithLegPull,
    RollingSidePlank,
    SideKickPlank,
    SidePlank,
    SidePlankAndRow,
    SidePlankLift,
    SidePlankToPlankWithReachUnder,
    SidePlankWithElbowOnBosuBall,
    SidePlankWithFeetOnBench,
    SidePlankWithKneeCircle,
    SidePlankWithKneeTuck,
    SidePlankWithLegLift,
    SidePlankWithReachUnder,
    SingleLegElevatedFeetPlank,
    SingleLegFlexAndExtend,
    SingleLegSidePlank,
    SpidermanPlank,
    StraightArmPlank,
    StraightArmPlankWithShoulderTouch,
    SwissBallPlank,
    SwissBallPlankLegLift,
    SwissBallPlankLegLiftAndHold,
    SwissBallPlankWithFeetOnBench,
    SwissBallProneJackknife,
    SwissBallSidePlank,
    TStabilization,
    ThreeWayPlank,
    TowelPlankAndKneeIn,
    TurkishGetUpToSidePlank,
    TwoPointPlank,
    Weighted45DegreePlank,
    Weighted90DegreeStaticHold,
    WeightedBearCrawl,
    WeightedCrossBodyMountainClimber,
    WeightedElbowPlankPikeJacks,
    WeightedElevatedFeetPlank,
    WeightedElevatorAbs,
    WeightedExtendedPlank,
    WeightedFullPlankPasseTwist,
    WeightedInchingElbowPlank,
    WeightedInchwormToSidePlank,
    WeightedKneelingPlank,
    WeightedKneelingSidePlankWithLegLift,
    WeightedLateralRoll,
    WeightedLyingReversePlank,
    WeightedMedicineBallMountainClimber,
    WeightedModifiedMountainClimberAndExtension,
    WeightedMountainClimber,
    WeightedMountainClimberOnSlidingDiscs,
    WeightedMountainClimberWithFeetOnBosuBall,
    WeightedMountainClimberWithHandsOnBench,
    WeightedMountainClimberWithHandsOnSwissBall,
    WeightedPlank,
    WeightedPlankJacksWithFeetOnSlidingDiscs,
    WeightedPlankKneeTwist,
    WeightedPlankPikeJumps,
    WeightedPlankPikes,
    WeightedPlankToStandUp,
    WeightedPlankWithArmRaise,
    WeightedPlankWithKneeToElbow,
    WeightedPlankWithObliqueCrunch,
    WeightedPlyometricSidePlank,
    WeightedRollingSidePlank,
    WeightedSideKickPlank,
    WeightedSidePlank,
    WeightedSidePlankAndRow,
    WeightedSidePlankLift,
    WeightedSidePlankWithElbowOnBosuBall,
    WeightedSidePlankWithFeetOnBench,
    WeightedSidePlankWithKneeCircle,
    WeightedSidePlankWithKneeTuck,
    WeightedSidePlankWithLegLift,
    WeightedSidePlankWithReachUnder,
    WeightedSingleLegElevatedFeetPlank,
    WeightedSingleLegFlexAndExtend,
    WeightedSingleLegSidePlank,
    WeightedSpidermanPlank,
    WeightedStraightArmPlank,
    WeightedStraightArmPlankWithShoulderTouch,
    WeightedSwissBallPlank,
    WeightedSwissBallPlankLegLift,
    WeightedSwissBallPlankLegLiftAndHold,
    WeightedSwissBallPlankWithFeetOnBench,
    WeightedSwissBallProneJackknife,
    WeightedSwissBallSidePlank,
    WeightedTStabilization,
    WeightedThreeWayPlank,
    WeightedTowelPlankAndKneeIn,
    WeightedTurkishGetUpToSidePlank,
    WeightedTwoPointPlank,
    WeightedWideStancePlankWithDiagonalArmLift,
    WeightedWideStancePlankWithDiagonalLegLift,
    WeightedWideStancePlankWithLegLift,
    WeightedWideStancePlankWithOppositeArmAndLegLift,
    WideStancePlankWithDiagonalArmLift,
    WideStancePlankWithDiagonalLegLift,
    WideStancePlankWithLegLift,
    WideStancePlankWithOppositeArmAndLegLift,
    UnknownValue(u64),
}

impl<N> From<N> for PlankExerciseName
    where N: Into<u64>
{
    fn from(number: N) -> PlankExerciseName {
        match number.into() {
            0 => PlankExerciseName::FourtyFiveDegreePlank,
            1 => PlankExerciseName::Weighted45DegreePlank,
            2 => PlankExerciseName::NinetyDegreePlank,
            3 => PlankExerciseName::Weighted90DegreeStaticHold,
            4 => PlankExerciseName::BearCrawl,
            5 => PlankExerciseName::WeightedBearCrawl,
            6 => PlankExerciseName::CrossBodyMountainClimber,
            7 => PlankExerciseName::WeightedCrossBodyMountainClimber,
            8 => PlankExerciseName::ElbowPlankPikeJacks,
            9 => PlankExerciseName::WeightedElbowPlankPikeJacks,
            10 => PlankExerciseName::ElevatedFeetPlank,
            11 => PlankExerciseName::WeightedElevatedFeetPlank,
            12 => PlankExerciseName::ElevatorAbs,
            13 => PlankExerciseName::WeightedElevatorAbs,
            14 => PlankExerciseName::ExtendedPlank,
            15 => PlankExerciseName::WeightedExtendedPlank,
            16 => PlankExerciseName::FullPlankPasseTwist,
            17 => PlankExerciseName::WeightedFullPlankPasseTwist,
            18 => PlankExerciseName::InchingElbowPlank,
            19 => PlankExerciseName::WeightedInchingElbowPlank,
            20 => PlankExerciseName::InchwormToSidePlank,
            21 => PlankExerciseName::WeightedInchwormToSidePlank,
            22 => PlankExerciseName::KneelingPlank,
            23 => PlankExerciseName::WeightedKneelingPlank,
            24 => PlankExerciseName::KneelingSidePlankWithLegLift,
            25 => PlankExerciseName::WeightedKneelingSidePlankWithLegLift,
            26 => PlankExerciseName::LateralRoll,
            27 => PlankExerciseName::WeightedLateralRoll,
            28 => PlankExerciseName::LyingReversePlank,
            29 => PlankExerciseName::WeightedLyingReversePlank,
            30 => PlankExerciseName::MedicineBallMountainClimber,
            31 => PlankExerciseName::WeightedMedicineBallMountainClimber,
            32 => PlankExerciseName::ModifiedMountainClimberAndExtension,
            33 => PlankExerciseName::WeightedModifiedMountainClimberAndExtension,
            34 => PlankExerciseName::MountainClimber,
            35 => PlankExerciseName::WeightedMountainClimber,
            36 => PlankExerciseName::MountainClimberOnSlidingDiscs,
            37 => PlankExerciseName::WeightedMountainClimberOnSlidingDiscs,
            38 => PlankExerciseName::MountainClimberWithFeetOnBosuBall,
            39 => PlankExerciseName::WeightedMountainClimberWithFeetOnBosuBall,
            40 => PlankExerciseName::MountainClimberWithHandsOnBench,
            41 => PlankExerciseName::MountainClimberWithHandsOnSwissBall,
            42 => PlankExerciseName::WeightedMountainClimberWithHandsOnSwissBall,
            43 => PlankExerciseName::Plank,
            44 => PlankExerciseName::PlankJacksWithFeetOnSlidingDiscs,
            45 => PlankExerciseName::WeightedPlankJacksWithFeetOnSlidingDiscs,
            46 => PlankExerciseName::PlankKneeTwist,
            47 => PlankExerciseName::WeightedPlankKneeTwist,
            48 => PlankExerciseName::PlankPikeJumps,
            49 => PlankExerciseName::WeightedPlankPikeJumps,
            50 => PlankExerciseName::PlankPikes,
            51 => PlankExerciseName::WeightedPlankPikes,
            52 => PlankExerciseName::PlankToStandUp,
            53 => PlankExerciseName::WeightedPlankToStandUp,
            54 => PlankExerciseName::PlankWithArmRaise,
            55 => PlankExerciseName::WeightedPlankWithArmRaise,
            56 => PlankExerciseName::PlankWithKneeToElbow,
            57 => PlankExerciseName::WeightedPlankWithKneeToElbow,
            58 => PlankExerciseName::PlankWithObliqueCrunch,
            59 => PlankExerciseName::WeightedPlankWithObliqueCrunch,
            60 => PlankExerciseName::PlyometricSidePlank,
            61 => PlankExerciseName::WeightedPlyometricSidePlank,
            62 => PlankExerciseName::RollingSidePlank,
            63 => PlankExerciseName::WeightedRollingSidePlank,
            64 => PlankExerciseName::SideKickPlank,
            65 => PlankExerciseName::WeightedSideKickPlank,
            66 => PlankExerciseName::SidePlank,
            67 => PlankExerciseName::WeightedSidePlank,
            68 => PlankExerciseName::SidePlankAndRow,
            69 => PlankExerciseName::WeightedSidePlankAndRow,
            70 => PlankExerciseName::SidePlankLift,
            71 => PlankExerciseName::WeightedSidePlankLift,
            72 => PlankExerciseName::SidePlankWithElbowOnBosuBall,
            73 => PlankExerciseName::WeightedSidePlankWithElbowOnBosuBall,
            74 => PlankExerciseName::SidePlankWithFeetOnBench,
            75 => PlankExerciseName::WeightedSidePlankWithFeetOnBench,
            76 => PlankExerciseName::SidePlankWithKneeCircle,
            77 => PlankExerciseName::WeightedSidePlankWithKneeCircle,
            78 => PlankExerciseName::SidePlankWithKneeTuck,
            79 => PlankExerciseName::WeightedSidePlankWithKneeTuck,
            80 => PlankExerciseName::SidePlankWithLegLift,
            81 => PlankExerciseName::WeightedSidePlankWithLegLift,
            82 => PlankExerciseName::SidePlankWithReachUnder,
            83 => PlankExerciseName::WeightedSidePlankWithReachUnder,
            84 => PlankExerciseName::SingleLegElevatedFeetPlank,
            85 => PlankExerciseName::WeightedSingleLegElevatedFeetPlank,
            86 => PlankExerciseName::SingleLegFlexAndExtend,
            87 => PlankExerciseName::WeightedSingleLegFlexAndExtend,
            88 => PlankExerciseName::SingleLegSidePlank,
            89 => PlankExerciseName::WeightedSingleLegSidePlank,
            90 => PlankExerciseName::SpidermanPlank,
            91 => PlankExerciseName::WeightedSpidermanPlank,
            92 => PlankExerciseName::StraightArmPlank,
            93 => PlankExerciseName::WeightedStraightArmPlank,
            94 => PlankExerciseName::StraightArmPlankWithShoulderTouch,
            95 => PlankExerciseName::WeightedStraightArmPlankWithShoulderTouch,
            96 => PlankExerciseName::SwissBallPlank,
            97 => PlankExerciseName::WeightedSwissBallPlank,
            98 => PlankExerciseName::SwissBallPlankLegLift,
            99 => PlankExerciseName::WeightedSwissBallPlankLegLift,
            100 => PlankExerciseName::SwissBallPlankLegLiftAndHold,
            101 => PlankExerciseName::SwissBallPlankWithFeetOnBench,
            102 => PlankExerciseName::WeightedSwissBallPlankWithFeetOnBench,
            103 => PlankExerciseName::SwissBallProneJackknife,
            104 => PlankExerciseName::WeightedSwissBallProneJackknife,
            105 => PlankExerciseName::SwissBallSidePlank,
            106 => PlankExerciseName::WeightedSwissBallSidePlank,
            107 => PlankExerciseName::ThreeWayPlank,
            108 => PlankExerciseName::WeightedThreeWayPlank,
            109 => PlankExerciseName::TowelPlankAndKneeIn,
            110 => PlankExerciseName::WeightedTowelPlankAndKneeIn,
            111 => PlankExerciseName::TStabilization,
            112 => PlankExerciseName::WeightedTStabilization,
            113 => PlankExerciseName::TurkishGetUpToSidePlank,
            114 => PlankExerciseName::WeightedTurkishGetUpToSidePlank,
            115 => PlankExerciseName::TwoPointPlank,
            116 => PlankExerciseName::WeightedTwoPointPlank,
            117 => PlankExerciseName::WeightedPlank,
            118 => PlankExerciseName::WideStancePlankWithDiagonalArmLift,
            119 => PlankExerciseName::WeightedWideStancePlankWithDiagonalArmLift,
            120 => PlankExerciseName::WideStancePlankWithDiagonalLegLift,
            121 => PlankExerciseName::WeightedWideStancePlankWithDiagonalLegLift,
            122 => PlankExerciseName::WideStancePlankWithLegLift,
            123 => PlankExerciseName::WeightedWideStancePlankWithLegLift,
            124 => PlankExerciseName::WideStancePlankWithOppositeArmAndLegLift,
            125 => PlankExerciseName::WeightedMountainClimberWithHandsOnBench,
            126 => PlankExerciseName::WeightedSwissBallPlankLegLiftAndHold,
            127 => PlankExerciseName::WeightedWideStancePlankWithOppositeArmAndLegLift,
            128 => PlankExerciseName::PlankWithFeetOnSwissBall,
            129 => PlankExerciseName::SidePlankToPlankWithReachUnder,
            130 => PlankExerciseName::BridgeWithGluteLowerLift,
            131 => PlankExerciseName::BridgeOneLegBridge,
            132 => PlankExerciseName::PlankWithArmVariations,
            133 => PlankExerciseName::PlankWithLegLift,
            134 => PlankExerciseName::ReversePlankWithLegPull,
            n => PlankExerciseName::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum PullUpExerciseName {
    BandAssistedChinUp,
    BandedPullUps,
    BurpeePullUp,
    CloseGripChinUp,
    CloseGripLatPulldown,
    CrossoverChinUp,
    EzBarPullover,
    HangingHurdle,
    JumpingPullUps,
    KippingPullUp,
    KneelingLatPulldown,
    KneelingUnderhandGripLatPulldown,
    LPullUp,
    LatPulldown,
    MixedGripChinUp,
    MixedGripPullUp,
    PullUp,
    ReverseGripPulldown,
    StandingCablePullover,
    StraightArmPulldown,
    SuspendedChinUp,
    SwissBallEzBarPullover,
    ThirtyDegreeLatPulldown,
    TowelPullUp,
    WeightedBurpeePullUp,
    WeightedCloseGripChinUp,
    WeightedCrossoverChinUp,
    WeightedHangingHurdle,
    WeightedJumpingPullUps,
    WeightedKippingPullUp,
    WeightedLPullUp,
    WeightedMixedGripChinUp,
    WeightedMixedGripPullUp,
    WeightedPullUp,
    WeightedSuspendedChinUp,
    WeightedTowelPullUp,
    WeightedWideGripPullUp,
    WideGripLatPulldown,
    WideGripPullUp,
    UnknownValue(u64),
}

impl<N> From<N> for PullUpExerciseName
    where N: Into<u64>
{
    fn from(number: N) -> PullUpExerciseName {
        match number.into() {
            0 => PullUpExerciseName::BandedPullUps,
            1 => PullUpExerciseName::ThirtyDegreeLatPulldown,
            2 => PullUpExerciseName::BandAssistedChinUp,
            3 => PullUpExerciseName::CloseGripChinUp,
            4 => PullUpExerciseName::WeightedCloseGripChinUp,
            5 => PullUpExerciseName::CloseGripLatPulldown,
            6 => PullUpExerciseName::CrossoverChinUp,
            7 => PullUpExerciseName::WeightedCrossoverChinUp,
            8 => PullUpExerciseName::EzBarPullover,
            9 => PullUpExerciseName::HangingHurdle,
            10 => PullUpExerciseName::WeightedHangingHurdle,
            11 => PullUpExerciseName::KneelingLatPulldown,
            12 => PullUpExerciseName::KneelingUnderhandGripLatPulldown,
            13 => PullUpExerciseName::LatPulldown,
            14 => PullUpExerciseName::MixedGripChinUp,
            15 => PullUpExerciseName::WeightedMixedGripChinUp,
            16 => PullUpExerciseName::MixedGripPullUp,
            17 => PullUpExerciseName::WeightedMixedGripPullUp,
            18 => PullUpExerciseName::ReverseGripPulldown,
            19 => PullUpExerciseName::StandingCablePullover,
            20 => PullUpExerciseName::StraightArmPulldown,
            21 => PullUpExerciseName::SwissBallEzBarPullover,
            22 => PullUpExerciseName::TowelPullUp,
            23 => PullUpExerciseName::WeightedTowelPullUp,
            24 => PullUpExerciseName::WeightedPullUp,
            25 => PullUpExerciseName::WideGripLatPulldown,
            26 => PullUpExerciseName::WideGripPullUp,
            27 => PullUpExerciseName::WeightedWideGripPullUp,
            28 => PullUpExerciseName::BurpeePullUp,
            29 => PullUpExerciseName::WeightedBurpeePullUp,
            30 => PullUpExerciseName::JumpingPullUps,
            31 => PullUpExerciseName::WeightedJumpingPullUps,
            32 => PullUpExerciseName::KippingPullUp,
            33 => PullUpExerciseName::WeightedKippingPullUp,
            34 => PullUpExerciseName::LPullUp,
            35 => PullUpExerciseName::WeightedLPullUp,
            36 => PullUpExerciseName::SuspendedChinUp,
            37 => PullUpExerciseName::WeightedSuspendedChinUp,
            38 => PullUpExerciseName::PullUp,
            n => PullUpExerciseName::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum RowExerciseName {
    BarbellStraightLegDeadliftToRow,
    CableRowStanding,
    DumbbellRow,
    ElevatedFeetInvertedRow,
    FacePull,
    FacePullWithExternalRotation,
    InvertedRowWithFeetOnSwissBall,
    KettlebellRow,
    ModifiedInvertedRow,
    NeutralGripAlternatingDumbbellRow,
    OneArmBentOverRow,
    OneLeggedDumbbellRow,
    RenegadeRow,
    ReverseGripBarbellRow,
    RopeHandleCableRow,
    SeatedCableRow,
    SeatedDumbbellRow,
    SingleArmCableRow,
    SingleArmCableRowAndRotation,
    SingleArmInvertedRow,
    SingleArmNeutralGripDumbbellRow,
    SingleArmNeutralGripDumbbellRowAndRotation,
    SuspendedInvertedRow,
    TBarRow,
    TowelGripInvertedRow,
    UnderhandGripCableRow,
    VGripCableRow,
    WeightedElevatedFeetInvertedRow,
    WeightedInvertedRowWithFeetOnSwissBall,
    WeightedModifiedInvertedRow,
    WeightedSingleArmInvertedRow,
    WeightedSuspendedInvertedRow,
    WeightedTowelGripInvertedRow,
    WideGripSeatedCableRow,
    UnknownValue(u64),
}

impl<N> From<N> for RowExerciseName
    where N: Into<u64>
{
    fn from(number: N) -> RowExerciseName {
        match number.into() {
            0 => RowExerciseName::BarbellStraightLegDeadliftToRow,
            1 => RowExerciseName::CableRowStanding,
            2 => RowExerciseName::DumbbellRow,
            3 => RowExerciseName::ElevatedFeetInvertedRow,
            4 => RowExerciseName::WeightedElevatedFeetInvertedRow,
            5 => RowExerciseName::FacePull,
            6 => RowExerciseName::FacePullWithExternalRotation,
            7 => RowExerciseName::InvertedRowWithFeetOnSwissBall,
            8 => RowExerciseName::WeightedInvertedRowWithFeetOnSwissBall,
            9 => RowExerciseName::KettlebellRow,
            10 => RowExerciseName::ModifiedInvertedRow,
            11 => RowExerciseName::WeightedModifiedInvertedRow,
            12 => RowExerciseName::NeutralGripAlternatingDumbbellRow,
            13 => RowExerciseName::OneArmBentOverRow,
            14 => RowExerciseName::OneLeggedDumbbellRow,
            15 => RowExerciseName::RenegadeRow,
            16 => RowExerciseName::ReverseGripBarbellRow,
            17 => RowExerciseName::RopeHandleCableRow,
            18 => RowExerciseName::SeatedCableRow,
            19 => RowExerciseName::SeatedDumbbellRow,
            20 => RowExerciseName::SingleArmCableRow,
            21 => RowExerciseName::SingleArmCableRowAndRotation,
            22 => RowExerciseName::SingleArmInvertedRow,
            23 => RowExerciseName::WeightedSingleArmInvertedRow,
            24 => RowExerciseName::SingleArmNeutralGripDumbbellRow,
            25 => RowExerciseName::SingleArmNeutralGripDumbbellRowAndRotation,
            26 => RowExerciseName::SuspendedInvertedRow,
            27 => RowExerciseName::WeightedSuspendedInvertedRow,
            28 => RowExerciseName::TBarRow,
            29 => RowExerciseName::TowelGripInvertedRow,
            30 => RowExerciseName::WeightedTowelGripInvertedRow,
            31 => RowExerciseName::UnderhandGripCableRow,
            32 => RowExerciseName::VGripCableRow,
            33 => RowExerciseName::WideGripSeatedCableRow,
            n => RowExerciseName::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum ShoulderStabilityExerciseName {
    BandExternalRotation,
    BandInternalRotation,
    BentArmLateralRaiseAndExternalRotation,
    CableExternalRotation,
    DumbbellFacePullWithExternalRotation,
    FloorIRaise,
    FloorTRaise,
    FloorYRaise,
    InclineIRaise,
    InclineLRaise,
    InclineTRaise,
    InclineWRaise,
    InclineYRaise,
    LyingExternalRotation,
    NinetyDegreeCableExternalRotation,
    SeatedDumbbellExternalRotation,
    StandingLRaise,
    SwissBallIRaise,
    SwissBallTRaise,
    SwissBallWRaise,
    SwissBallYRaise,
    WeightedFloorIRaise,
    WeightedFloorTRaise,
    WeightedFloorYRaise,
    WeightedInclineIRaise,
    WeightedInclineLRaise,
    WeightedInclineTRaise,
    WeightedInclineWRaise,
    WeightedInclineYRaise,
    WeightedSwissBallIRaise,
    WeightedSwissBallTRaise,
    WeightedSwissBallWRaise,
    WeightedSwissBallYRaise,
    UnknownValue(u64),
}

impl<N> From<N> for ShoulderStabilityExerciseName
    where N: Into<u64>
{
    fn from(number: N) -> ShoulderStabilityExerciseName {
        match number.into() {
            0 => ShoulderStabilityExerciseName::NinetyDegreeCableExternalRotation,
            1 => ShoulderStabilityExerciseName::BandExternalRotation,
            2 => ShoulderStabilityExerciseName::BandInternalRotation,
            3 => ShoulderStabilityExerciseName::BentArmLateralRaiseAndExternalRotation,
            4 => ShoulderStabilityExerciseName::CableExternalRotation,
            5 => ShoulderStabilityExerciseName::DumbbellFacePullWithExternalRotation,
            6 => ShoulderStabilityExerciseName::FloorIRaise,
            7 => ShoulderStabilityExerciseName::WeightedFloorIRaise,
            8 => ShoulderStabilityExerciseName::FloorTRaise,
            9 => ShoulderStabilityExerciseName::WeightedFloorTRaise,
            10 => ShoulderStabilityExerciseName::FloorYRaise,
            11 => ShoulderStabilityExerciseName::WeightedFloorYRaise,
            12 => ShoulderStabilityExerciseName::InclineIRaise,
            13 => ShoulderStabilityExerciseName::WeightedInclineIRaise,
            14 => ShoulderStabilityExerciseName::InclineLRaise,
            15 => ShoulderStabilityExerciseName::WeightedInclineLRaise,
            16 => ShoulderStabilityExerciseName::InclineTRaise,
            17 => ShoulderStabilityExerciseName::WeightedInclineTRaise,
            18 => ShoulderStabilityExerciseName::InclineWRaise,
            19 => ShoulderStabilityExerciseName::WeightedInclineWRaise,
            20 => ShoulderStabilityExerciseName::InclineYRaise,
            21 => ShoulderStabilityExerciseName::WeightedInclineYRaise,
            22 => ShoulderStabilityExerciseName::LyingExternalRotation,
            23 => ShoulderStabilityExerciseName::SeatedDumbbellExternalRotation,
            24 => ShoulderStabilityExerciseName::StandingLRaise,
            25 => ShoulderStabilityExerciseName::SwissBallIRaise,
            26 => ShoulderStabilityExerciseName::WeightedSwissBallIRaise,
            27 => ShoulderStabilityExerciseName::SwissBallTRaise,
            28 => ShoulderStabilityExerciseName::WeightedSwissBallTRaise,
            29 => ShoulderStabilityExerciseName::SwissBallWRaise,
            30 => ShoulderStabilityExerciseName::WeightedSwissBallWRaise,
            31 => ShoulderStabilityExerciseName::SwissBallYRaise,
            32 => ShoulderStabilityExerciseName::WeightedSwissBallYRaise,
            n => ShoulderStabilityExerciseName::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum SitUpExerciseName {
    AlternatingSitUp,
    BentKneeVUp,
    ButterflySitUp,
    CrossPunchRollUp,
    CrossedArmsSitUp,
    GetUpSitUp,
    HoveringSitUp,
    KettlebellSitUp,
    MedicineBallAlternatingVUp,
    MedicineBallSitUp,
    MedicineBallVUp,
    ModifiedSitUp,
    NegativeSitUp,
    OneArmFullSitUp,
    RecliningCircle,
    ReverseCurlUp,
    SingleLegSwissBallJackknife,
    SitUp,
    TheTeaser,
    TheTeaserWeighted,
    ThreePartRollDown,
    VUp,
    WeightedAlternatingSitUp,
    WeightedBentKneeVUp,
    WeightedButterflySitup,
    WeightedCrossPunchRollUp,
    WeightedCrossedArmsSitUp,
    WeightedGetUpSitUp,
    WeightedHoveringSitUp,
    WeightedRecliningCircle,
    WeightedReverseCurlUp,
    WeightedRussianTwistOnSwissBall,
    WeightedSingleLegSwissBallJackknife,
    WeightedSitUp,
    WeightedThreePartRollDown,
    WeightedVUp,
    WeightedXAbs,
    XAbs,
    UnknownValue(u64),
}

impl<N> From<N> for SitUpExerciseName
    where N: Into<u64>
{
    fn from(number: N) -> SitUpExerciseName {
        match number.into() {
            0 => SitUpExerciseName::AlternatingSitUp,
            1 => SitUpExerciseName::WeightedAlternatingSitUp,
            2 => SitUpExerciseName::BentKneeVUp,
            3 => SitUpExerciseName::WeightedBentKneeVUp,
            4 => SitUpExerciseName::ButterflySitUp,
            5 => SitUpExerciseName::WeightedButterflySitup,
            6 => SitUpExerciseName::CrossPunchRollUp,
            7 => SitUpExerciseName::WeightedCrossPunchRollUp,
            8 => SitUpExerciseName::CrossedArmsSitUp,
            9 => SitUpExerciseName::WeightedCrossedArmsSitUp,
            10 => SitUpExerciseName::GetUpSitUp,
            11 => SitUpExerciseName::WeightedGetUpSitUp,
            12 => SitUpExerciseName::HoveringSitUp,
            13 => SitUpExerciseName::WeightedHoveringSitUp,
            14 => SitUpExerciseName::KettlebellSitUp,
            15 => SitUpExerciseName::MedicineBallAlternatingVUp,
            16 => SitUpExerciseName::MedicineBallSitUp,
            17 => SitUpExerciseName::MedicineBallVUp,
            18 => SitUpExerciseName::ModifiedSitUp,
            19 => SitUpExerciseName::NegativeSitUp,
            20 => SitUpExerciseName::OneArmFullSitUp,
            21 => SitUpExerciseName::RecliningCircle,
            22 => SitUpExerciseName::WeightedRecliningCircle,
            23 => SitUpExerciseName::ReverseCurlUp,
            24 => SitUpExerciseName::WeightedReverseCurlUp,
            25 => SitUpExerciseName::SingleLegSwissBallJackknife,
            26 => SitUpExerciseName::WeightedSingleLegSwissBallJackknife,
            27 => SitUpExerciseName::TheTeaser,
            28 => SitUpExerciseName::TheTeaserWeighted,
            29 => SitUpExerciseName::ThreePartRollDown,
            30 => SitUpExerciseName::WeightedThreePartRollDown,
            31 => SitUpExerciseName::VUp,
            32 => SitUpExerciseName::WeightedVUp,
            33 => SitUpExerciseName::WeightedRussianTwistOnSwissBall,
            34 => SitUpExerciseName::WeightedSitUp,
            35 => SitUpExerciseName::XAbs,
            36 => SitUpExerciseName::WeightedXAbs,
            37 => SitUpExerciseName::SitUp,
            n => SitUpExerciseName::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum TotalBodyExerciseName {
    Burpee,
    BurpeeBoxJump,
    HighPullBurpee,
    ManMakers,
    OneArmBurpee,
    SquatPlankPushUp,
    SquatThrusts,
    StandingTRotationBalance,
    WeightedBurpee,
    WeightedBurpeeBoxJump,
    WeightedSquatPlankPushUp,
    WeightedSquatThrusts,
    WeightedStandingTRotationBalance,
    UnknownValue(u64),
}

impl<N> From<N> for TotalBodyExerciseName
    where N: Into<u64>
{
    fn from(number: N) -> TotalBodyExerciseName {
        match number.into() {
            0 => TotalBodyExerciseName::Burpee,
            1 => TotalBodyExerciseName::WeightedBurpee,
            2 => TotalBodyExerciseName::BurpeeBoxJump,
            3 => TotalBodyExerciseName::WeightedBurpeeBoxJump,
            4 => TotalBodyExerciseName::HighPullBurpee,
            5 => TotalBodyExerciseName::ManMakers,
            6 => TotalBodyExerciseName::OneArmBurpee,
            7 => TotalBodyExerciseName::SquatThrusts,
            8 => TotalBodyExerciseName::WeightedSquatThrusts,
            9 => TotalBodyExerciseName::SquatPlankPushUp,
            10 => TotalBodyExerciseName::WeightedSquatPlankPushUp,
            11 => TotalBodyExerciseName::StandingTRotationBalance,
            12 => TotalBodyExerciseName::WeightedStandingTRotationBalance,
            n => TotalBodyExerciseName::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum WarmUpExerciseName {
    AnkleCircles,
    AnkleDorsiflexionWithBand,
    AnkleInternalRotation,
    ArmCircles,
    BentOverReachToSky,
    CatCamel,
    ElbowToFootLunge,
    ForwardAndBackwardLegSwings,
    Groiners,
    InvertedHamstringStretch,
    LateralDuckUnder,
    NeckRotations,
    NeckTilts,
    OppositeArmAndLegBalance,
    QuadrupedRocking,
    ReachRollAndLift,
    Scorpion,
    ShoulderCircles,
    SideToSideLegSwings,
    SleeperStretch,
    SlideOut,
    SwissBallHipCrossover,
    SwissBallReachRollAndLift,
    SwissBallWindshieldWipers,
    ThoracicRotation,
    WalkingHighKicks,
    WalkingHighKnees,
    WalkingKneeHugs,
    WalkingLegCradles,
    Walkout,
    WalkoutFromPushUpPosition,
    UnknownValue(u64),
}

impl<N> From<N> for WarmUpExerciseName
    where N: Into<u64>
{
    fn from(number: N) -> WarmUpExerciseName {
        match number.into() {
            0 => WarmUpExerciseName::QuadrupedRocking,
            1 => WarmUpExerciseName::NeckTilts,
            2 => WarmUpExerciseName::AnkleCircles,
            3 => WarmUpExerciseName::AnkleDorsiflexionWithBand,
            4 => WarmUpExerciseName::AnkleInternalRotation,
            5 => WarmUpExerciseName::ArmCircles,
            6 => WarmUpExerciseName::BentOverReachToSky,
            7 => WarmUpExerciseName::CatCamel,
            8 => WarmUpExerciseName::ElbowToFootLunge,
            9 => WarmUpExerciseName::ForwardAndBackwardLegSwings,
            10 => WarmUpExerciseName::Groiners,
            11 => WarmUpExerciseName::InvertedHamstringStretch,
            12 => WarmUpExerciseName::LateralDuckUnder,
            13 => WarmUpExerciseName::NeckRotations,
            14 => WarmUpExerciseName::OppositeArmAndLegBalance,
            15 => WarmUpExerciseName::ReachRollAndLift,
            16 => WarmUpExerciseName::Scorpion,
            17 => WarmUpExerciseName::ShoulderCircles,
            18 => WarmUpExerciseName::SideToSideLegSwings,
            19 => WarmUpExerciseName::SleeperStretch,
            20 => WarmUpExerciseName::SlideOut,
            21 => WarmUpExerciseName::SwissBallHipCrossover,
            22 => WarmUpExerciseName::SwissBallReachRollAndLift,
            23 => WarmUpExerciseName::SwissBallWindshieldWipers,
            24 => WarmUpExerciseName::ThoracicRotation,
            25 => WarmUpExerciseName::WalkingHighKicks,
            26 => WarmUpExerciseName::WalkingHighKnees,
            27 => WarmUpExerciseName::WalkingKneeHugs,
            28 => WarmUpExerciseName::WalkingLegCradles,
            29 => WarmUpExerciseName::Walkout,
            30 => WarmUpExerciseName::WalkoutFromPushUpPosition,
            n => WarmUpExerciseName::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum WaterType {
    Custom,
    En13319,
    Fresh,
    Salt,
    UnknownValue(u64),
}

impl<N> From<N> for WaterType
    where N: Into<u64>
{
    fn from(number: N) -> WaterType {
        match number.into() {
            0 => WaterType::Fresh,
            1 => WaterType::Salt,
            2 => WaterType::En13319,
            3 => WaterType::Custom,
            n => WaterType::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum DiveGasStatus {
    BackupOnly,
    Disabled,
    Enabled,
    UnknownValue(u64),
}

impl<N> From<N> for DiveGasStatus
    where N: Into<u64>
{
    fn from(number: N) -> DiveGasStatus {
        match number.into() {
            0 => DiveGasStatus::Disabled,
            1 => DiveGasStatus::Enabled,
            2 => DiveGasStatus::BackupOnly,
            n => DiveGasStatus::UnknownValue(n)
        }
    }
}

#[derive(Clone, Debug)]
pub enum DiveBacklightMode {
    AlwaysOn,
    AtDepth,
    UnknownValue(u64),
}

impl<N> From<N> for DiveBacklightMode
    where N: Into<u64>
{
    fn from(number: N) -> DiveBacklightMode {
        match number.into() {
            0 => DiveBacklightMode::AtDepth,
            1 => DiveBacklightMode::AlwaysOn,
            n => DiveBacklightMode::UnknownValue(n)
        }
    }
}

