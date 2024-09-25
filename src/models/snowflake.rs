use serde::{
    de::{Deserialize, Error as DeError, Unexpected, Visitor},
    ser::Serialize,
};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};


#[derive(Debug)]
pub struct Id(u64);


impl Id {
    pub fn timestamp(&self) -> u64 {
        const DISCORD_EPOCH: u64 = 1_420_070_400_000;

        (&self.0 >> 22) + DISCORD_EPOCH
    }
}


impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}


impl<'de> Deserialize<'de> for Id {
    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
    D: serde::Deserializer<'de> 
    {
        struct IdVisitor {}
    
        impl IdVisitor {
            const fn new() -> Self {
                Self {}
            }
        }

        impl<'de> Visitor<'de> for IdVisitor {
            type Value = Id;

            fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
                formatter.write_str("a discord id")
            }

            fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
                println!("WHOA");
                Ok(
                    Id (
                        value.parse()
                            .map_err(|_| E::invalid_value(Unexpected::Str(value), &"a u64 string"))?
                    )
                )
            }
        }

        deserializer.deserialize_str(IdVisitor::new())
    }
}


impl Serialize for Id {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_newtype_struct("Id", &self.to_string())
    }
}
