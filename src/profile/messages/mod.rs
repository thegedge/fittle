// DO NOT EDIT -- generated code

mod accelerometer_data;
mod activity;
mod ant_channel_id;
mod ant_tx;
mod aviation_attitude;
mod barometer_data;
mod bike_profile;
mod blood_pressure;
mod cadence_zone;
mod camera_event;
mod capabilities;
mod connectivity;
mod course;
mod course_point;
mod developer_data_id;
mod device_settings;
mod dive_gas;
mod dive_settings;
mod event;
mod exd_data_concept_configuration;
mod exd_data_field_configuration;
mod exercise_title;
mod field_capabilities;
mod file_capabilities;
mod file_creator;
mod file_id;
mod goal;
mod gyroscope_data;
mod hr;
mod hr_zone;
mod hrm_profile;
mod hrv;
mod length;
mod magnetometer_data;
mod memo_glob;
mod mesg_capabilities;
mod met_zone;
mod monitoring;
mod monitoring_info;
mod nmea_sentence;
mod obdii_data;
mod ohr_settings;
mod one_d_sensor_calibration;
mod power_zone;
mod record;
mod schedule;
mod sdm_profile;
mod segment_file;
mod segment_id;
mod segment_lap;
mod segment_leaderboard_entry;
mod segment_point;
mod session;
mod set;
mod slave_device;
mod software;
mod speed_zone;
mod sport;
mod three_d_sensor_calibration;
mod timestamp_correlation;
mod totals;
mod training_file;
mod video;
mod video_clip;
mod video_description;
mod video_frame;
mod video_title;
mod weather_alert;
mod weather_conditions;
mod weight_scale;
mod workout;
mod workout_session;
mod zones_target;
use self::accelerometer_data::AccelerometerData;
use self::activity::Activity;
use self::ant_channel_id::AntChannelId;
use self::ant_tx::AntTx;
use self::aviation_attitude::AviationAttitude;
use self::barometer_data::BarometerData;
use self::bike_profile::BikeProfile;
use self::blood_pressure::BloodPressure;
use self::cadence_zone::CadenceZone;
use self::camera_event::CameraEvent;
use self::capabilities::Capabilities;
use self::connectivity::Connectivity;
use self::course::Course;
use self::course_point::CoursePoint;
use self::developer_data_id::DeveloperDataId;
use self::device_settings::DeviceSettings;
use self::dive_gas::DiveGas;
use self::dive_settings::DiveSettings;
use self::event::Event;
use self::exd_data_concept_configuration::ExdDataConceptConfiguration;
use self::exd_data_field_configuration::ExdDataFieldConfiguration;
use self::exercise_title::ExerciseTitle;
use self::field_capabilities::FieldCapabilities;
use self::file_capabilities::FileCapabilities;
use self::file_creator::FileCreator;
use self::file_id::FileId;
use self::goal::Goal;
use self::gyroscope_data::GyroscopeData;
use self::hr::Hr;
use self::hr_zone::HrZone;
use self::hrm_profile::HrmProfile;
use self::hrv::Hrv;
use self::length::Length;
use self::magnetometer_data::MagnetometerData;
use self::memo_glob::MemoGlob;
use self::mesg_capabilities::MesgCapabilities;
use self::met_zone::MetZone;
use self::monitoring::Monitoring;
use self::monitoring_info::MonitoringInfo;
use self::nmea_sentence::NmeaSentence;
use self::obdii_data::ObdiiData;
use self::ohr_settings::OhrSettings;
use self::one_d_sensor_calibration::OneDSensorCalibration;
use self::power_zone::PowerZone;
use self::record::Record;
use self::schedule::Schedule;
use self::sdm_profile::SdmProfile;
use self::segment_file::SegmentFile;
use self::segment_id::SegmentId;
use self::segment_lap::SegmentLap;
use self::segment_leaderboard_entry::SegmentLeaderboardEntry;
use self::segment_point::SegmentPoint;
use self::session::Session;
use self::set::Set;
use self::slave_device::SlaveDevice;
use self::software::Software;
use self::speed_zone::SpeedZone;
use self::sport::Sport;
use self::three_d_sensor_calibration::ThreeDSensorCalibration;
use self::timestamp_correlation::TimestampCorrelation;
use self::totals::Totals;
use self::training_file::TrainingFile;
use self::video::Video;
use self::video_clip::VideoClip;
use self::video_description::VideoDescription;
use self::video_frame::VideoFrame;
use self::video_title::VideoTitle;
use self::weather_alert::WeatherAlert;
use self::weather_conditions::WeatherConditions;
use self::weight_scale::WeightScale;
use self::workout::Workout;
use self::workout_session::WorkoutSession;
use self::zones_target::ZonesTarget;
#[derive(Debug)]
pub enum Message {
    AccelerometerData(AccelerometerData),
    Activity(Activity),
    AntChannelId(AntChannelId),
    AntTx(AntTx),
    AviationAttitude(AviationAttitude),
    BarometerData(BarometerData),
    BikeProfile(BikeProfile),
    BloodPressure(BloodPressure),
    CadenceZone(CadenceZone),
    CameraEvent(CameraEvent),
    Capabilities(Capabilities),
    Connectivity(Connectivity),
    Course(Course),
    CoursePoint(CoursePoint),
    DeveloperDataId(DeveloperDataId),
    DeviceSettings(DeviceSettings),
    DiveGas(DiveGas),
    DiveSettings(DiveSettings),
    Event(Event),
    ExdDataConceptConfiguration(ExdDataConceptConfiguration),
    ExdDataFieldConfiguration(ExdDataFieldConfiguration),
    ExerciseTitle(ExerciseTitle),
    FieldCapabilities(FieldCapabilities),
    FileCapabilities(FileCapabilities),
    FileCreator(FileCreator),
    FileId(FileId),
    Goal(Goal),
    GyroscopeData(GyroscopeData),
    Hr(Hr),
    HrZone(HrZone),
    HrmProfile(HrmProfile),
    Hrv(Hrv),
    Length(Length),
    MagnetometerData(MagnetometerData),
    MemoGlob(MemoGlob),
    MesgCapabilities(MesgCapabilities),
    MetZone(MetZone),
    Monitoring(Monitoring),
    MonitoringInfo(MonitoringInfo),
    NmeaSentence(NmeaSentence),
    ObdiiData(ObdiiData),
    OhrSettings(OhrSettings),
    OneDSensorCalibration(OneDSensorCalibration),
    PowerZone(PowerZone),
    Record(Record),
    Schedule(Schedule),
    SdmProfile(SdmProfile),
    SegmentFile(SegmentFile),
    SegmentId(SegmentId),
    SegmentLap(SegmentLap),
    SegmentLeaderboardEntry(SegmentLeaderboardEntry),
    SegmentPoint(SegmentPoint),
    Session(Session),
    Set(Set),
    SlaveDevice(SlaveDevice),
    Software(Software),
    SpeedZone(SpeedZone),
    Sport(Sport),
    ThreeDSensorCalibration(ThreeDSensorCalibration),
    TimestampCorrelation(TimestampCorrelation),
    Totals(Totals),
    TrainingFile(TrainingFile),
    Video(Video),
    VideoClip(VideoClip),
    VideoDescription(VideoDescription),
    VideoFrame(VideoFrame),
    VideoTitle(VideoTitle),
    WeatherAlert(WeatherAlert),
    WeatherConditions(WeatherConditions),
    WeightScale(WeightScale),
    Workout(Workout),
    WorkoutSession(WorkoutSession),
    ZonesTarget(ZonesTarget),
}

