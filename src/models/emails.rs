use models::*;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Email {
    pub to: String,
    pub subject: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAddEmail {
    pub to: String,
    pub base_url: String,
    pub token: DeviceConfirmToken,
    pub device_id: DeviceId,
}

impl DeviceAddEmail {
    pub fn new(to: String, base_url: String, token: DeviceConfirmToken, device_id: DeviceId) -> Self {
        Self {
            to,
            base_url,
            token,
            device_id,
        }
    }
}

impl Email {
    pub fn new(to: String, subject: String, text: String) -> Self {
        Self { to, subject, text }
    }
}
