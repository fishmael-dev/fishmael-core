use itertools::Itertools;
use redis::{Cmd, ToRedisArgs};


pub trait ToRedisHArgs<T: ToRedisArgs> {
    fn write_redis_hargs(&self, key: &str, cmd: &mut Cmd) -> ();
}

macro_rules! impl_to_redis_hargs_for {
    (&$lt:lifetime $t:ty) => {
        impl <$lt> ToRedisHArgs<&$lt $t> for &$lt $t {
            fn write_redis_hargs(&self, key: &str, cmd: &mut Cmd) -> () {
                key.write_redis_args(cmd);
                self.write_redis_args(cmd);
            }
        }
    };
    ($t:ty) => {
        impl ToRedisHArgs<$t> for $t {
            fn write_redis_hargs(&self, key: &str, cmd: &mut Cmd) -> () {
                key.write_redis_args(cmd);
                self.write_redis_args(cmd);
            }
        }
    };
}

impl_to_redis_hargs_for!(u8);
impl_to_redis_hargs_for!(u16);
impl_to_redis_hargs_for!(u32);
impl_to_redis_hargs_for!(u64);
impl_to_redis_hargs_for!(usize);
impl_to_redis_hargs_for!(bool);
impl_to_redis_hargs_for!(String);
impl_to_redis_hargs_for!(&'a str);

impl<T: ToRedisArgs> ToRedisHArgs<Option<T>> for Option<T> {
    fn write_redis_hargs(&self, key: &str, cmd: &mut Cmd) -> () {
        if let Some(value) = self {
            key.write_redis_args(cmd);
            value.write_redis_args(cmd);
        }
    }
}

impl<T: ToString + ToRedisArgs> ToRedisHArgs<T> for Vec<T> {
    fn write_redis_hargs(&self, key: &str, cmd: &mut Cmd) -> () {
        if !self.is_empty() {
            key.write_redis_args(cmd);
            self.into_iter().map(ToString::to_string).join(",").write_redis_args(cmd);
        }
    }
} 
