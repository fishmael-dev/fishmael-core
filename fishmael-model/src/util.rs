macro_rules! impl_serde_for_flags {
    ($( $t:ty ),+) => {
        $(
            impl<'de> serde::de::Deserialize<'de> for $t {
                
                fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    struct FlagVisitor;
    
                    impl<'de> serde::de::Visitor<'de> for FlagVisitor {
                        type Value = $t;

                        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                            formatter.write_str("an integer flag")
                        }

                        fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                            self.visit_u64(
                                value.parse()
                                    .map_err(|_| E::invalid_value(
                                        serde::de::Unexpected::Str(value),
                                        &"a u64 string"
                                    ))?
                            )
                        }

                        fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
                            Ok(<$t>::from_bits_truncate(value))
                        }
                    }

                    deserializer.deserialize_any(FlagVisitor)
                }
            }
            
            impl serde::ser::Serialize for $t {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::ser::Serializer,
                {
                    serializer.serialize_u64(self.bits())
                }
            }
        )*     
    };
}

pub(crate) use impl_serde_for_flags;


pub mod null_bool {
    use std::fmt::{Formatter, Result as FmtResult};
    
    use serde::{de::{Error as DeError, Visitor}, Serializer};
    
    struct NullBoolVisitor;
    
    use serde::Deserializer;

    impl<'de> Visitor<'de> for NullBoolVisitor {
        type Value = bool;
    
        fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
            formatter.write_str("null")
        }
    
        fn visit_none<E: DeError>(self) -> Result<Self::Value, E> {
            Ok(true)
        }
    
        fn visit_unit<E: DeError>(self) -> Result<Self::Value, E> {
            Ok(true)
        }

        fn visit_bool<E: DeError>(self, v: bool) -> Result<Self::Value, E> {
            Ok(v)
        }
    }
    
    pub fn serialize<S: Serializer>(_: &bool, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_none()
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
        deserializer.deserialize_option(NullBoolVisitor)
    }
}


pub(crate) fn is_false(value: &bool) -> bool {
    !value
}