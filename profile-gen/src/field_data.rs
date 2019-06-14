use inflector::cases::pascalcase::to_pascal_case;

use serde::{
    Serialize,
    Serializer,

    ser::SerializeStruct,
};

#[derive(Clone, Default, Serialize)]
pub struct FieldComponent {
    pub field: Option<String>,
    pub scale: Option<u16>,
    pub offset: Option<i16>,
    pub unit: Option<String>,
    pub bits: Option<u8>,
}

#[derive(Clone)]
pub enum Components {
    Some(Vec<FieldComponent>),
    None(FieldComponent),
}

#[derive(Clone)]
pub struct FieldData {
    pub base_type: String,
    pub array_length: Option<u8>,
    pub components: Components,
}

// TODO memoize things that get called several times

impl FieldData {
    fn field_type(&self) -> String {
        let field_type = match self.unit_type_and_base() {
            Some((unit_type, _, _)) => format!("crate::fields::{0}", unit_type),
            None => self.adjusted_rust_base_type(),
        };

        match self.array_length {
            Some(_n) => format!("Vec<{0}>", field_type),
            _ => field_type,
        }
    }

    fn conversion_function(&self) -> Option<String> {
        self.single_component().and_then(|comp| {
            match self.base_type.as_str() {
                // These are always annotated with a time unit, but we're using chrono instead of uom
                "date_time" => None,
                "local_date_time" => None,

                _ => comp.unit_type_and_base().and_then(|(unit_type, uom_base, unit_base)| {
                    self.scale_conversion_function().map(|conv|
                        format!(
                            "|v| crate::fields::{0}::new::<uom::si::{1}::{2}, {3}>(({4})(v))",
                            unit_type,
                            uom_base,
                            unit_base,
                            self.adjusted_rust_base_type(),
                            conv,
                        )
                    ).or_else(|| Some(
                        format!(
                            "crate::fields::{0}::new::<uom::si::{1}::{2}, {3}>",
                            unit_type,
                            uom_base,
                            unit_base,
                            self.adjusted_rust_base_type(),
                        )
                    ))
                })
            }
        }).or_else(|| self.scale_conversion_function())
    }

    fn scale_conversion_function(&self) -> Option<String> {
        self.scale_and_offset().map(|(scale, offset)| {
            format!(
                "|v| {{ f64::from(v) / {0}.0 - {1}.0 }}",
                scale,
                offset,
            )
        })
    }

    // The base type adjusted for scale, offsets, etc
    fn adjusted_rust_base_type(&self) -> String {
        if self.scale_and_offset().is_some() {
            "f64".to_string()
        } else {
            self.field_base_type()
        }
    }

    fn field_content(&self) -> &str {
        match self.base_type.as_str() {
            "sint8" => "SignedInt8",
            "sint16" => "SignedInt16",
            "sint32" => "SignedInt32",
            "sint64" => "SignedInt16",
            "uint8" => "UnsignedInt8",
            "uint8z" => "UnsignedInt8z",
            "byte" => "UnsignedInt8",
            "uint16" => "UnsignedInt16",
            "uint16z" => "UnsignedInt16z",
            "uint32" => "UnsignedInt32",
            "uint32z" => "UnsignedInt32z",
            "uint64" => "UnsignedInt64",
            "uint64z" => "UnsignedInt64z",
            "float32" => "Float32",
            "float64" => "Float64",
            "bool" => "Boolean",
            "string" => "String",
            _ => "Enum", // anything else we will assume is an enum
        }
    }

    fn field_storage_type(&self) -> &str {
        match self.base_type.as_str() {
            // Simple base types
            "sint8" => "i8",
            "sint16" => "i16",
            "sint32" => "i32",
            "sint64" => "i16",
            "uint8" | "uint8z" | "byte" => "u8",
            "uint16" | "uint16z" => "u16",
            "uint32" | "uint32z" => "u32",
            "uint64" | "uint64z" => "u64",
            "float32" => "f32",
            "float64" => "f64",
            "bool" => "bool",
            "string" => "String",

            // Specialized forms
            // TODO these are technically in the "Types" sheet, so could provide field data with
            //      both the storage type and the base type
            "date_time" => "u32",
            "local_date_time" => "u32",
            "weight" => "u16",

            // Everything else will assume an enum
            _ => "u8",
        }
    }

    fn field_base_type(&self) -> String {
        match self.base_type.as_str() {
            // Simple base types
            "sint8" => "i8".to_string(),
            "sint16" => "i16".to_string(),
            "sint32" => "i32".to_string(),
            "sint64" => "i16".to_string(),
            "uint8" | "uint8z" | "byte" => "u8".to_string(),
            "uint16" | "uint16z" => "u16".to_string(),
            "uint32" | "uint32z" => "u32".to_string(),
            "uint64" | "uint64z" => "u64".to_string(),
            "float32" => "f32".to_string(),
            "float64" => "f64".to_string(),
            "bool" => "bool".to_string(),
            "string" => "String".to_string(),

            // Specialized forms
            "date_time" => "crate::fields::DateTime".to_string(),
            "local_date_time" => "crate::fields::LocalDateTime".to_string(),
            "weight" => "u16".to_string(),

            // Everything else will be an enum
            v => format!("crate::profile::enums::{}", to_pascal_case(v)),
        }
    }

    fn to_bits_function(&self) -> Option<&str> {
        match self.array_length {
            Some(_) => Some("as_slice"),
            _ => Some("to_le_bytes"),
        }
    }

    fn scale_and_offset(&self) -> Option<(u16, i16)> {
        self.single_component().and_then(FieldComponent::scale_and_offset)
    }

    fn unit_type_and_base(&self) -> Option<(&str, &str, &str)> {
        match self.base_type.as_str() {
            "date_time" => None,
            "local_date_time" => None,
            "timestamp_ms" => None,
            _ => self.single_component()
        }.and_then(FieldComponent::unit_type_and_base)
    }

    fn single_component(&self) -> Option<&FieldComponent> {
        match &self.components {
            Components::None(comp) => Some(comp),
            Components::Some(comps) => {
                // If only a single component, have this field mimic it
                if comps.len() == 1 {
                    Some(&comps[0])
                } else {
                    None
                }
            },
        }
    }
}

impl FieldComponent {
    pub fn scale_and_offset(&self) -> Option<(u16, i16)> {
        let adjusted_scale = match self.unit.as_ref().map(String::as_ref) {
            Some("100 * m") => 100,
            _ => 1,
        };

        match (self.scale, self.offset) {
            (Some(s), Some(o)) => Some((adjusted_scale * s, o)),
            (None, Some(o)) => Some((adjusted_scale, o)),
            (Some(s), None) => Some((adjusted_scale * s, 0)),
            (None, None) => None
        }
    }

    pub fn unit_type_and_base(&self) -> Option<(&str, &str, &str)> {
        match self.unit {
            Some(ref unit) => match unit.as_str() {
                "C" => Some(("ThermodynamicTemperature", "thermodynamic_temperature", "degree_celsius")),
                "G" => Some(("MagneticFluxDensity", "magnetic_flux_density", "gauss")),
                "J" => Some(("Energy", "energy", "joule")),
                "Pa" => Some(("Pressure", "pressure", "pascal")),
                "V" => Some(("ElectricPotential", "electric_potential", "volt")),
                // TODO would be nice to emit the unit or some other descriptor as "bpm"
                "bpm" => Some(("Frequency", "frequency", "cycle_per_minute")),
                "calories" => Some(("Energy", "energy", "calorie")),
                "deg/s" => Some(("Frequency", "frequency", "hertz")),
                "hr" => Some(("Time", "time", "hour")),
                "kcal" => Some(("Energy", "energy", "kilocalorie")),
                "kg" => Some(("Mass", "mass", "kilogram")),
                "kg/m^3" => Some(("Density", "density", "kilogram_per_cubic_meter")),
                "m" | "100 * m" => Some(("Length", "length", "meter")),
                "m/s" => Some(("Velocity", "velocity", "meter_per_second")),
                "m/s^2" => Some(("Acceleration", "acceleration", "meter_per_second_squared")),
                "min" | "minutes" => Some(("Time", "time", "minute")),
                "mm" => Some(("Length", "length", "millimeter")),
                "mmHg" => Some(("Pressure", "pressure", "millimeter_of_mercury")),
                "ms" => Some(("Time", "time", "millisecond")),
                "radians/second" => Some(("Frequency", "frequency", "hertz")),
                "rpm" => Some(("Frequency", "frequency", "cycle_per_minute")),
                "s" => Some(("Time", "time", "second")),
                "strides/min" => Some(("Frequency", "frequency", "cycle_per_minute")),
                "strokes/min" => Some(("Frequency", "frequency", "cycle_per_minute")),
                "watts" => Some(("Power", "power", "watt")),
                "years" => Some(("Time", "time", "year")),

                // TODO power, but uom doesn't have the unit
                "kcal / day" | "kcal/day" => None,
                "kcal / min" => None,

                // TODO can express this one, but need to adjust the scale
                "g/dL" => None,

                // Energy, but not quite
                "kcal/cycle" => None,

                // Distance, but not quite
                "m/cycle" => None,

                // A frequency, but not including time
                "strokes/lap" => None,

                // These are a little more complicated. Need to be an enum :/
                "% or bpm" => None,
                "% or watts" => None,

                // Enum, not a unit...
                "swim_stroke" => None,

                // These are just values, but it would be nice to annotate them in the output
                "%" => None,
                "2 * cycles (steps)" => None,
                "bytes" => None,
                "counts" => None,
                "cycles" => None,
                "degrees" => None,
                "lengths" => None,
                "percent" => None,
                "radians" => None,
                "semicircles" => None,
                "steps" => None,
                "strides" => None,
                "strokes" => None,

                // calibrated_accel_{x,y,z}
                "g" => None,
                // compressed_calibrated_accel_{x,y,z}
                "mG" => None,
                // oxygen toxicity unit
                "OTUs" => None,
                // intensity factor
                "if" => None,
                // training stress score
                "tss" => None,

                // TODO report new units
                _ => None,
            },
            None => None,
        }
    }
}

impl Serialize for FieldData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let null = ();
        let mut state = serializer.serialize_struct("FieldData", 9)?;
        {
            let field_class = match self.array_length {
                Some(_) => "many",
                _ => "one",
            };

            state.serialize_field("field_type", &self.field_type())?;
            state.serialize_field("field_base_type", &self.field_base_type())?;
            state.serialize_field("field_storage_type", &self.field_storage_type())?;
            state.serialize_field("field_class", &field_class)?;
            state.serialize_field("field_content", self.field_content())?;
            state.serialize_field("conversion_function", &self.conversion_function())?;
            state.serialize_field("to_bits_function", &self.to_bits_function())?;

            match &self.components {
                Components::Some(c) => {
                    state.serialize_field("components", c)?;
                    state.serialize_field("field_data", &null)?;
                },
                Components::None(c) => {
                    state.serialize_field("components", &null)?;
                    state.serialize_field("field_data", c)?;
                },
            };
        }
        state.end()
    }
}
