// DO NOT EDIT -- generated code

use byteorder::{ByteOrder, ReadBytesExt};
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

#[derive(Debug)]
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

impl Message {
    pub fn read<'i, Order, Reader>(reader: &mut Reader, msg: MesgNum, fields: &Vec<FieldDefinition>)
        -> Result<Message, String>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        match msg {

            MesgNum::AccelerometerData =>
                accelerometer_data::AccelerometerData::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::AccelerometerData(m))
                    .map_err(|_e| "could not read AccelerometerData".to_owned()),

            MesgNum::Activity =>
                activity::Activity::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::Activity(m))
                    .map_err(|_e| "could not read Activity".to_owned()),

            MesgNum::AntChannelId =>
                ant_channel_id::AntChannelId::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::AntChannelId(m))
                    .map_err(|_e| "could not read AntChannelId".to_owned()),

            MesgNum::AntRx =>
                ant_rx::AntRx::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::AntRx(m))
                    .map_err(|_e| "could not read AntRx".to_owned()),

            MesgNum::AntTx =>
                ant_tx::AntTx::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::AntTx(m))
                    .map_err(|_e| "could not read AntTx".to_owned()),

            MesgNum::AviationAttitude =>
                aviation_attitude::AviationAttitude::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::AviationAttitude(m))
                    .map_err(|_e| "could not read AviationAttitude".to_owned()),

            MesgNum::BarometerData =>
                barometer_data::BarometerData::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::BarometerData(m))
                    .map_err(|_e| "could not read BarometerData".to_owned()),

            MesgNum::BikeProfile =>
                bike_profile::BikeProfile::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::BikeProfile(m))
                    .map_err(|_e| "could not read BikeProfile".to_owned()),

            MesgNum::BloodPressure =>
                blood_pressure::BloodPressure::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::BloodPressure(m))
                    .map_err(|_e| "could not read BloodPressure".to_owned()),

            MesgNum::CadenceZone =>
                cadence_zone::CadenceZone::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::CadenceZone(m))
                    .map_err(|_e| "could not read CadenceZone".to_owned()),

            MesgNum::CameraEvent =>
                camera_event::CameraEvent::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::CameraEvent(m))
                    .map_err(|_e| "could not read CameraEvent".to_owned()),

            MesgNum::Capabilities =>
                capabilities::Capabilities::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::Capabilities(m))
                    .map_err(|_e| "could not read Capabilities".to_owned()),

            MesgNum::Connectivity =>
                connectivity::Connectivity::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::Connectivity(m))
                    .map_err(|_e| "could not read Connectivity".to_owned()),

            MesgNum::Course =>
                course::Course::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::Course(m))
                    .map_err(|_e| "could not read Course".to_owned()),

            MesgNum::CoursePoint =>
                course_point::CoursePoint::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::CoursePoint(m))
                    .map_err(|_e| "could not read CoursePoint".to_owned()),

            MesgNum::DeveloperDataId =>
                developer_data_id::DeveloperDataId::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::DeveloperDataId(m))
                    .map_err(|_e| "could not read DeveloperDataId".to_owned()),

            MesgNum::DeviceInfo =>
                device_info::DeviceInfo::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::DeviceInfo(m))
                    .map_err(|_e| "could not read DeviceInfo".to_owned()),

            MesgNum::DeviceSettings =>
                device_settings::DeviceSettings::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::DeviceSettings(m))
                    .map_err(|_e| "could not read DeviceSettings".to_owned()),

            MesgNum::DiveAlarm =>
                dive_alarm::DiveAlarm::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::DiveAlarm(m))
                    .map_err(|_e| "could not read DiveAlarm".to_owned()),

            MesgNum::DiveGas =>
                dive_gas::DiveGas::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::DiveGas(m))
                    .map_err(|_e| "could not read DiveGas".to_owned()),

            MesgNum::DiveSettings =>
                dive_settings::DiveSettings::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::DiveSettings(m))
                    .map_err(|_e| "could not read DiveSettings".to_owned()),

            MesgNum::DiveSummary =>
                dive_summary::DiveSummary::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::DiveSummary(m))
                    .map_err(|_e| "could not read DiveSummary".to_owned()),

            MesgNum::Event =>
                event::Event::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::Event(m))
                    .map_err(|_e| "could not read Event".to_owned()),

            MesgNum::ExdDataConceptConfiguration =>
                exd_data_concept_configuration::ExdDataConceptConfiguration::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::ExdDataConceptConfiguration(m))
                    .map_err(|_e| "could not read ExdDataConceptConfiguration".to_owned()),

            MesgNum::ExdDataFieldConfiguration =>
                exd_data_field_configuration::ExdDataFieldConfiguration::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::ExdDataFieldConfiguration(m))
                    .map_err(|_e| "could not read ExdDataFieldConfiguration".to_owned()),

            MesgNum::ExdScreenConfiguration =>
                exd_screen_configuration::ExdScreenConfiguration::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::ExdScreenConfiguration(m))
                    .map_err(|_e| "could not read ExdScreenConfiguration".to_owned()),

            MesgNum::ExerciseTitle =>
                exercise_title::ExerciseTitle::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::ExerciseTitle(m))
                    .map_err(|_e| "could not read ExerciseTitle".to_owned()),

            MesgNum::FieldCapabilities =>
                field_capabilities::FieldCapabilities::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::FieldCapabilities(m))
                    .map_err(|_e| "could not read FieldCapabilities".to_owned()),

            MesgNum::FieldDescription =>
                field_description::FieldDescription::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::FieldDescription(m))
                    .map_err(|_e| "could not read FieldDescription".to_owned()),

            MesgNum::FileCapabilities =>
                file_capabilities::FileCapabilities::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::FileCapabilities(m))
                    .map_err(|_e| "could not read FileCapabilities".to_owned()),

            MesgNum::FileCreator =>
                file_creator::FileCreator::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::FileCreator(m))
                    .map_err(|_e| "could not read FileCreator".to_owned()),

            MesgNum::FileId =>
                file_id::FileId::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::FileId(m))
                    .map_err(|_e| "could not read FileId".to_owned()),

            MesgNum::Goal =>
                goal::Goal::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::Goal(m))
                    .map_err(|_e| "could not read Goal".to_owned()),

            MesgNum::GpsMetadata =>
                gps_metadata::GpsMetadata::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::GpsMetadata(m))
                    .map_err(|_e| "could not read GpsMetadata".to_owned()),

            MesgNum::GyroscopeData =>
                gyroscope_data::GyroscopeData::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::GyroscopeData(m))
                    .map_err(|_e| "could not read GyroscopeData".to_owned()),

            MesgNum::Hr =>
                hr::Hr::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::Hr(m))
                    .map_err(|_e| "could not read Hr".to_owned()),

            MesgNum::HrZone =>
                hr_zone::HrZone::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::HrZone(m))
                    .map_err(|_e| "could not read HrZone".to_owned()),

            MesgNum::HrmProfile =>
                hrm_profile::HrmProfile::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::HrmProfile(m))
                    .map_err(|_e| "could not read HrmProfile".to_owned()),

            MesgNum::Hrv =>
                hrv::Hrv::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::Hrv(m))
                    .map_err(|_e| "could not read Hrv".to_owned()),

            MesgNum::Lap =>
                lap::Lap::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::Lap(m))
                    .map_err(|_e| "could not read Lap".to_owned()),

            MesgNum::Length =>
                length::Length::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::Length(m))
                    .map_err(|_e| "could not read Length".to_owned()),

            MesgNum::MagnetometerData =>
                magnetometer_data::MagnetometerData::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::MagnetometerData(m))
                    .map_err(|_e| "could not read MagnetometerData".to_owned()),

            MesgNum::MemoGlob =>
                memo_glob::MemoGlob::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::MemoGlob(m))
                    .map_err(|_e| "could not read MemoGlob".to_owned()),

            MesgNum::MesgCapabilities =>
                mesg_capabilities::MesgCapabilities::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::MesgCapabilities(m))
                    .map_err(|_e| "could not read MesgCapabilities".to_owned()),

            MesgNum::MetZone =>
                met_zone::MetZone::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::MetZone(m))
                    .map_err(|_e| "could not read MetZone".to_owned()),

            MesgNum::Monitoring =>
                monitoring::Monitoring::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::Monitoring(m))
                    .map_err(|_e| "could not read Monitoring".to_owned()),

            MesgNum::MonitoringInfo =>
                monitoring_info::MonitoringInfo::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::MonitoringInfo(m))
                    .map_err(|_e| "could not read MonitoringInfo".to_owned()),

            MesgNum::NmeaSentence =>
                nmea_sentence::NmeaSentence::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::NmeaSentence(m))
                    .map_err(|_e| "could not read NmeaSentence".to_owned()),

            MesgNum::ObdiiData =>
                obdii_data::ObdiiData::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::ObdiiData(m))
                    .map_err(|_e| "could not read ObdiiData".to_owned()),

            MesgNum::OhrSettings =>
                ohr_settings::OhrSettings::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::OhrSettings(m))
                    .map_err(|_e| "could not read OhrSettings".to_owned()),

            MesgNum::OneDSensorCalibration =>
                one_d_sensor_calibration::OneDSensorCalibration::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::OneDSensorCalibration(m))
                    .map_err(|_e| "could not read OneDSensorCalibration".to_owned()),

            MesgNum::PowerZone =>
                power_zone::PowerZone::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::PowerZone(m))
                    .map_err(|_e| "could not read PowerZone".to_owned()),

            MesgNum::Record =>
                record::Record::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::Record(m))
                    .map_err(|_e| "could not read Record".to_owned()),

            MesgNum::Schedule =>
                schedule::Schedule::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::Schedule(m))
                    .map_err(|_e| "could not read Schedule".to_owned()),

            MesgNum::SdmProfile =>
                sdm_profile::SdmProfile::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::SdmProfile(m))
                    .map_err(|_e| "could not read SdmProfile".to_owned()),

            MesgNum::SegmentFile =>
                segment_file::SegmentFile::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::SegmentFile(m))
                    .map_err(|_e| "could not read SegmentFile".to_owned()),

            MesgNum::SegmentId =>
                segment_id::SegmentId::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::SegmentId(m))
                    .map_err(|_e| "could not read SegmentId".to_owned()),

            MesgNum::SegmentLap =>
                segment_lap::SegmentLap::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::SegmentLap(m))
                    .map_err(|_e| "could not read SegmentLap".to_owned()),

            MesgNum::SegmentLeaderboardEntry =>
                segment_leaderboard_entry::SegmentLeaderboardEntry::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::SegmentLeaderboardEntry(m))
                    .map_err(|_e| "could not read SegmentLeaderboardEntry".to_owned()),

            MesgNum::SegmentPoint =>
                segment_point::SegmentPoint::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::SegmentPoint(m))
                    .map_err(|_e| "could not read SegmentPoint".to_owned()),

            MesgNum::Session =>
                session::Session::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::Session(m))
                    .map_err(|_e| "could not read Session".to_owned()),

            MesgNum::Set =>
                set::Set::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::Set(m))
                    .map_err(|_e| "could not read Set".to_owned()),

            MesgNum::SlaveDevice =>
                slave_device::SlaveDevice::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::SlaveDevice(m))
                    .map_err(|_e| "could not read SlaveDevice".to_owned()),

            MesgNum::Software =>
                software::Software::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::Software(m))
                    .map_err(|_e| "could not read Software".to_owned()),

            MesgNum::SpeedZone =>
                speed_zone::SpeedZone::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::SpeedZone(m))
                    .map_err(|_e| "could not read SpeedZone".to_owned()),

            MesgNum::Sport =>
                sport::Sport::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::Sport(m))
                    .map_err(|_e| "could not read Sport".to_owned()),

            MesgNum::StressLevel =>
                stress_level::StressLevel::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::StressLevel(m))
                    .map_err(|_e| "could not read StressLevel".to_owned()),

            MesgNum::ThreeDSensorCalibration =>
                three_d_sensor_calibration::ThreeDSensorCalibration::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::ThreeDSensorCalibration(m))
                    .map_err(|_e| "could not read ThreeDSensorCalibration".to_owned()),

            MesgNum::TimestampCorrelation =>
                timestamp_correlation::TimestampCorrelation::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::TimestampCorrelation(m))
                    .map_err(|_e| "could not read TimestampCorrelation".to_owned()),

            MesgNum::Totals =>
                totals::Totals::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::Totals(m))
                    .map_err(|_e| "could not read Totals".to_owned()),

            MesgNum::TrainingFile =>
                training_file::TrainingFile::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::TrainingFile(m))
                    .map_err(|_e| "could not read TrainingFile".to_owned()),

            MesgNum::UserProfile =>
                user_profile::UserProfile::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::UserProfile(m))
                    .map_err(|_e| "could not read UserProfile".to_owned()),

            MesgNum::Video =>
                video::Video::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::Video(m))
                    .map_err(|_e| "could not read Video".to_owned()),

            MesgNum::VideoClip =>
                video_clip::VideoClip::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::VideoClip(m))
                    .map_err(|_e| "could not read VideoClip".to_owned()),

            MesgNum::VideoDescription =>
                video_description::VideoDescription::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::VideoDescription(m))
                    .map_err(|_e| "could not read VideoDescription".to_owned()),

            MesgNum::VideoFrame =>
                video_frame::VideoFrame::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::VideoFrame(m))
                    .map_err(|_e| "could not read VideoFrame".to_owned()),

            MesgNum::VideoTitle =>
                video_title::VideoTitle::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::VideoTitle(m))
                    .map_err(|_e| "could not read VideoTitle".to_owned()),

            MesgNum::WatchfaceSettings =>
                watchface_settings::WatchfaceSettings::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::WatchfaceSettings(m))
                    .map_err(|_e| "could not read WatchfaceSettings".to_owned()),

            MesgNum::WeatherAlert =>
                weather_alert::WeatherAlert::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::WeatherAlert(m))
                    .map_err(|_e| "could not read WeatherAlert".to_owned()),

            MesgNum::WeatherConditions =>
                weather_conditions::WeatherConditions::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::WeatherConditions(m))
                    .map_err(|_e| "could not read WeatherConditions".to_owned()),

            MesgNum::WeightScale =>
                weight_scale::WeightScale::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::WeightScale(m))
                    .map_err(|_e| "could not read WeightScale".to_owned()),

            MesgNum::Workout =>
                workout::Workout::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::Workout(m))
                    .map_err(|_e| "could not read Workout".to_owned()),

            MesgNum::WorkoutSession =>
                workout_session::WorkoutSession::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::WorkoutSession(m))
                    .map_err(|_e| "could not read WorkoutSession".to_owned()),

            MesgNum::WorkoutStep =>
                workout_step::WorkoutStep::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::WorkoutStep(m))
                    .map_err(|_e| "could not read WorkoutStep".to_owned()),

            MesgNum::ZonesTarget =>
                zones_target::ZonesTarget::from_fields::<Order, Reader>(reader, fields)
                    .map(|m| Message::ZonesTarget(m))
                    .map_err(|_e| "could not read ZonesTarget".to_owned()),
            m => {
                fields.iter()
                    .map(|f| f.content_from::<Order, Reader>(reader))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| e.to_string())?;
                Err(format!("unknown message: {:?}", m))
            },
        }
    }
}
