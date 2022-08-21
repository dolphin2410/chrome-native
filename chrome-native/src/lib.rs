use serde::{Deserialize, Serialize};
use std::{any::Any, error::Error};

pub const ERRORCODE_OK: i32 = 0;
pub const ERRORCODE_FAIL: i32 = -1;
pub trait Plugin: Any + Send + Sync {
    fn handle_command(&self, command: String) -> Result<String, Box<dyn Error>>;
}

#[derive(thiserror::Error, Debug)]
pub enum ChromeNativeErrors {
    #[error("Not the right type")]
    NotRightType,
}

#[cfg(feature = "serde")]
pub fn parse_data<'a, T>(data: &'a str) -> Result<T, Box<impl Error>>
where
    T: Serialize + Deserialize<'a>,
{
    serde_json::from_str::<T>(data).map_err(|_| Box::new(ChromeNativeErrors::NotRightType))
}

#[cfg(feature = "macros")]
#[allow(unused_imports)]
#[macro_use]
extern crate chrome_native_macros;
#[cfg(feature = "macros")]
pub use chrome_native_macros::*;
