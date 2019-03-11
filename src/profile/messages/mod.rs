// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::profile::enums::MesgNum;
use crate::fields::FieldDefinition;


mod accelerometer_data;
mod activity;
mod ant_channel_id;
mod ant_rx;
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
mod device_info;
mod device_settings;
mod dive_alarm;
mod dive_gas;
mod dive_settings;
mod dive_summary;
mod event;
mod exd_data_concept_configuration;
mod exd_data_field_configuration;
mod exd_screen_configuration;
mod exercise_title;
mod field_capabilities;
mod field_description;
mod file_capabilities;
mod file_creator;
mod file_id;
mod goal;
mod gps_metadata;
mod gyroscope_data;
mod hr;
mod hr_zone;
mod hrm_profile;
mod hrv;
mod lap;
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
mod stress_level;
mod three_d_sensor_calibration;
mod timestamp_correlation;
mod totals;
mod training_file;
mod user_profile;
mod video;
mod video_clip;
mod video_description;
mod video_frame;
mod video_title;
mod watchface_settings;
mod weather_alert;
mod weather_conditions;
mod weight_scale;
mod workout;
mod workout_session;
mod workout_step;
mod zones_target;


use self::accelerometer_data::AccelerometerData;
use self::activity::Activity;
use self::ant_channel_id::AntChannelId;
use self::ant_rx::AntRx;
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
use self::device_info::DeviceInfo;
use self::device_settings::DeviceSettings;
use self::dive_alarm::DiveAlarm;
use self::dive_gas::DiveGas;
use self::dive_settings::DiveSettings;
use self::dive_summary::DiveSummary;
use self::event::Event;
use self::exd_data_concept_configuration::ExdDataConceptConfiguration;
use self::exd_data_field_configuration::ExdDataFieldConfiguration;
use self::exd_screen_configuration::ExdScreenConfiguration;
use self::exercise_title::ExerciseTitle;
use self::field_capabilities::FieldCapabilities;
use self::field_description::FieldDescription;
use self::file_capabilities::FileCapabilities;
use self::file_creator::FileCreator;
use self::file_id::FileId;
use self::goal::Goal;
use self::gps_metadata::GpsMetadata;
use self::gyroscope_data::GyroscopeData;
use self::hr::Hr;
use self::hr_zone::HrZone;
use self::hrm_profile::HrmProfile;
use self::hrv::Hrv;
use self::lap::Lap;
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
use self::stress_level::StressLevel;
use self::three_d_sensor_calibration::ThreeDSensorCalibration;
use self::timestamp_correlation::TimestampCorrelation;
use self::totals::Totals;
use self::training_file::TrainingFile;
use self::user_profile::UserProfile;
use self::video::Video;
use self::video_clip::VideoClip;
use self::video_description::VideoDescription;
use self::video_frame::VideoFrame;
use self::video_title::VideoTitle;
use self::watchface_settings::WatchfaceSettings;
use self::weather_alert::WeatherAlert;
use self::weather_conditions::WeatherConditions;
use self::weight_scale::WeightScale;
use self::workout::Workout;
use self::workout_session::WorkoutSession;
use self::workout_step::WorkoutStep;
use self::zones_target::ZonesTarget;

#[derive(Debug, Serialize)]
#[serde(tag = "message_type")]
#[serde(rename_all = "snake_case")]
pub enum Message {
    AccelerometerData(AccelerometerData),
    Activity(Activity),
    AntChannelId(AntChannelId),
    AntRx(AntRx),
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
    DeviceInfo(DeviceInfo),
    DeviceSettings(DeviceSettings),
    DiveAlarm(DiveAlarm),
    DiveGas(DiveGas),
    DiveSettings(DiveSettings),
    DiveSummary(DiveSummary),
    Event(Event),
    ExdDataConceptConfiguration(ExdDataConceptConfiguration),
    ExdDataFieldConfiguration(ExdDataFieldConfiguration),
    ExdScreenConfiguration(ExdScreenConfiguration),
    ExerciseTitle(ExerciseTitle),
    FieldCapabilities(FieldCapabilities),
    FieldDescription(FieldDescription),
    FileCapabilities(FileCapabilities),
    FileCreator(FileCreator),
    FileId(FileId),
    Goal(Goal),
    GpsMetadata(GpsMetadata),
    GyroscopeData(GyroscopeData),
    Hr(Hr),
    HrZone(HrZone),
    HrmProfile(HrmProfile),
    Hrv(Hrv),
    Lap(Lap),
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
    StressLevel(StressLevel),
    ThreeDSensorCalibration(ThreeDSensorCalibration),
    TimestampCorrelation(TimestampCorrelation),
    Totals(Totals),
    TrainingFile(TrainingFile),
    UserProfile(UserProfile),
    Video(Video),
    VideoClip(VideoClip),
    VideoDescription(VideoDescription),
    VideoFrame(VideoFrame),
    VideoTitle(VideoTitle),
    WatchfaceSettings(WatchfaceSettings),
    WeatherAlert(WeatherAlert),
    WeatherConditions(WeatherConditions),
    WeightScale(WeightScale),
    Workout(Workout),
    WorkoutSession(WorkoutSession),
    WorkoutStep(WorkoutStep),
    ZonesTarget(ZonesTarget),
}

// Read a Message from a cursor
impl Message {
    pub fn read<'i, Order, Reader>(reader: &mut Reader, msg: MesgNum, fields: &Vec<FieldDefinition>)
        -> Result<Message, String>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        // TODO this match statement works for now, but I'd like others to be able to extend this
        //      library with their own implementations for custom fields/messages. May require a
        //      big refactor down the road...
        match msg {
            MesgNum::AccelerometerData =>
                accelerometer_data::AccelerometerData::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::AccelerometerData)
                    .map_err(|_e| "could not read message AccelerometerData".to_owned()),

            MesgNum::Activity =>
                activity::Activity::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::Activity)
                    .map_err(|_e| "could not read message Activity".to_owned()),

            MesgNum::AntChannelId =>
                ant_channel_id::AntChannelId::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::AntChannelId)
                    .map_err(|_e| "could not read message AntChannelId".to_owned()),

            MesgNum::AntRx =>
                ant_rx::AntRx::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::AntRx)
                    .map_err(|_e| "could not read message AntRx".to_owned()),

            MesgNum::AntTx =>
                ant_tx::AntTx::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::AntTx)
                    .map_err(|_e| "could not read message AntTx".to_owned()),

            MesgNum::AviationAttitude =>
                aviation_attitude::AviationAttitude::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::AviationAttitude)
                    .map_err(|_e| "could not read message AviationAttitude".to_owned()),

            MesgNum::BarometerData =>
                barometer_data::BarometerData::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::BarometerData)
                    .map_err(|_e| "could not read message BarometerData".to_owned()),

            MesgNum::BikeProfile =>
                bike_profile::BikeProfile::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::BikeProfile)
                    .map_err(|_e| "could not read message BikeProfile".to_owned()),

            MesgNum::BloodPressure =>
                blood_pressure::BloodPressure::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::BloodPressure)
                    .map_err(|_e| "could not read message BloodPressure".to_owned()),

            MesgNum::CadenceZone =>
                cadence_zone::CadenceZone::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::CadenceZone)
                    .map_err(|_e| "could not read message CadenceZone".to_owned()),

            MesgNum::CameraEvent =>
                camera_event::CameraEvent::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::CameraEvent)
                    .map_err(|_e| "could not read message CameraEvent".to_owned()),

            MesgNum::Capabilities =>
                capabilities::Capabilities::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::Capabilities)
                    .map_err(|_e| "could not read message Capabilities".to_owned()),

            MesgNum::Connectivity =>
                connectivity::Connectivity::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::Connectivity)
                    .map_err(|_e| "could not read message Connectivity".to_owned()),

            MesgNum::Course =>
                course::Course::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::Course)
                    .map_err(|_e| "could not read message Course".to_owned()),

            MesgNum::CoursePoint =>
                course_point::CoursePoint::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::CoursePoint)
                    .map_err(|_e| "could not read message CoursePoint".to_owned()),

            MesgNum::DeveloperDataId =>
                developer_data_id::DeveloperDataId::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::DeveloperDataId)
                    .map_err(|_e| "could not read message DeveloperDataId".to_owned()),

            MesgNum::DeviceInfo =>
                device_info::DeviceInfo::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::DeviceInfo)
                    .map_err(|_e| "could not read message DeviceInfo".to_owned()),

            MesgNum::DeviceSettings =>
                device_settings::DeviceSettings::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::DeviceSettings)
                    .map_err(|_e| "could not read message DeviceSettings".to_owned()),

            MesgNum::DiveAlarm =>
                dive_alarm::DiveAlarm::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::DiveAlarm)
                    .map_err(|_e| "could not read message DiveAlarm".to_owned()),

            MesgNum::DiveGas =>
                dive_gas::DiveGas::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::DiveGas)
                    .map_err(|_e| "could not read message DiveGas".to_owned()),

            MesgNum::DiveSettings =>
                dive_settings::DiveSettings::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::DiveSettings)
                    .map_err(|_e| "could not read message DiveSettings".to_owned()),

            MesgNum::DiveSummary =>
                dive_summary::DiveSummary::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::DiveSummary)
                    .map_err(|_e| "could not read message DiveSummary".to_owned()),

            MesgNum::Event =>
                event::Event::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::Event)
                    .map_err(|_e| "could not read message Event".to_owned()),

            MesgNum::ExdDataConceptConfiguration =>
                exd_data_concept_configuration::ExdDataConceptConfiguration::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::ExdDataConceptConfiguration)
                    .map_err(|_e| "could not read message ExdDataConceptConfiguration".to_owned()),

            MesgNum::ExdDataFieldConfiguration =>
                exd_data_field_configuration::ExdDataFieldConfiguration::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::ExdDataFieldConfiguration)
                    .map_err(|_e| "could not read message ExdDataFieldConfiguration".to_owned()),

            MesgNum::ExdScreenConfiguration =>
                exd_screen_configuration::ExdScreenConfiguration::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::ExdScreenConfiguration)
                    .map_err(|_e| "could not read message ExdScreenConfiguration".to_owned()),

            MesgNum::ExerciseTitle =>
                exercise_title::ExerciseTitle::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::ExerciseTitle)
                    .map_err(|_e| "could not read message ExerciseTitle".to_owned()),

            MesgNum::FieldCapabilities =>
                field_capabilities::FieldCapabilities::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::FieldCapabilities)
                    .map_err(|_e| "could not read message FieldCapabilities".to_owned()),

            MesgNum::FieldDescription =>
                field_description::FieldDescription::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::FieldDescription)
                    .map_err(|_e| "could not read message FieldDescription".to_owned()),

            MesgNum::FileCapabilities =>
                file_capabilities::FileCapabilities::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::FileCapabilities)
                    .map_err(|_e| "could not read message FileCapabilities".to_owned()),

            MesgNum::FileCreator =>
                file_creator::FileCreator::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::FileCreator)
                    .map_err(|_e| "could not read message FileCreator".to_owned()),

            MesgNum::FileId =>
                file_id::FileId::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::FileId)
                    .map_err(|_e| "could not read message FileId".to_owned()),

            MesgNum::Goal =>
                goal::Goal::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::Goal)
                    .map_err(|_e| "could not read message Goal".to_owned()),

            MesgNum::GpsMetadata =>
                gps_metadata::GpsMetadata::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::GpsMetadata)
                    .map_err(|_e| "could not read message GpsMetadata".to_owned()),

            MesgNum::GyroscopeData =>
                gyroscope_data::GyroscopeData::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::GyroscopeData)
                    .map_err(|_e| "could not read message GyroscopeData".to_owned()),

            MesgNum::Hr =>
                hr::Hr::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::Hr)
                    .map_err(|_e| "could not read message Hr".to_owned()),

            MesgNum::HrZone =>
                hr_zone::HrZone::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::HrZone)
                    .map_err(|_e| "could not read message HrZone".to_owned()),

            MesgNum::HrmProfile =>
                hrm_profile::HrmProfile::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::HrmProfile)
                    .map_err(|_e| "could not read message HrmProfile".to_owned()),

            MesgNum::Hrv =>
                hrv::Hrv::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::Hrv)
                    .map_err(|_e| "could not read message Hrv".to_owned()),

            MesgNum::Lap =>
                lap::Lap::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::Lap)
                    .map_err(|_e| "could not read message Lap".to_owned()),

            MesgNum::Length =>
                length::Length::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::Length)
                    .map_err(|_e| "could not read message Length".to_owned()),

            MesgNum::MagnetometerData =>
                magnetometer_data::MagnetometerData::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::MagnetometerData)
                    .map_err(|_e| "could not read message MagnetometerData".to_owned()),

            MesgNum::MemoGlob =>
                memo_glob::MemoGlob::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::MemoGlob)
                    .map_err(|_e| "could not read message MemoGlob".to_owned()),

            MesgNum::MesgCapabilities =>
                mesg_capabilities::MesgCapabilities::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::MesgCapabilities)
                    .map_err(|_e| "could not read message MesgCapabilities".to_owned()),

            MesgNum::MetZone =>
                met_zone::MetZone::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::MetZone)
                    .map_err(|_e| "could not read message MetZone".to_owned()),

            MesgNum::Monitoring =>
                monitoring::Monitoring::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::Monitoring)
                    .map_err(|_e| "could not read message Monitoring".to_owned()),

            MesgNum::MonitoringInfo =>
                monitoring_info::MonitoringInfo::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::MonitoringInfo)
                    .map_err(|_e| "could not read message MonitoringInfo".to_owned()),

            MesgNum::NmeaSentence =>
                nmea_sentence::NmeaSentence::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::NmeaSentence)
                    .map_err(|_e| "could not read message NmeaSentence".to_owned()),

            MesgNum::ObdiiData =>
                obdii_data::ObdiiData::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::ObdiiData)
                    .map_err(|_e| "could not read message ObdiiData".to_owned()),

            MesgNum::OhrSettings =>
                ohr_settings::OhrSettings::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::OhrSettings)
                    .map_err(|_e| "could not read message OhrSettings".to_owned()),

            MesgNum::OneDSensorCalibration =>
                one_d_sensor_calibration::OneDSensorCalibration::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::OneDSensorCalibration)
                    .map_err(|_e| "could not read message OneDSensorCalibration".to_owned()),

            MesgNum::PowerZone =>
                power_zone::PowerZone::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::PowerZone)
                    .map_err(|_e| "could not read message PowerZone".to_owned()),

            MesgNum::Record =>
                record::Record::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::Record)
                    .map_err(|_e| "could not read message Record".to_owned()),

            MesgNum::Schedule =>
                schedule::Schedule::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::Schedule)
                    .map_err(|_e| "could not read message Schedule".to_owned()),

            MesgNum::SdmProfile =>
                sdm_profile::SdmProfile::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::SdmProfile)
                    .map_err(|_e| "could not read message SdmProfile".to_owned()),

            MesgNum::SegmentFile =>
                segment_file::SegmentFile::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::SegmentFile)
                    .map_err(|_e| "could not read message SegmentFile".to_owned()),

            MesgNum::SegmentId =>
                segment_id::SegmentId::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::SegmentId)
                    .map_err(|_e| "could not read message SegmentId".to_owned()),

            MesgNum::SegmentLap =>
                segment_lap::SegmentLap::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::SegmentLap)
                    .map_err(|_e| "could not read message SegmentLap".to_owned()),

            MesgNum::SegmentLeaderboardEntry =>
                segment_leaderboard_entry::SegmentLeaderboardEntry::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::SegmentLeaderboardEntry)
                    .map_err(|_e| "could not read message SegmentLeaderboardEntry".to_owned()),

            MesgNum::SegmentPoint =>
                segment_point::SegmentPoint::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::SegmentPoint)
                    .map_err(|_e| "could not read message SegmentPoint".to_owned()),

            MesgNum::Session =>
                session::Session::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::Session)
                    .map_err(|_e| "could not read message Session".to_owned()),

            MesgNum::Set =>
                set::Set::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::Set)
                    .map_err(|_e| "could not read message Set".to_owned()),

            MesgNum::SlaveDevice =>
                slave_device::SlaveDevice::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::SlaveDevice)
                    .map_err(|_e| "could not read message SlaveDevice".to_owned()),

            MesgNum::Software =>
                software::Software::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::Software)
                    .map_err(|_e| "could not read message Software".to_owned()),

            MesgNum::SpeedZone =>
                speed_zone::SpeedZone::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::SpeedZone)
                    .map_err(|_e| "could not read message SpeedZone".to_owned()),

            MesgNum::Sport =>
                sport::Sport::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::Sport)
                    .map_err(|_e| "could not read message Sport".to_owned()),

            MesgNum::StressLevel =>
                stress_level::StressLevel::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::StressLevel)
                    .map_err(|_e| "could not read message StressLevel".to_owned()),

            MesgNum::ThreeDSensorCalibration =>
                three_d_sensor_calibration::ThreeDSensorCalibration::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::ThreeDSensorCalibration)
                    .map_err(|_e| "could not read message ThreeDSensorCalibration".to_owned()),

            MesgNum::TimestampCorrelation =>
                timestamp_correlation::TimestampCorrelation::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::TimestampCorrelation)
                    .map_err(|_e| "could not read message TimestampCorrelation".to_owned()),

            MesgNum::Totals =>
                totals::Totals::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::Totals)
                    .map_err(|_e| "could not read message Totals".to_owned()),

            MesgNum::TrainingFile =>
                training_file::TrainingFile::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::TrainingFile)
                    .map_err(|_e| "could not read message TrainingFile".to_owned()),

            MesgNum::UserProfile =>
                user_profile::UserProfile::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::UserProfile)
                    .map_err(|_e| "could not read message UserProfile".to_owned()),

            MesgNum::Video =>
                video::Video::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::Video)
                    .map_err(|_e| "could not read message Video".to_owned()),

            MesgNum::VideoClip =>
                video_clip::VideoClip::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::VideoClip)
                    .map_err(|_e| "could not read message VideoClip".to_owned()),

            MesgNum::VideoDescription =>
                video_description::VideoDescription::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::VideoDescription)
                    .map_err(|_e| "could not read message VideoDescription".to_owned()),

            MesgNum::VideoFrame =>
                video_frame::VideoFrame::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::VideoFrame)
                    .map_err(|_e| "could not read message VideoFrame".to_owned()),

            MesgNum::VideoTitle =>
                video_title::VideoTitle::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::VideoTitle)
                    .map_err(|_e| "could not read message VideoTitle".to_owned()),

            MesgNum::WatchfaceSettings =>
                watchface_settings::WatchfaceSettings::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::WatchfaceSettings)
                    .map_err(|_e| "could not read message WatchfaceSettings".to_owned()),

            MesgNum::WeatherAlert =>
                weather_alert::WeatherAlert::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::WeatherAlert)
                    .map_err(|_e| "could not read message WeatherAlert".to_owned()),

            MesgNum::WeatherConditions =>
                weather_conditions::WeatherConditions::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::WeatherConditions)
                    .map_err(|_e| "could not read message WeatherConditions".to_owned()),

            MesgNum::WeightScale =>
                weight_scale::WeightScale::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::WeightScale)
                    .map_err(|_e| "could not read message WeightScale".to_owned()),

            MesgNum::Workout =>
                workout::Workout::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::Workout)
                    .map_err(|_e| "could not read message Workout".to_owned()),

            MesgNum::WorkoutSession =>
                workout_session::WorkoutSession::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::WorkoutSession)
                    .map_err(|_e| "could not read message WorkoutSession".to_owned()),

            MesgNum::WorkoutStep =>
                workout_step::WorkoutStep::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::WorkoutStep)
                    .map_err(|_e| "could not read message WorkoutStep".to_owned()),

            MesgNum::ZonesTarget =>
                zones_target::ZonesTarget::from_fields::<Order, Reader>(reader, fields)
                    .map(Message::ZonesTarget)
                    .map_err(|_e| "could not read message ZonesTarget".to_owned()),

            m => {
                // Ensure we move the reader forward
                fields.iter()
                    .map(|f| f.content_from::<Order, Reader>(reader))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| e.to_string())?;

                Err(format!("unknown message: {:?}", m))
            },
        }
    }
}

