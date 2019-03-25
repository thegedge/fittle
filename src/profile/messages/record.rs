// DO NOT EDIT -- generated code

use byteorder::{
    ByteOrder,
    ReadBytesExt
};

use serde::Serialize;

use crate::fields::FieldDefinition;

#[derive(Debug, Default, Serialize)]
pub struct Record {
    #[serde(skip_serializing_if = "Option::is_none")]
    absolute_pressure: Option<crate::fields::Pressure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    accumulated_power: Option<crate::fields::Power>,

    #[serde(skip_serializing_if = "Option::is_none")]
    activity_type: Option<crate::profile::enums::ActivityType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    altitude: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ball_speed: Option<crate::fields::Velocity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    battery_soc: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cadence: Option<crate::fields::Frequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cadence256: Option<crate::fields::Frequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    calories: Option<crate::fields::Energy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cns_load: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    combined_pedal_smoothness: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    compressed_accumulated_power: Option<crate::fields::Power>,

    #[serde(skip_serializing_if = "Option::is_none")]
    compressed_speed_distance: Option<Vec<u8>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cycle_length: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cycles: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    depth: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    device_index: Option<crate::profile::enums::DeviceIndex>,

    #[serde(skip_serializing_if = "Option::is_none")]
    distance: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    enhanced_altitude: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    enhanced_speed: Option<crate::fields::Velocity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fractional_cadence: Option<crate::fields::Frequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    gps_accuracy: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    grade: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    heart_rate: Option<crate::fields::Frequency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left_pco: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left_pedal_smoothness: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left_power_phase: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left_power_phase_peak: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left_right_balance: Option<crate::profile::enums::LeftRightBalance>,

    #[serde(skip_serializing_if = "Option::is_none")]
    left_torque_effectiveness: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    motor_power: Option<crate::fields::Power>,

    #[serde(skip_serializing_if = "Option::is_none")]
    n2_load: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ndl_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    next_stop_depth: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    next_stop_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    position_lat: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    position_long: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    power: Option<crate::fields::Power>,

    #[serde(skip_serializing_if = "Option::is_none")]
    resistance: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    right_pco: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    right_pedal_smoothness: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    right_power_phase: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    right_power_phase_peak: Option<Vec<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    right_torque_effectiveness: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    saturated_hemoglobin_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    saturated_hemoglobin_percent_max: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    saturated_hemoglobin_percent_min: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    speed: Option<crate::fields::Velocity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    speed_1s: Option<Vec<crate::fields::Velocity>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stance_time: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stance_time_balance: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stance_time_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    step_length: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stroke_type: Option<crate::profile::enums::StrokeType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<crate::fields::ThermodynamicTemperature>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time128: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_from_course: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    time_to_surface: Option<crate::fields::Time>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<crate::fields::DateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_cycles: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_hemoglobin_conc: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_hemoglobin_conc_max: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    total_hemoglobin_conc_min: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    vertical_oscillation: Option<crate::fields::Length>,

    #[serde(skip_serializing_if = "Option::is_none")]
    vertical_ratio: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    vertical_speed: Option<crate::fields::Velocity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<u8>,
}

impl Record {
    pub fn from_fields<Order, Reader>(reader: &mut Reader, fields: &Vec<FieldDefinition>)
        -> Result<Self, std::io::Error>
        where
            Order: ByteOrder,
            Reader: ReadBytesExt,
    {
        let mut msg: Self = Default::default();
        for field in fields {
            let (number, content) = field.content_from::<Order, Reader>(reader)?;
            match number {
                0 => msg.position_lat = content.one().map(<i32>::from),
                1 => msg.position_long = content.one().map(<i32>::from),
                2 => msg.altitude = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u16>::from(v)) / 5.0 - 500.0 })(v))),
                3 => msg.heart_rate = content.one().map(|v| crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>((<u8>::from)(v))),
                4 => msg.cadence = content.one().map(|v| crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, u8>((<u8>::from)(v))),
                5 => msg.distance = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u32>::from(v)) / 100.0 - 0.0 })(v))),
                6 => msg.speed = content.one().map(|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { <f64>::from(<u16>::from(v)) / 1000.0 - 0.0 })(v))),
                7 => msg.power = content.one().map(|v| crate::fields::Power::new::<uom::si::power::watt, u16>((<u16>::from)(v))),
                8 => msg.compressed_speed_distance = content.many().map(|vec| vec.into_iter().map(<u8>::from).collect()),
                9 => msg.grade = content.one().map(|v| { <f64>::from(<i16>::from(v)) / 100.0 - 0.0 }),
                10 => msg.resistance = content.one().map(<u8>::from),
                11 => msg.time_from_course = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<i32>::from(v)) / 1000.0 - 0.0 })(v))),
                12 => msg.cycle_length = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u8>::from(v)) / 100.0 - 0.0 })(v))),
                13 => msg.temperature = content.one().map(|v| crate::fields::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::degree_celsius, i8>((<i8>::from)(v))),
                17 => msg.speed_1s = content.many().map(|vec| vec.into_iter().map(|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { <f64>::from(<u8>::from(v)) / 16.0 - 0.0 })(v))).collect()),
                18 => msg.cycles = content.one().map(<u8>::from),
                19 => msg.total_cycles = content.one().map(<u32>::from),
                28 => msg.compressed_accumulated_power = content.one().map(|v| crate::fields::Power::new::<uom::si::power::watt, u16>((<u16>::from)(v))),
                29 => msg.accumulated_power = content.one().map(|v| crate::fields::Power::new::<uom::si::power::watt, u32>((<u32>::from)(v))),
                30 => msg.left_right_balance = content.one().map(<crate::profile::enums::LeftRightBalance>::from),
                31 => msg.gps_accuracy = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, u8>((<u8>::from)(v))),
                32 => msg.vertical_speed = content.one().map(|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { <f64>::from(<i16>::from(v)) / 1000.0 - 0.0 })(v))),
                33 => msg.calories = content.one().map(|v| crate::fields::Energy::new::<uom::si::energy::kilocalorie, u16>((<u16>::from)(v))),
                39 => msg.vertical_oscillation = content.one().map(|v| crate::fields::Length::new::<uom::si::length::millimeter, f64>((|v| { <f64>::from(<u16>::from(v)) / 10.0 - 0.0 })(v))),
                40 => msg.stance_time_percent = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 }),
                41 => msg.stance_time = content.one().map(|v| crate::fields::Time::new::<uom::si::time::millisecond, f64>((|v| { <f64>::from(<u16>::from(v)) / 10.0 - 0.0 })(v))),
                42 => msg.activity_type = content.one().map(<crate::profile::enums::ActivityType>::from),
                43 => msg.left_torque_effectiveness = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 2.0 - 0.0 }),
                44 => msg.right_torque_effectiveness = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 2.0 - 0.0 }),
                45 => msg.left_pedal_smoothness = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 2.0 - 0.0 }),
                46 => msg.right_pedal_smoothness = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 2.0 - 0.0 }),
                47 => msg.combined_pedal_smoothness = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 2.0 - 0.0 }),
                48 => msg.time128 = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u8>::from(v)) / 128.0 - 0.0 })(v))),
                49 => msg.stroke_type = content.one().map(<crate::profile::enums::StrokeType>::from),
                50 => msg.zone = content.one().map(<u8>::from),
                51 => msg.ball_speed = content.one().map(|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 })(v))),
                52 => msg.cadence256 = content.one().map(|v| crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, f64>((|v| { <f64>::from(<u16>::from(v)) / 256.0 - 0.0 })(v))),
                53 => msg.fractional_cadence = content.one().map(|v| crate::fields::Frequency::new::<uom::si::frequency::cycle_per_minute, f64>((|v| { <f64>::from(<u8>::from(v)) / 128.0 - 0.0 })(v))),
                54 => msg.total_hemoglobin_conc = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 }),
                55 => msg.total_hemoglobin_conc_min = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 }),
                56 => msg.total_hemoglobin_conc_max = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 }),
                57 => msg.saturated_hemoglobin_percent = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 10.0 - 0.0 }),
                58 => msg.saturated_hemoglobin_percent_min = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 10.0 - 0.0 }),
                59 => msg.saturated_hemoglobin_percent_max = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 10.0 - 0.0 }),
                62 => msg.device_index = content.one().map(<crate::profile::enums::DeviceIndex>::from),
                67 => msg.left_pco = content.one().map(|v| crate::fields::Length::new::<uom::si::length::millimeter, i8>((<i8>::from)(v))),
                68 => msg.right_pco = content.one().map(|v| crate::fields::Length::new::<uom::si::length::millimeter, i8>((<i8>::from)(v))),
                69 => msg.left_power_phase = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u8>::from(v)) / 0.0 - 0.0 }).collect()),
                70 => msg.left_power_phase_peak = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u8>::from(v)) / 0.0 - 0.0 }).collect()),
                71 => msg.right_power_phase = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u8>::from(v)) / 0.0 - 0.0 }).collect()),
                72 => msg.right_power_phase_peak = content.many().map(|vec| vec.into_iter().map(|v| { <f64>::from(<u8>::from(v)) / 0.0 - 0.0 }).collect()),
                73 => msg.enhanced_speed = content.one().map(|v| crate::fields::Velocity::new::<uom::si::velocity::meter_per_second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                78 => msg.enhanced_altitude = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u32>::from(v)) / 5.0 - 500.0 })(v))),
                81 => msg.battery_soc = content.one().map(|v| { <f64>::from(<u8>::from(v)) / 2.0 - 0.0 }),
                82 => msg.motor_power = content.one().map(|v| crate::fields::Power::new::<uom::si::power::watt, u16>((<u16>::from)(v))),
                83 => msg.vertical_ratio = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 }),
                84 => msg.stance_time_balance = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 100.0 - 0.0 }),
                85 => msg.step_length = content.one().map(|v| crate::fields::Length::new::<uom::si::length::millimeter, f64>((|v| { <f64>::from(<u16>::from(v)) / 10.0 - 0.0 })(v))),
                91 => msg.absolute_pressure = content.one().map(|v| crate::fields::Pressure::new::<uom::si::pressure::pascal, u32>((<u32>::from)(v))),
                92 => msg.depth = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                93 => msg.next_stop_depth = content.one().map(|v| crate::fields::Length::new::<uom::si::length::meter, f64>((|v| { <f64>::from(<u32>::from(v)) / 1000.0 - 0.0 })(v))),
                94 => msg.next_stop_time = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1.0 - 0.0 })(v))),
                95 => msg.time_to_surface = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1.0 - 0.0 })(v))),
                96 => msg.ndl_time = content.one().map(|v| crate::fields::Time::new::<uom::si::time::second, f64>((|v| { <f64>::from(<u32>::from(v)) / 1.0 - 0.0 })(v))),
                97 => msg.cns_load = content.one().map(<u8>::from),
                98 => msg.n2_load = content.one().map(|v| { <f64>::from(<u16>::from(v)) / 1.0 - 0.0 }),
                253 => msg.timestamp = content.one().map(<crate::fields::DateTime>::from),
                _ => (),
            };
        }

        Ok(msg)
    }
}
