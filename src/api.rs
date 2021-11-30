use serde::{Deserialize, Serialize};

use crate::{Client, Result, Error, ErrorKind, MobileNumber};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsInput {
    pub mobile: MobileNumber,
    pub otp: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmsData {
    pub sender: String,
    pub template_id: String,
    pub mobile: String,
    pub authkey: String,
    pub extra_param: String,
}

pub struct Handler<'client> {
    client: &'client Client,
}

impl<'client> Handler<'client> {
    pub(crate) fn new(client: &'client Client) -> Self {
        Self { client }
    }

    pub async fn send_otp(&self, data: SmsInput) -> Result<String> {
        let otp = data.otp.to_string();
        if otp.len() != 6 {
            return Err(Error::new("OTP must be 6 digits", ErrorKind::InvalidData));
        }
        let param_data = SmsData {
            sender: self.client.sender.clone(),
            template_id: self.client.template_id.clone(),
            mobile: format!("+91{}", data.mobile.to_string()),
            authkey: self.client.auth_key.clone(),
            extra_param: format!("{{\"OTP\": \"{}\"}}", data.otp),
        };
        self.client.get("otp".to_string(), Some(&param_data)).await
    }
}
