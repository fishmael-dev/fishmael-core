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
