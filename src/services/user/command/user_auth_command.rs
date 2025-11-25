use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use crate::shared::models::response::PaginationRequest;


#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserAuthGetCommand {
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserAuthCreateCommand{
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserAuthUpdateCommand {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserAuthDeleteCommand {
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserAuthListCommand {
    pub pagination: Option<PaginationRequest>,
}
