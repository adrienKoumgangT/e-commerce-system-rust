use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use crate::shared::models::response::PaginationRequest;


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserGetCommand {
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserGetByUsernameCommand {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserGetByTitleCommand {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserGetByCountryCommand {
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserGetBySearchCommand {
    pub country: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreateCommand {
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
pub struct UserUpdateCommand {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub hired_date: Option<DateTime<Utc>>,
    pub title: Option<String>,
    pub address: Option<String>,
    pub country: Option<String>,
    pub phone: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserUpdatePasswordCommand {
    pub code: String,
    pub username: String,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserUpdateProfilePicUrlCommand {
    pub id: i64,
    pub profile_pic_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserUpdateStatusCommand {
    pub id: i64,
    pub status: i64,
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserDeleteCommand {
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserListCommand {
    pub pagination: Option<PaginationRequest>,
}

