use std::fmt::Error;

use itertools::Itertools;
use redis::{Cmd, ToRedisArgs};


pub trait HArgProvider<T: ToRedisArgs> {
    fn to_arg(self) -> Result<impl ToRedisArgs, Error>;

    fn to_hargs_for(self, cmd: &mut Cmd, key: &str) -> ()
    where
        Self: Sized
    {
        if let Ok(arg) = self.to_arg() {
            cmd.arg(key).arg(arg);
        }
    }        
}

macro_rules! impl_harg_provider_for {
    ($($t:ty),+) => {
        $(
            impl HArgProvider<$t> for $t {
                fn to_arg(self) -> Result<impl ToRedisArgs, Error> {
                    Ok(self)
                }
            }
        )*
    };
}

impl_harg_provider_for!(u8, u16, u32, u64, usize);
impl_harg_provider_for!(bool);
impl_harg_provider_for!(String);

impl<'a> HArgProvider<&'a str> for &str {
    fn to_arg(self) -> Result<impl ToRedisArgs, Error> {
        Ok(self)
    }
}

impl<T: ToRedisArgs> HArgProvider<Option<T>> for Option<T> {
    fn to_arg(self) -> Result<impl ToRedisArgs, Error> {
        self.ok_or(Error)
    }
}

impl<T: ToString, U: ToRedisArgs> HArgProvider<U> for Vec<T> {
    fn to_arg(self) -> Result<impl ToRedisArgs, Error> {
        Ok(
            self.into_iter()
                .map(|i| i.to_string())
                .join(",")
        )
    }
} 

pub trait HArgConsumer<T: ToRedisArgs> {
    fn hargs<U: HArgProvider<T>>(self, key: &str, value: U) -> Self;
}

impl<T: ToRedisArgs> HArgConsumer<T> for &mut Cmd {
    fn hargs<U: HArgProvider<T>>(self, key: &str, value: U) -> Self {
        value.to_hargs_for(self, key);
        self
    }
}
