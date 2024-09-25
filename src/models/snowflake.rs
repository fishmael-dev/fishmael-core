use serde::{
    de::{Deserialize, Error as DeError, Unexpected, Visitor},
    ser::Serialize,
};
use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    marker::PhantomData,
};


#[derive(Debug)]
pub struct UserMarker;

#[derive(Debug)]
pub struct GuildMarker;

#[derive(Debug)]
pub struct ChannelMarker;


#[derive(Debug)]
pub struct Id<T> {
    phantom: PhantomData<T>,
    value: u64,
}


impl<T> Id<T> {
    pub fn new(value: u64) -> Self {
        Self {
            phantom: PhantomData,
            value,
        }
    }

    pub fn timestamp(&self) -> u64 {
        const DISCORD_EPOCH: u64 = 1_420_070_400_000;

        (self.value >> 22) + DISCORD_EPOCH
    }
}


impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.value, f)
    }
}


impl<'de, T> Deserialize<'de> for Id<T> {
    
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
    D: serde::Deserializer<'de> 
    {
        struct IdVisitor<T> {
            phantom: PhantomData<T>,
        }
    
        impl<T> IdVisitor<T> {
            const fn new() -> Self {
                Self {
                    phantom: PhantomData
                }
            }
        }

        impl<'de, T> Visitor<'de> for IdVisitor<T> {
            type Value = Id<T>;

            fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
                formatter.write_str("a discord id")
            }

            fn visit_u64<E: DeError>(self, value: u64) -> Result<Self::Value, E> {
                Ok(Id::new(value))
            }

            fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
                self.visit_u64(
                    value.parse::<u64>()
                        .map_err(|_| E::invalid_value(Unexpected::Str(value), &"a u64 string"))?
                )
            }
        }

        deserializer.deserialize_str(IdVisitor::new())
    }
}


impl<T> Serialize for Id<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_newtype_struct("Id", &self.to_string())
    }
}
