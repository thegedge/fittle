// DO NOT EDIT -- generated code

use crate::enums;

pub struct FileId {
    Type: enums::File,
    Manufacturer: enums::Manufacturer,
    Product: u16,
    SerialNumber: u32,
    TimeCreated: enums::DateTime,
    Number: u16,
    ProductName: String,
}

pub struct FileCreator {
    SoftwareVersion: u16,
    HardwareVersion: u8,
}

pub struct TimestampCorrelation {
    Timestamp: enums::DateTime,
    FractionalTimestamp: u16,
    SystemTimestamp: enums::DateTime,
    FractionalSystemTimestamp: u16,
    LocalTimestamp: enums::LocalDateTime,
    TimestampMs: u16,
    SystemTimestampMs: u16,
}

pub struct Software {
    MessageIndex: enums::MessageIndex,
    Version: u16,
    PartNumber: String,
}

pub struct SlaveDevice {
    Manufacturer: enums::Manufacturer,
    Product: u16,
}

pub struct Capabilities {
    WorkoutsSupported: enums::WorkoutCapabilities,
    ConnectivitySupported: enums::ConnectivityCapabilities,
}

pub struct FileCapabilities {
    MessageIndex: enums::MessageIndex,
    Type: enums::File,
    Flags: enums::FileFlags,
    Directory: String,
    MaxCount: u16,
    MaxSize: u32,
}

pub struct MesgCapabilities {
    MessageIndex: enums::MessageIndex,
    File: enums::File,
    MesgNum: enums::MesgNum,
    CountType: enums::MesgCount,
    Count: u16,
}

pub struct FieldCapabilities {
    MessageIndex: enums::MessageIndex,
    File: enums::File,
    MesgNum: enums::MesgNum,
    FieldNum: u8,
    Count: u16,
}

pub struct DeviceSettings {
    ActiveTimeZone: u8,
    UtcOffset: u32,
    TimeOffset: u32,
    TimeMode: enums::TimeMode,
    TimeZoneOffset: i8,
    BacklightMode: enums::BacklightMode,
    ActivityTrackerEnabled: bool,
    ClockTime: enums::DateTime,
    PagesEnabled: u16,
    MoveAlertEnabled: bool,
    DateMode: enums::DateMode,
    DisplayOrientation: enums::DisplayOrientation,
    MountingSide: enums::Side,
    DefaultPage: u16,
    AutosyncMinSteps: u16,
    AutosyncMinTime: u16,
    LactateThresholdAutodetectEnabled: bool,
    BleAutoUploadEnabled: bool,
    AutoSyncFrequency: enums::AutoSyncFrequency,
    AutoActivityDetect: enums::AutoActivityDetect,
    NumberOfScreens: u8,
    SmartNotificationDisplayOrientation: enums::DisplayOrientation,
    TapInterface: enums::Switch,
}

pub struct HrmProfile {
    MessageIndex: enums::MessageIndex,
    Enabled: bool,
    HrmAntId: u16,
    LogHrv: bool,
    HrmAntIdTransType: u8,
}

pub struct SdmProfile {
    MessageIndex: enums::MessageIndex,
    Enabled: bool,
    SdmAntId: u16,
    SdmCalFactor: u16,
    Odometer: u32,
    SpeedSource: bool,
    SdmAntIdTransType: u8,
    OdometerRollover: u8,
}

pub struct BikeProfile {
    MessageIndex: enums::MessageIndex,
    Name: String,
    Sport: enums::Sport,
    SubSport: enums::SubSport,
    Odometer: u32,
    BikeSpdAntId: u16,
    BikeCadAntId: u16,
    BikeSpdcadAntId: u16,
    BikePowerAntId: u16,
    CustomWheelsize: u16,
    AutoWheelsize: u16,
    BikeWeight: u16,
    PowerCalFactor: u16,
    AutoWheelCal: bool,
    AutoPowerZero: bool,
    Id: u8,
    SpdEnabled: bool,
    CadEnabled: bool,
    SpdcadEnabled: bool,
    PowerEnabled: bool,
    CrankLength: u8,
    Enabled: bool,
    BikeSpdAntIdTransType: u8,
    BikeCadAntIdTransType: u8,
    BikeSpdcadAntIdTransType: u8,
    BikePowerAntIdTransType: u8,
    OdometerRollover: u8,
    FrontGearNum: u8,
    FrontGear: u8,
    RearGearNum: u8,
    RearGear: u8,
    ShimanoDi2Enabled: bool,
}

pub struct Connectivity {
    BluetoothEnabled: bool,
    BluetoothLeEnabled: bool,
    AntEnabled: bool,
    Name: String,
    LiveTrackingEnabled: bool,
    WeatherConditionsEnabled: bool,
    WeatherAlertsEnabled: bool,
    AutoActivityUploadEnabled: bool,
    CourseDownloadEnabled: bool,
    WorkoutDownloadEnabled: bool,
    GpsEphemerisDownloadEnabled: bool,
    IncidentDetectionEnabled: bool,
    GrouptrackEnabled: bool,
}

pub struct OhrSettings {
    Timestamp: enums::DateTime,
    Enabled: enums::Switch,
}

pub struct ZonesTarget {
    MaxHeartRate: u8,
    ThresholdHeartRate: u8,
    FunctionalThresholdPower: u16,
    HrCalcType: enums::HrZoneCalc,
    PwrCalcType: enums::PwrZoneCalc,
}

pub struct Sport {
    Sport: enums::Sport,
    SubSport: enums::SubSport,
    Name: String,
}

pub struct HrZone {
    MessageIndex: enums::MessageIndex,
    HighBpm: u8,
    Name: String,
}

pub struct SpeedZone {
    MessageIndex: enums::MessageIndex,
    HighValue: u16,
    Name: String,
}

pub struct CadenceZone {
    MessageIndex: enums::MessageIndex,
    HighValue: u8,
    Name: String,
}

pub struct PowerZone {
    MessageIndex: enums::MessageIndex,
    HighValue: u16,
    Name: String,
}

pub struct MetZone {
    MessageIndex: enums::MessageIndex,
    HighBpm: u8,
    Calories: u16,
    FatCalories: u8,
}

pub struct DiveSettings {
    MessageIndex: enums::MessageIndex,
    Name: String,
    Model: enums::TissueModelType,
    GfLow: u8,
    GfHigh: u8,
    WaterType: enums::WaterType,
    WaterDensity: f32,
    Po2Warn: u8,
    Po2Critical: u8,
    Po2Deco: u8,
    SafetyStopEnabled: bool,
    BottomDepth: f32,
    BottomTime: u32,
    ApneaCountdownEnabled: bool,
    ApneaCountdownTime: u32,
    BacklightMode: enums::DiveBacklightMode,
    BacklightBrightness: u8,
    BacklightTimeout: enums::BacklightTimeout,
    RepeatDiveInterval: u16,
    SafetyStopTime: u16,
    HeartRateSourceType: enums::SourceType,
    HeartRateSource: u8,
}

pub struct DiveGas {
    MessageIndex: enums::MessageIndex,
    HeliumContent: u8,
    OxygenContent: u8,
    Status: enums::DiveGasStatus,
}

pub struct Goal {
    MessageIndex: enums::MessageIndex,
    Sport: enums::Sport,
    SubSport: enums::SubSport,
    StartDate: enums::DateTime,
    EndDate: enums::DateTime,
    Type: enums::Goal,
    Value: u32,
    Repeat: bool,
    TargetValue: u32,
    Recurrence: enums::GoalRecurrence,
    RecurrenceValue: u16,
    Enabled: bool,
    Source: enums::GoalSource,
}

pub struct Activity {
    Timestamp: enums::DateTime,
    TotalTimerTime: u32,
    NumSessions: u16,
    Type: enums::Activity,
    Event: enums::Event,
    EventType: enums::EventType,
    LocalTimestamp: enums::LocalDateTime,
    EventGroup: u8,
}

pub struct Session {
    MessageIndex: enums::MessageIndex,
    Timestamp: enums::DateTime,
    Event: enums::Event,
    EventType: enums::EventType,
    StartTime: enums::DateTime,
    StartPositionLat: i32,
    StartPositionLong: i32,
    Sport: enums::Sport,
    SubSport: enums::SubSport,
    TotalElapsedTime: u32,
    TotalTimerTime: u32,
    TotalDistance: u32,
    TotalCycles: u32,
    TotalCalories: u16,
    TotalFatCalories: u16,
    AvgSpeed: u16,
    MaxSpeed: u16,
    AvgHeartRate: u8,
    MaxHeartRate: u8,
    AvgCadence: u8,
    MaxCadence: u8,
    AvgPower: u16,
    MaxPower: u16,
    TotalAscent: u16,
    TotalDescent: u16,
    TotalTrainingEffect: u8,
    FirstLapIndex: u16,
    NumLaps: u16,
    EventGroup: u8,
    Trigger: enums::SessionTrigger,
    NecLat: i32,
    NecLong: i32,
    SwcLat: i32,
    SwcLong: i32,
    NormalizedPower: u16,
    TrainingStressScore: u16,
    IntensityFactor: u16,
    LeftRightBalance: enums::LeftRightBalance100,
    AvgStrokeCount: u32,
    AvgStrokeDistance: u16,
    SwimStroke: enums::SwimStroke,
    PoolLength: u16,
    ThresholdPower: u16,
    PoolLengthUnit: enums::DisplayMeasure,
    NumActiveLengths: u16,
    TotalWork: u32,
    AvgAltitude: u16,
    MaxAltitude: u16,
    GpsAccuracy: u8,
    AvgGrade: i16,
    AvgPosGrade: i16,
    AvgNegGrade: i16,
    MaxPosGrade: i16,
    MaxNegGrade: i16,
    AvgTemperature: i8,
    MaxTemperature: i8,
    TotalMovingTime: u32,
    AvgPosVerticalSpeed: i16,
    AvgNegVerticalSpeed: i16,
    MaxPosVerticalSpeed: i16,
    MaxNegVerticalSpeed: i16,
    MinHeartRate: u8,
    TimeInHrZone: u32,
    TimeInSpeedZone: u32,
    TimeInCadenceZone: u32,
    TimeInPowerZone: u32,
    AvgLapTime: u32,
    BestLapIndex: u16,
    MinAltitude: u16,
    PlayerScore: u16,
    OpponentScore: u16,
    OpponentName: String,
    StrokeCount: u16,
    ZoneCount: u16,
    MaxBallSpeed: u16,
    AvgBallSpeed: u16,
    AvgVerticalOscillation: u16,
    AvgStanceTimePercent: u16,
    AvgStanceTime: u16,
    AvgFractionalCadence: u8,
    MaxFractionalCadence: u8,
    TotalFractionalCycles: u8,
    AvgTotalHemoglobinConc: u16,
    MinTotalHemoglobinConc: u16,
    MaxTotalHemoglobinConc: u16,
    AvgSaturatedHemoglobinPercent: u16,
    MinSaturatedHemoglobinPercent: u16,
    MaxSaturatedHemoglobinPercent: u16,
    AvgLeftTorqueEffectiveness: u8,
    AvgRightTorqueEffectiveness: u8,
    AvgLeftPedalSmoothness: u8,
    AvgRightPedalSmoothness: u8,
    AvgCombinedPedalSmoothness: u8,
    SportIndex: u8,
    TimeStanding: u32,
    StandCount: u16,
    AvgLeftPco: i8,
    AvgRightPco: i8,
    AvgLeftPowerPhase: u8,
    AvgLeftPowerPhasePeak: u8,
    AvgRightPowerPhase: u8,
    AvgRightPowerPhasePeak: u8,
    AvgPowerPosition: u16,
    MaxPowerPosition: u16,
    AvgCadencePosition: u8,
    MaxCadencePosition: u8,
    EnhancedAvgSpeed: u32,
    EnhancedMaxSpeed: u32,
    EnhancedAvgAltitude: u32,
    EnhancedMinAltitude: u32,
    EnhancedMaxAltitude: u32,
    AvgLevMotorPower: u16,
    MaxLevMotorPower: u16,
    LevBatteryConsumption: u8,
    AvgVerticalRatio: u16,
    AvgStanceTimeBalance: u16,
    AvgStepLength: u16,
    TotalAnaerobicTrainingEffect: u8,
    AvgVam: u16,
}

pub struct Length {
    MessageIndex: enums::MessageIndex,
    Timestamp: enums::DateTime,
    Event: enums::Event,
    EventType: enums::EventType,
    StartTime: enums::DateTime,
    TotalElapsedTime: u32,
    TotalTimerTime: u32,
    TotalStrokes: u16,
    AvgSpeed: u16,
    SwimStroke: enums::SwimStroke,
    AvgSwimmingCadence: u8,
    EventGroup: u8,
    TotalCalories: u16,
    LengthType: enums::LengthType,
    PlayerScore: u16,
    OpponentScore: u16,
    StrokeCount: u16,
    ZoneCount: u16,
}

pub struct Record {
    Timestamp: enums::DateTime,
    PositionLat: i32,
    PositionLong: i32,
    Altitude: u16,
    HeartRate: u8,
    Cadence: u8,
    Distance: u32,
    Speed: u16,
    Power: u16,
    CompressedSpeedDistance: u8,
    Grade: i16,
    Resistance: u8,
    TimeFromCourse: i32,
    CycleLength: u8,
    Temperature: i8,
    Speed1S: u8,
    Cycles: u8,
    TotalCycles: u32,
    CompressedAccumulatedPower: u16,
    AccumulatedPower: u32,
    LeftRightBalance: enums::LeftRightBalance,
    GpsAccuracy: u8,
    VerticalSpeed: i16,
    Calories: u16,
    VerticalOscillation: u16,
    StanceTimePercent: u16,
    StanceTime: u16,
    ActivityType: enums::ActivityType,
    LeftTorqueEffectiveness: u8,
    RightTorqueEffectiveness: u8,
    LeftPedalSmoothness: u8,
    RightPedalSmoothness: u8,
    CombinedPedalSmoothness: u8,
    Time128: u8,
    StrokeType: enums::StrokeType,
    Zone: u8,
    BallSpeed: u16,
    Cadence256: u16,
    FractionalCadence: u8,
    TotalHemoglobinConc: u16,
    TotalHemoglobinConcMin: u16,
    TotalHemoglobinConcMax: u16,
    SaturatedHemoglobinPercent: u16,
    SaturatedHemoglobinPercentMin: u16,
    SaturatedHemoglobinPercentMax: u16,
    DeviceIndex: enums::DeviceIndex,
    LeftPco: i8,
    RightPco: i8,
    LeftPowerPhase: u8,
    LeftPowerPhasePeak: u8,
    RightPowerPhase: u8,
    RightPowerPhasePeak: u8,
    EnhancedSpeed: u32,
    EnhancedAltitude: u32,
    BatterySoc: u8,
    MotorPower: u16,
    VerticalRatio: u16,
    StanceTimeBalance: u16,
    StepLength: u16,
    AbsolutePressure: u32,
    Depth: u32,
    NextStopDepth: u32,
    NextStopTime: u32,
    TimeToSurface: u32,
    NdlTime: u32,
    CnsLoad: u8,
    N2Load: u16,
}

pub struct Event {
    Timestamp: enums::DateTime,
    Event: enums::Event,
    EventType: enums::EventType,
    Data16: u16,
    Data: u32,
    EventGroup: u8,
    Score: u16,
    OpponentScore: u16,
    FrontGearNum: u8,
    FrontGear: u8,
    RearGearNum: u8,
    RearGear: u8,
    DeviceIndex: enums::DeviceIndex,
}

pub struct TrainingFile {
    Timestamp: enums::DateTime,
    Type: enums::File,
    Manufacturer: enums::Manufacturer,
    Product: u16,
    SerialNumber: u32,
    TimeCreated: enums::DateTime,
}

pub struct Hrv {
    Time: u16,
}

pub struct WeatherConditions {
    Timestamp: enums::DateTime,
    WeatherReport: enums::WeatherReport,
    Temperature: i8,
    Condition: enums::WeatherStatus,
    WindDirection: u16,
    WindSpeed: u16,
    PrecipitationProbability: u8,
    TemperatureFeelsLike: i8,
    RelativeHumidity: u8,
    Location: String,
    ObservedAtTime: enums::DateTime,
    ObservedLocationLat: i32,
    ObservedLocationLong: i32,
    DayOfWeek: enums::DayOfWeek,
    HighTemperature: i8,
    LowTemperature: i8,
}

pub struct WeatherAlert {
    Timestamp: enums::DateTime,
    ReportId: String,
    IssueTime: enums::DateTime,
    ExpireTime: enums::DateTime,
    Severity: enums::WeatherSeverity,
    Type: enums::WeatherSevereType,
}

pub struct CameraEvent {
    Timestamp: enums::DateTime,
    TimestampMs: u16,
    CameraEventType: enums::CameraEventType,
    CameraFileUuid: String,
    CameraOrientation: enums::CameraOrientationType,
}

pub struct GyroscopeData {
    Timestamp: enums::DateTime,
    TimestampMs: u16,
    SampleTimeOffset: u16,
    GyroX: u16,
    GyroY: u16,
    GyroZ: u16,
    CalibratedGyroX: f32,
    CalibratedGyroY: f32,
    CalibratedGyroZ: f32,
}

pub struct AccelerometerData {
    Timestamp: enums::DateTime,
    TimestampMs: u16,
    SampleTimeOffset: u16,
    AccelX: u16,
    AccelY: u16,
    AccelZ: u16,
    CalibratedAccelX: f32,
    CalibratedAccelY: f32,
    CalibratedAccelZ: f32,
    CompressedCalibratedAccelX: i16,
    CompressedCalibratedAccelY: i16,
    CompressedCalibratedAccelZ: i16,
}

pub struct MagnetometerData {
    Timestamp: enums::DateTime,
    TimestampMs: u16,
    SampleTimeOffset: u16,
    MagX: u16,
    MagY: u16,
    MagZ: u16,
    CalibratedMagX: f32,
    CalibratedMagY: f32,
    CalibratedMagZ: f32,
}

pub struct BarometerData {
    Timestamp: enums::DateTime,
    TimestampMs: u16,
    SampleTimeOffset: u16,
    BaroPres: u32,
}

pub struct ThreeDSensorCalibration {
    Timestamp: enums::DateTime,
    SensorType: enums::SensorType,
    CalibrationFactor: u32,
    CalibrationDivisor: u32,
    LevelShift: u32,
    OffsetCal: i32,
    OrientationMatrix: i32,
}

pub struct OneDSensorCalibration {
    Timestamp: enums::DateTime,
    SensorType: enums::SensorType,
    CalibrationFactor: u32,
    CalibrationDivisor: u32,
    LevelShift: u32,
    OffsetCal: i32,
}

pub struct VideoFrame {
    Timestamp: enums::DateTime,
    TimestampMs: u16,
    FrameNumber: u32,
}

pub struct ObdiiData {
    Timestamp: enums::DateTime,
    TimestampMs: u16,
    TimeOffset: u16,
    Pid: u8,
    RawData: u8,
    PidDataSize: u8,
    SystemTime: u32,
    StartTimestamp: enums::DateTime,
    StartTimestampMs: u16,
}

pub struct NmeaSentence {
    Timestamp: enums::DateTime,
    TimestampMs: u16,
    Sentence: String,
}

pub struct AviationAttitude {
    Timestamp: enums::DateTime,
    TimestampMs: u16,
    SystemTime: u32,
    Pitch: i16,
    Roll: i16,
    AccelLateral: i16,
    AccelNormal: i16,
    TurnRate: i16,
    Stage: enums::AttitudeStage,
    AttitudeStageComplete: u8,
    Track: u16,
    Validity: enums::AttitudeValidity,
}

pub struct Video {
    Url: String,
    HostingProvider: String,
    Duration: u32,
}

pub struct VideoTitle {
    MessageIndex: enums::MessageIndex,
    MessageCount: u16,
    Text: String,
}

pub struct VideoDescription {
    MessageIndex: enums::MessageIndex,
    MessageCount: u16,
    Text: String,
}

pub struct VideoClip {
    ClipNumber: u16,
    StartTimestamp: enums::DateTime,
    StartTimestampMs: u16,
    EndTimestamp: enums::DateTime,
    EndTimestampMs: u16,
    ClipStart: u32,
    ClipEnd: u32,
}

pub struct Set {
    Timestamp: enums::DateTime,
    Duration: u32,
    Repetitions: u16,
    Weight: u16,
    SetType: enums::SetType,
    StartTime: enums::DateTime,
    Category: enums::ExerciseCategory,
    CategorySubtype: u16,
    WeightDisplayUnit: enums::FitBaseUnit,
    MessageIndex: enums::MessageIndex,
    WktStepIndex: enums::MessageIndex,
}

pub struct Course {
    Sport: enums::Sport,
    Name: String,
    Capabilities: enums::CourseCapabilities,
    SubSport: enums::SubSport,
}

pub struct CoursePoint {
    MessageIndex: enums::MessageIndex,
    Timestamp: enums::DateTime,
    PositionLat: i32,
    PositionLong: i32,
    Distance: u32,
    Type: enums::CoursePoint,
    Name: String,
    Favorite: bool,
}

pub struct SegmentId {
    Name: String,
    Uuid: String,
    Sport: enums::Sport,
    Enabled: bool,
    UserProfilePrimaryKey: u32,
    DeviceId: u32,
    DefaultRaceLeader: u8,
    DeleteStatus: enums::SegmentDeleteStatus,
    SelectionType: enums::SegmentSelectionType,
}

pub struct SegmentLeaderboardEntry {
    MessageIndex: enums::MessageIndex,
    Name: String,
    Type: enums::SegmentLeaderboardType,
    GroupPrimaryKey: u32,
    ActivityId: u32,
    SegmentTime: u32,
    ActivityIdString: String,
}

pub struct SegmentPoint {
    MessageIndex: enums::MessageIndex,
    PositionLat: i32,
    PositionLong: i32,
    Distance: u32,
    Altitude: u16,
    LeaderTime: u32,
}

pub struct SegmentLap {
    MessageIndex: enums::MessageIndex,
    Timestamp: enums::DateTime,
    Event: enums::Event,
    EventType: enums::EventType,
    StartTime: enums::DateTime,
    StartPositionLat: i32,
    StartPositionLong: i32,
    EndPositionLat: i32,
    EndPositionLong: i32,
    TotalElapsedTime: u32,
    TotalTimerTime: u32,
    TotalDistance: u32,
    TotalCycles: u32,
    TotalCalories: u16,
    TotalFatCalories: u16,
    AvgSpeed: u16,
    MaxSpeed: u16,
    AvgHeartRate: u8,
    MaxHeartRate: u8,
    AvgCadence: u8,
    MaxCadence: u8,
    AvgPower: u16,
    MaxPower: u16,
    TotalAscent: u16,
    TotalDescent: u16,
    Sport: enums::Sport,
    EventGroup: u8,
    NecLat: i32,
    NecLong: i32,
    SwcLat: i32,
    SwcLong: i32,
    Name: String,
    NormalizedPower: u16,
    LeftRightBalance: enums::LeftRightBalance100,
    SubSport: enums::SubSport,
    TotalWork: u32,
    AvgAltitude: u16,
    MaxAltitude: u16,
    GpsAccuracy: u8,
    AvgGrade: i16,
    AvgPosGrade: i16,
    AvgNegGrade: i16,
    MaxPosGrade: i16,
    MaxNegGrade: i16,
    AvgTemperature: i8,
    MaxTemperature: i8,
    TotalMovingTime: u32,
    AvgPosVerticalSpeed: i16,
    AvgNegVerticalSpeed: i16,
    MaxPosVerticalSpeed: i16,
    MaxNegVerticalSpeed: i16,
    TimeInHrZone: u32,
    TimeInSpeedZone: u32,
    TimeInCadenceZone: u32,
    TimeInPowerZone: u32,
    RepetitionNum: u16,
    MinAltitude: u16,
    MinHeartRate: u8,
    ActiveTime: u32,
    WktStepIndex: enums::MessageIndex,
    SportEvent: enums::SportEvent,
    AvgLeftTorqueEffectiveness: u8,
    AvgRightTorqueEffectiveness: u8,
    AvgLeftPedalSmoothness: u8,
    AvgRightPedalSmoothness: u8,
    AvgCombinedPedalSmoothness: u8,
    Status: enums::SegmentLapStatus,
    Uuid: String,
    AvgFractionalCadence: u8,
    MaxFractionalCadence: u8,
    TotalFractionalCycles: u8,
    FrontGearShiftCount: u16,
    RearGearShiftCount: u16,
    TimeStanding: u32,
    StandCount: u16,
    AvgLeftPco: i8,
    AvgRightPco: i8,
    AvgLeftPowerPhase: u8,
    AvgLeftPowerPhasePeak: u8,
    AvgRightPowerPhase: u8,
    AvgRightPowerPhasePeak: u8,
    AvgPowerPosition: u16,
    MaxPowerPosition: u16,
    AvgCadencePosition: u8,
    MaxCadencePosition: u8,
    Manufacturer: enums::Manufacturer,
}

pub struct SegmentFile {
    MessageIndex: enums::MessageIndex,
    FileUuid: String,
    Enabled: bool,
    UserProfilePrimaryKey: u32,
    LeaderType: enums::SegmentLeaderboardType,
    LeaderGroupPrimaryKey: u32,
    LeaderActivityId: u32,
    LeaderActivityIdString: String,
    DefaultRaceLeader: u8,
}

pub struct Workout {
    Sport: enums::Sport,
    Capabilities: enums::WorkoutCapabilities,
    NumValidSteps: u16,
    WktName: String,
    SubSport: enums::SubSport,
    PoolLength: u16,
    PoolLengthUnit: enums::DisplayMeasure,
}

pub struct WorkoutSession {
    MessageIndex: enums::MessageIndex,
    Sport: enums::Sport,
    SubSport: enums::SubSport,
    NumValidSteps: u16,
    FirstStepIndex: u16,
    PoolLength: u16,
    PoolLengthUnit: enums::DisplayMeasure,
}

pub struct ExerciseTitle {
    MessageIndex: enums::MessageIndex,
    ExerciseCategory: enums::ExerciseCategory,
    ExerciseName: u16,
    WktStepName: String,
}

pub struct Schedule {
    Manufacturer: enums::Manufacturer,
    Product: u16,
    SerialNumber: u32,
    TimeCreated: enums::DateTime,
    Completed: bool,
    Type: enums::Schedule,
    ScheduledTime: enums::LocalDateTime,
}

pub struct Totals {
    MessageIndex: enums::MessageIndex,
    Timestamp: enums::DateTime,
    TimerTime: u32,
    Distance: u32,
    Calories: u32,
    Sport: enums::Sport,
    ElapsedTime: u32,
    Sessions: u16,
    ActiveTime: u32,
    SportIndex: u8,
}

pub struct WeightScale {
    Timestamp: enums::DateTime,
    Weight: enums::Weight,
    PercentFat: u16,
    PercentHydration: u16,
    VisceralFatMass: u16,
    BoneMass: u16,
    MuscleMass: u16,
    BasalMet: u16,
    PhysiqueRating: u8,
    ActiveMet: u16,
    MetabolicAge: u8,
    VisceralFatRating: u8,
    UserProfileIndex: enums::MessageIndex,
}

pub struct BloodPressure {
    Timestamp: enums::DateTime,
    SystolicPressure: u16,
    DiastolicPressure: u16,
    MeanArterialPressure: u16,
    Map3SampleMean: u16,
    MapMorningValues: u16,
    MapEveningValues: u16,
    HeartRate: u8,
    HeartRateType: enums::HrType,
    Status: enums::BpStatus,
    UserProfileIndex: enums::MessageIndex,
}

pub struct MonitoringInfo {
    Timestamp: enums::DateTime,
    LocalTimestamp: enums::LocalDateTime,
    ActivityType: enums::ActivityType,
    CyclesToDistance: u16,
    CyclesToCalories: u16,
    RestingMetabolicRate: u16,
}

pub struct Monitoring {
    Timestamp: enums::DateTime,
    DeviceIndex: enums::DeviceIndex,
    Calories: u16,
    Distance: u32,
    Cycles: u32,
    ActiveTime: u32,
    ActivityType: enums::ActivityType,
    ActivitySubtype: enums::ActivitySubtype,
    ActivityLevel: enums::ActivityLevel,
    Distance16: u16,
    Cycles16: u16,
    ActiveTime16: u16,
    LocalTimestamp: enums::LocalDateTime,
    Temperature: i16,
    TemperatureMin: i16,
    TemperatureMax: i16,
    ActivityTime: u16,
    ActiveCalories: u16,
    CurrentActivityTypeIntensity: u8,
    TimestampMin8: u8,
    Timestamp16: u16,
    HeartRate: u8,
    Intensity: u8,
    DurationMin: u16,
    Duration: u32,
    Ascent: u32,
    Descent: u32,
    ModerateActivityMinutes: u16,
    VigorousActivityMinutes: u16,
}

pub struct Hr {
    Timestamp: enums::DateTime,
    FractionalTimestamp: u16,
    Time256: u8,
    FilteredBpm: u8,
    EventTimestamp: u32,
    EventTimestamp12: u8,
}

pub struct MemoGlob {
    PartIndex: u32,
    Memo: u8,
    MessageNumber: u16,
    MessageIndex: enums::MessageIndex,
}

pub struct AntChannelId {
    ChannelNumber: u8,
    DeviceType: u8,
    DeviceNumber: u16,
    TransmissionType: u8,
    DeviceIndex: enums::DeviceIndex,
}

pub struct AntTx {
    Timestamp: enums::DateTime,
    FractionalTimestamp: u16,
    MesgId: u8,
    MesgData: u8,
    ChannelNumber: u8,
    Data: u8,
}

pub struct ExdDataFieldConfiguration {
    ScreenIndex: u8,
    ConceptField: u8,
    FieldId: u8,
    ConceptCount: u8,
    DisplayType: enums::ExdDisplayType,
    Title: String,
}

pub struct ExdDataConceptConfiguration {
    ScreenIndex: u8,
    ConceptField: u8,
    FieldId: u8,
    ConceptIndex: u8,
    DataPage: u8,
    ConceptKey: u8,
    Scaling: u8,
    DataUnits: enums::ExdDataUnits,
    Qualifier: enums::ExdQualifiers,
    Descriptor: enums::ExdDescriptors,
    IsSigned: bool,
}

pub struct DeveloperDataId {
    DeveloperId: u8,
    ApplicationId: u8,
    ManufacturerId: enums::Manufacturer,
    DeveloperDataIndex: u8,
    ApplicationVersion: u32,
}

