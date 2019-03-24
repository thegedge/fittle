macro_rules! from_impl {
    ( $into_type:ty, $( $enums:tt )|+ ) => {
        from_impl!($into_type, $($enums)|+, |v| <$into_type>::from(v));
    };

    ( $generic_param:tt, $into_type:ty, $( $enums:tt )|+, $conversion:expr ) => {
        impl<$generic_param> From<$crate::fields::FieldContent> for $into_type {
            fn from(fc: $crate::fields::FieldContent) -> Self {
                ($conversion)($generic_param::from(fc))
            }
        }
    };

    ( $into_type:ty, $( $enums:tt )|+, $conversion:expr ) => {
        impl From<$crate::fields::FieldContent> for $into_type {
            fn from(fc: $crate::fields::FieldContent) -> $into_type {
                match fc {
                    $(
                        $crate::fields::FieldContent::$enums(v) => ($conversion)(v),
                    )*
                    v => panic!("cannot convert {:?} into {}", v, stringify!($into_type)),
                }
            }
        }
    };
}
