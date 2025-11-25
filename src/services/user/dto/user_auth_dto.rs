use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

use crate::services::user::model::user_model::{User, UserAuth};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserAuthResponse {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

impl UserAuthResponse {
    pub fn from(user_auth: UserAuth) -> Self {
        Self {
            id: user_auth.id.unwrap_or(0),
            name: user_auth.name,
            description: user_auth.description,
        }
    }

    pub fn from_user(user: &User) -> Self {
        Self {
            id: user.auth,
            name: user.auth_name.clone().unwrap_or("user".to_string()),
            description: user.auth_description.clone(),
        }
    }
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserAuthCreateRequest {
    pub name: String,
    pub description: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserAuthUpdateRequest {
    pub name: String,
    pub description: Option<String>,
}

