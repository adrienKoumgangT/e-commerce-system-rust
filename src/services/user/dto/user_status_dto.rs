use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use crate::services::user::model::user_model::{User, UserStatus};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserStatusResponse {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

impl UserStatusResponse {
    pub fn from(user_status: UserStatus) -> Self {
        Self {
            id: user_status.id.unwrap_or(0),
            name: user_status.name,
            description: user_status.description,
        }
    }

    pub fn from_user(user: &User) -> Self {
        Self {
            id: user.status,
            name: user.status_name.clone().unwrap_or("active".to_string()),
            description: user.status_description.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserStatusCreateRequest {
    pub name: String,
    pub description: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserStatusUpdateRequest {
    pub name: String,
    pub description: Option<String>,
}

