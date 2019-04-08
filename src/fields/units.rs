use serde::{
    Serialize,
    Serializer,

    ser::SerializeStruct,
};

use uom::si::{
    SI,
    Unit,
};

pub type QuantityValueType = f64;

// TODO allow customization of the output unit

macro_rules! unit {
    ( $type:tt, $uom_module:tt, $uom_base:tt ) => {
        use uom::si::$uom_module;

        #[derive(Debug)]
        pub struct $type($uom_module::$type<SI<QuantityValueType>, QuantityValueType>);

        impl $type {
            pub fn new<U, V>(v: V) -> $type
                where
                    V: Into<QuantityValueType>,
                    U: $uom_module::Unit + $uom_module::Conversion<QuantityValueType>,
            {
                $type($uom_module::$type::new::<U>(v.into()))
            }
        }

        impl Serialize for $type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
            {
                let mut state = serializer.serialize_struct("Quantity", 2)?;
                state.serialize_field("value", &self.0.get::<$uom_module::$uom_base>())?;
                state.serialize_field("units", <$uom_module::$uom_base as Unit>::abbreviation())?;
                state.end()
            }
        }
    };
}

unit!(Acceleration, acceleration, meter_per_second_squared);
unit!(Density, density, kilogram_per_cubic_meter);
unit!(ElectricPotential, electric_potential, volt);
unit!(Energy, energy, kilocalorie);
unit!(Frequency, frequency, cycle_per_minute);
unit!(Length, length, meter);
unit!(MagneticFluxDensity, magnetic_flux_density, gauss);
unit!(Mass, mass, kilogram);
unit!(Power, power, watt);
unit!(Pressure, pressure, pascal);
unit!(ThermodynamicTemperature, thermodynamic_temperature, degree_celsius);
unit!(Time, time, second);
unit!(Velocity, velocity, meter_per_second);
