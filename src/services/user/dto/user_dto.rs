use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use utoipa::{IntoParams, ToSchema};
use crate::services::user::dto::user_auth_dto::UserAuthResponse;
use crate::services::user::dto::user_status_dto::UserStatusResponse;
use crate::services::user::model::user_model::User;


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    // pub email: String,
    pub username: String,
    pub profile_pic_url: Option<String>,

    pub auth: UserAuthResponse,
    pub status: UserStatusResponse,


    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl UserResponse {
    pub fn from(user: User) -> Self {
        let auth = UserAuthResponse::from_user(&user);
        let status = UserStatusResponse::from_user(&user);
        Self {
            id: user.id.unwrap_or(0),
            first_name: user.first_name,
            last_name: user.last_name,
            username: user.username,
            profile_pic_url: user.profile_pic_url,
            auth,
            status,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ManagerUserResponse {
    pub user: UserResponse,

    pub hired_date: Option<DateTime<Utc>>,
    pub title: Option<String>,
}

impl ManagerUserResponse {
    pub fn from(user: User) -> Self {
        let user_response = UserResponse::from(user.clone());
        Self {
            user: user_response,
            hired_date: user.hired_date,
            title: user.title,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CustomerUserResponse {
    pub user: UserResponse,

    pub address: Option<String>,
    pub country: Option<String>,
    pub phone: Option<String>,
}

impl CustomerUserResponse {
    pub fn from(user: User) -> Self {
        let user_response = UserResponse::from(user.clone());
        Self {
            user: user_response,
            address: user.address,
            country: user.country,
            phone: user.phone,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreateRequest {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: Option<String>,
    pub auth: i64,
    pub status: i64,
    pub hired_date: Option<DateTime<Utc>>,
    pub title: Option<String>,
    pub address: Option<String>,
    pub country: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserUpdateRequest {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub hired_date: Option<DateTime<Utc>>,
    pub title: Option<String>,
    pub address: Option<String>,
    pub country: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct SearchCountryRequest {
    #[param(example = "Italy")]
    pub country: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct SearchTitleRequest {
    #[param(example = "abc")]
    pub title: Option<String>,
}


