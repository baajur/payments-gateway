use std::time::SystemTime;

use models::*;

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum DeviceType {
    Ios,
    Android,
    Web,
    Other,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostSessionsRequest {
    pub email: String,
    pub password: Password,
    pub device_type: DeviceType,
    pub device_os: Option<String>,
    pub device_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostSessionsOauthRequest {
    pub oauth_token: OauthToken,
    pub oauth_provider: Provider,
    pub device_type: DeviceType,
    pub device_os: Option<String>,
    pub device_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostUsersRequest {
    pub email: String,
    pub password: Password,
    pub first_name: String,
    pub last_name: String,
    pub device_type: DeviceType,
    pub device_os: Option<String>,
    pub device_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostUsersConfirmEmailRequest {
    pub email_confirm_token: EmailConfirmToken,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostAccountsRequest {
    pub id: AccountId,
    pub user_id: UserId,
    pub currency: Currency,
    pub name: String,
}

impl From<PostAccountsRequest> for CreateAccount {
    fn from(req: PostAccountsRequest) -> Self {
        Self {
            id: req.id,
            name: req.name,
            currency: req.currency,
            user_id: req.user_id,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PutAccountsRequest {
    pub name: String,
}

impl From<PutAccountsRequest> for UpdateAccount {
    fn from(req: PutAccountsRequest) -> Self {
        Self { name: req.name }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetUsersAccountsParams {
    pub limit: i64,
    pub offset: AccountId,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostTransactionsRequest {
    pub user_id: UserId,
    pub from: AccountId,
    pub to: Receipt,
    pub to_type: ReceiptType,
    pub to_currency: Currency,
    pub value: Amount,
    pub fee: Amount,
    pub hold_until: Option<SystemTime>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PutTransactionsRequest {
    pub status: TransactionStatus,
}

impl From<PutTransactionsRequest> for TransactionStatus {
    fn from(req: PutTransactionsRequest) -> Self {
        req.status
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetUsersTransactionsParams {
    pub limit: i64,
    pub offset: TransactionId,
}
