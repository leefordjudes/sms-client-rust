use std::{result, str::FromStr};

use regex::Regex;
use serde::{de, Deserialize, Serialize};

use crate::error::{Error, ErrorKind};

lazy_static::lazy_static! {
    pub static ref MOBILE_NO: Regex = Regex::new(r"^[6-9][0-9]{9}$").unwrap();
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MobileNumber(String);

impl FromStr for MobileNumber {
    type Err = Error;
    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let val = MOBILE_NO
            .is_match(s)
            .then(|| s)
            .ok_or(Error::new("Invalid mobile number", ErrorKind::InvalidData))?;
        let mobile_no = MobileNumber(format!("+91{}",val.to_lowercase()));
        Ok(mobile_no)
    }
}

impl ToString for MobileNumber {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl<'de> Deserialize<'de> for MobileNumber {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}
