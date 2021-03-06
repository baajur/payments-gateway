use chrono::NaiveDateTime;

use validator::Validate;

use models::*;
use schema::users;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename(deserialize = "rawId"))]
    pub id: UserId,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub device_id: Option<DeviceId>,
    pub device_os: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NewUser {
    #[validate(email(code = "not_valid", message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = "1", message = "First name must not be empty"))]
    pub first_name: String,
    #[validate(length(min = "1", message = "Last name must not be empty"))]
    pub last_name: String,
    #[validate(
        custom = "validate_password_len",
        custom = "validate_password_lower_case",
        custom = "validate_password_numbers"
    )]
    pub password: Password,
    pub device_type: DeviceType,
    pub phone: Option<String>,
    pub device_id: DeviceId,
    pub device_os: String,
    pub public_key: DevicePublicKey,
}

#[derive(Serialize, Deserialize, Insertable, Validate, AsChangeset, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[table_name = "users"]
pub struct UpdateUser {
    #[validate(length(min = "1", message = "First name must not be empty"))]
    pub first_name: Option<String>,
    #[validate(length(min = "1", message = "Last name must not be empty"))]
    pub last_name: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Clone)]
pub struct UserDB {
    pub id: UserId,
    pub email: String,
    pub phone: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub device_id: Option<DeviceId>,
    pub device_os: Option<String>,
    pub revoke_before: NaiveDateTime,
}

impl Default for UserDB {
    fn default() -> Self {
        Self {
            id: UserId::default(),
            email: String::default(),
            phone: None,
            first_name: None,
            last_name: None,
            created_at: ::chrono::Utc::now().naive_utc(),
            updated_at: ::chrono::Utc::now().naive_utc(),
            device_id: None,
            device_os: None,
            revoke_before: ::chrono::Utc::now().naive_utc(),
        }
    }
}

impl UserDB {
    pub fn get_full_name(&self) -> String {
        let first_name = self.first_name.clone().unwrap_or("unknown".to_string());
        let last_name = self
            .last_name
            .clone()
            .unwrap_or("unknown".to_string())
            .chars()
            .next()
            .unwrap_or_default();
        format!("{} {}.", first_name, last_name)
    }
}

#[derive(Debug, Insertable, Clone, Default)]
#[table_name = "users"]
pub struct NewUserDB {
    pub id: UserId,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub device_id: Option<DeviceId>,
    pub device_os: Option<String>,
}

impl From<NewUserDB> for UserDB {
    fn from(new_user: NewUserDB) -> Self {
        Self {
            id: new_user.id,
            email: new_user.email,
            first_name: Some(new_user.first_name),
            last_name: Some(new_user.last_name),
            phone: new_user.phone,
            ..Default::default()
        }
    }
}

impl From<User> for NewUserDB {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            first_name: user.first_name.unwrap_or_default(),
            last_name: user.last_name.unwrap_or_default(),
            phone: user.phone,
            device_id: user.device_id,
            device_os: user.device_os,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct ResetPassword {
    pub email: String,
    pub device: DeviceType,
}

#[derive(Serialize, Debug, Clone)]
pub struct ResendEmailVerify {
    pub email: String,
    pub device: DeviceType,
}

#[derive(Serialize, Debug, Clone, Validate)]
pub struct ResetPasswordConfirm {
    pub token: String,
    #[validate(
        custom = "validate_password_len",
        custom = "validate_password_lower_case",
        custom = "validate_password_numbers"
    )]
    pub password: Password,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChangePassword {
    pub old_password: Password,
    pub new_password: Password,
}
