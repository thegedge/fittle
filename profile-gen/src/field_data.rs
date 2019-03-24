use inflector::cases::pascalcase::to_pascal_case;

pub struct FieldData {
    pub base_type: String,
    pub array_length: Option<u8>,
    pub scale: Option<u16>,
    pub offset: Option<u16>,
    pub unit: Option<String>,
}

impl FieldData {
    pub fn rust_type(&self) -> String {
        let rust_type = self.rust_base_type();
        if self.array_length.is_some() {
            format!("Vec<{0}>", rust_type)
        } else {
            rust_type
        }
    }

    pub fn conversion_function(&self) -> String {
        let rust_base_type = self.rust_base_type();
        let default_constructor = format!("<{0}>::from", rust_base_type);
        let constructor = match self.base_type.as_str() {
            // These are always annotated with a time unit, but we're using chrono instead of uom
            "date_time" => default_constructor,
            "local_date_time" => default_constructor,
            _ => default_constructor,
        };

        if self.array_length.is_some() {
            format!("content.many().map(|vec| vec.into_iter().map({0}).collect())", constructor)
        } else {
            format!("content.one().map({0})", constructor)
        }
    }

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
}
