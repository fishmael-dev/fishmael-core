use fishmael_model::snowflake::Id;
use itertools::Itertools;
use redis::{Cmd, ToRedisArgs};


pub trait HArgProvider<T: ToRedisArgs> {
    fn to_redis_hargs(self, cmd: &mut Cmd, key: &str) -> ();
}

macro_rules! impl_harg_provider_for {
    ($($t:ty),+) => {
        $(
            impl HArgProvider<$t> for $t {
                fn to_redis_hargs(self, cmd: &mut Cmd, key: &str) -> () {
                    cmd.arg(key).arg(self);
                }        
            }
        )*
    };
}

impl_harg_provider_for!(u8, u16, u32, u64, usize);
impl_harg_provider_for!(bool);
impl_harg_provider_for!(String);

impl<'a> HArgProvider<&'a str> for &str {
    fn to_redis_hargs(self, cmd: &mut Cmd, key: &str) -> () {
        cmd.arg(key).arg(self);
    }
}

impl<T: ToRedisArgs> HArgProvider<Option<T>> for Option<T> {
    fn to_redis_hargs(self, cmd: &mut Cmd, key: &str) -> () {
        if let Some(value) = self {
            cmd.arg(key).arg(value);
        }
    }
}

impl<T> HArgProvider<Id<T>> for Id<T> {
    fn to_redis_hargs(self, cmd: &mut Cmd, key: &str) -> () {
        cmd.arg(key).arg(self);
    }
}

impl<T> HArgProvider<Vec<Id<T>>> for Vec<Id<T>> {
    fn to_redis_hargs(self, cmd: &mut Cmd, key: &str) -> () {
        if !self.is_empty() {
            cmd.arg(key).arg(self.iter().map(|id| id.value()).join(","));
        }
    }
}


pub trait HArgConsumer<T: ToRedisArgs> {
    fn hargs<U: HArgProvider<T>>(self, key: &str, value: U) -> Self;
}

impl<T: ToRedisArgs> HArgConsumer<T> for &mut Cmd {
    fn hargs<U: HArgProvider<T>>(self, key: &str, value: U) -> Self {
        value.to_redis_hargs(self, key);
        self
    }
}
