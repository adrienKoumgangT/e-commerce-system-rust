use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, Row, Error as SqlxError};
use crate::shared::database::mysql::FromSqlRow;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserAuth {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
}

impl UserAuth {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            id: None,
            name,
            description,
        }
    }
}

impl FromSqlRow for UserAuth {
    fn map_row_to_entity(row: MySqlRow, index_map: &HashMap<String, usize>) -> Result<Self, SqlxError> {
        Ok(UserAuth {
            id: row.try_get(index_map["id"])?,
            name: row.try_get(index_map["name"])?,
            description: row.try_get(index_map["description"])?,
        })
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserStatus {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
}

impl UserStatus {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            id: None,
            name,
            description,
        }
    }
}

impl FromSqlRow for UserStatus {
    fn map_row_to_entity(row: MySqlRow, index_map: &HashMap<String, usize>) -> Result<Self, SqlxError> {
        Ok(UserStatus {
            id: row.try_get(index_map["id"])?,
            name: row.try_get(index_map["name"])?,
            description: row.try_get(index_map["description"])?,
        })
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    /// Shared information
    pub id: Option<i64>,
    pub first_name: String,
    pub last_name: String,
    // pub email: String,
    pub username: String,
    pub password: String,
    pub profile_pic_url: Option<String>,
    // pub is_active: bool,

    /// User Authentication Level: user, customer, manager
    pub auth: i64,
    pub auth_name: Option<String>,
    pub auth_description: Option<String>,

    /// User Status Level: active, inactive, locked, deleted, suspended, expired, reseted, unverified, unknown
    pub status: i64,
    pub status_name: Option<String>,
    pub status_description: Option<String>,

    /// Manager
    pub hired_date: Option<DateTime<Utc>>,
    pub title: Option<String>,

    /// Customer
    pub address: Option<String>,
    pub country: Option<String>,
    pub phone: Option<String>,


    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(
        first_name: String, 
        last_name: String, 
        username: String, 
        password: String, 
        auth: i64, 
        status: i64, 
        hired_date: Option<DateTime<Utc>>, 
        title: Option<String>, 
        address: Option<String>, 
        country: Option<String>, 
        phone: Option<String>
    ) -> Self {
        Self {
            id: None,
            first_name,
            last_name,
            username,
            password,
            profile_pic_url: None,
            auth,
            auth_name: None,
            auth_description: None,
            status,
            status_name: None,
            status_description: None,
            hired_date,
            title,
            address,
            country,
            phone,
            created_at: None,
            updated_at: None,
        }
    }
}

impl FromSqlRow for User {
    fn map_row_to_entity(row: MySqlRow, index_map: &HashMap<String, usize>) -> Result<Self, SqlxError> {
        Ok(User {
            id: row.try_get(index_map["id"])?,
            first_name: row.try_get(index_map["first_name"])?,
            last_name: row.try_get(index_map["last_name"])?,
            username: row.try_get(index_map["username"])?,
            password: row.try_get(index_map["password"])?,
            profile_pic_url: row.try_get(index_map["profile_pic_url"])?,
            auth: row.try_get(index_map["auth"])?,
            auth_name: row.try_get(index_map["auth_name"])?,
            auth_description: row.try_get(index_map["auth_description"])?,
            status: row.try_get(index_map["status"])?,
            status_name: row.try_get(index_map["status_name"])?,
            status_description: row.try_get(index_map["status_description"])?,
            hired_date: row.try_get(index_map["hired_date"])?,
            title: row.try_get(index_map["title"])?,
            address: row.try_get(index_map["address"])?,
            country: row.try_get(index_map["country"])?,
            phone: row.try_get(index_map["phone"])?,
            created_at: row.try_get(index_map["created_at"])?,
            updated_at: row.try_get(index_map["updated_at"])?,
        })
    }
}
