use inflector::cases::pascalcase::to_pascal_case;

pub struct FieldData {
    pub base_type: String,
    pub array_length: Option<u8>,
    pub scale: Option<u16>,
    pub offset: Option<u16>,
    pub unit: Option<String>,
}

// TODO memoize things that get called several times

impl FieldData {
    pub fn rust_type(&self) -> String {
        let rust_type = if let Some((unit_type, _, _)) = self.unit_type_and_base() {
            format!("crate::fields::{0}", unit_type)
        } else {
            self.adjusted_rust_base_type()
        };

        if self.array_length.is_some() {
            format!("Vec<{0}>", rust_type)
        } else {
            rust_type
        }
    }

    pub fn conversion_function(&self) -> String {
        let rust_base_type = self.adjusted_rust_base_type();
        let default_constructor = self.default_constructor();
        let constructor = match self.base_type.as_str() {
            // These are always annotated with a time unit, but we're using chrono instead of uom
            "date_time" => default_constructor,
            "local_date_time" => default_constructor,

            _ => match self.unit_type_and_base() {
                Some((unit_type, uom_base, unit_base)) => {
                    format!(
                        "|v| crate::fields::{0}::new::<uom::si::{1}::{2}, {3}>(({4})(v))",
                        unit_type,
                        uom_base,
                        unit_base,
                        rust_base_type,
                        default_constructor,
                    )
                },
                None => default_constructor,
            },
        };

        if self.array_length.is_some() {
            format!("content.many().map(|vec| vec.into_iter().map({0}).collect())", constructor)
        } else {
            format!("content.one().map({0})", constructor)
        }
    }

    // The rust_base_type adjusted for scale, offsets, etc
    fn adjusted_rust_base_type(&self) -> String {
        if self.scale.is_some() || self.offset.is_some() {
            "f64".to_string()
        } else {
            self.rust_base_type()
        }
    }

    // The base type based on what was in the profile spreadsheet
    fn rust_base_type(&self) -> String {
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

    fn default_constructor(&self) -> String {
        let rust_base_type = self.rust_base_type();
        if let Some((scale, offset)) = self.scale_and_offset() {
            let adjusted_rust_base_type = self.adjusted_rust_base_type();
            format!(
                "|v| {{ <{1}>::from(<{0}>::from(v)) / {2}.0 - {3}.0 }}",
                rust_base_type,
                adjusted_rust_base_type,
                scale,
                offset,
            )
        } else {
            format!("<{0}>::from", rust_base_type)
        }
    }

    fn scale_and_offset(&self) -> Option<(u16, u16)> {
        let adjusted_scale = match self.unit.as_ref().map(AsRef::as_ref) {
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

    fn unit_type_and_base(&self) -> Option<(&str, &str, &str)> {
        match self.unit {
            Some(ref unit) => match unit.as_str() {
                "%" => None,
                "% or bpm" => None,
                "% or watts" => None,
                "2 * cycles (steps)" => None,
                "C" => None,
                "G" => None,
                "J" => None,
                "OTUs" => None,
                "Pa" => None,
                "Units" => None,
                "V" => None,
                "bpm" => None,
                "bytes" => None,
                "calories" => None,
                "counts" => None,
                "cycles" => None,
                "deg/s" => None,
                "degrees" => None,
                "g" => None,
                "g/dL" => None,
                "hr" => None,
                "kcal" => None,
                "kcal / day" => None,
                "kcal / min" => None,
                "kcal/cycle" => None,
                "kcal/day" => None,
                "kg" => None,
                "kg/m^3" => None,
                "lengths" => None,
                "m" => Some(("Length", "length", "meter")),
                "100 * m" => Some(("Length", "length", "meter")),
                "m/cycle" => None,
                "m/s" => None,
                "m/s^2" => None,
                "mG" => None,
                "min" => None,
                "minutes" => None,
                "mm" => None,
                "mmHg" => None,
                "ms" => None,
                "percent" => None,
                "radians" => None,
                "radians/second" => None,
                "rpm" => None,
                "s" => {
                    match self.base_type.as_str() {
                        "date_time" => None,
                        "local_date_time" => None,
                        _ => Some(("Time", "time", "second")),
                    }
                },
                "semicircles" => None,
                "steps" => None,
                "strides" => None,
                "strides/min" => None,
                "strokes" => None,
                "strokes/lap" => None,
                "strokes/min" => None,
                "swim_stroke" => None,
                "tss" => None,
                "watts" => None,
                "years" => None,
                _ => None,
            },
            None => None,
        }
    }
}
