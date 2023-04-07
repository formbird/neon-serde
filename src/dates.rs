use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JsDate {
    __neon_serde_date: f64,
}

impl JsDate {
    pub fn as_millis(&self) -> i64 {
        self.__neon_serde_date as i64
    }

    pub fn from_millis(millis: i64) -> Self {
        Self {
            __neon_serde_date: millis as f64,
        }
    }
}
