use axum::{Router, routing::{get}, extract::{Path, State}, Json, http::StatusCode};
use crate::shared::state::AppState;
use crate::services::user::command::user_auth_command::{UserAuthCreateCommand, UserAuthDeleteCommand, UserAuthGetCommand, UserAuthListCommand, UserAuthUpdateCommand};
use crate::services::user::dto::user_auth_dto::{UserAuthCreateRequest, UserAuthResponse, UserAuthUpdateRequest};
use crate::services::user::service::user_auth_service::{UserAuthService, UserAuthServiceInterface};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_user_auths).post(post_user_auth))
        .route("/{user_auth_id}", get(get_user_auth_by_id).put(put_user_auth).delete(delete_user_auth))
}


#[utoipa::path(
    get,
    path = "/api/user/auth",
    responses(
        (status = StatusCode::OK, description = "List of User Auth", body = Vec<UserAuthResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR)
    ),
    tag = "UserAuth"
)]
pub async fn get_user_auths(State(state): State<AppState>) -> Result<Json<Vec<UserAuthResponse>>, StatusCode> {
    let user_auth_list_command = UserAuthListCommand { pagination: None };
    let user_auth_service = UserAuthService::from_app_state(&state);
    let user_auths = user_auth_service.get_all(user_auth_list_command).await;
    match user_auths {
        Ok(user_auths) => Ok(Json(user_auths)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


#[utoipa::path(
    post,
    path = "/api/user/auth",
    responses(
        (status = StatusCode::OK, description = "User Auth successfully created", body = UserAuthResponse),
        (status = StatusCode::BAD_REQUEST),
        (status = StatusCode::INTERNAL_SERVER_ERROR)
    ),
    tag = "UserAuth"
)]
pub async fn post_user_auth(
    State(state): State<AppState>,
    Json(user_auth_create_request): Json<UserAuthCreateRequest>
) -> Result<Json<UserAuthResponse>, StatusCode> {
    let user_auth_create_command = UserAuthCreateCommand {name: user_auth_create_request.name, description: user_auth_create_request.description};
    let user_auth_service = UserAuthService::from_app_state(&state);
    let user_auth = user_auth_service.create(user_auth_create_command).await;
    match user_auth {
        Ok(user_auth) => Ok(Json(user_auth)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


#[utoipa::path(
    get,
    path = "/api/user/auth/{user_auth_id}",
    responses(
        (status = StatusCode::OK, description = "User Auth found successfully", body = UserAuthResponse),
        (status = StatusCode::NOT_FOUND, description = "User Auth not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR)
    ),
    tag = "UserAuth"
)]
pub async fn get_user_auth_by_id(
    Path(user_auth_id): Path<i64>,
    State(state): State<AppState>
) -> Result<Json<UserAuthResponse>, StatusCode> {
    let user_auth_get_command = UserAuthGetCommand{ id: user_auth_id };
    let user_auth_service = UserAuthService::from_app_state(&state);
    let user_auth = user_auth_service.get(user_auth_get_command).await;
    match user_auth {
        Ok(user_auth) => {
            match user_auth {
                Some(user_auth) => Ok(Json(user_auth)),
                None => Err(StatusCode::NOT_FOUND),
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


#[utoipa::path(
    put,
    path = "/api/user/auth/{user_auth_id}",
    responses(
        (status = StatusCode::OK, description = "User Auth successfully modified", body = UserAuthResponse),
        (status = StatusCode::NOT_FOUND, description = "User Auth not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR)
    ),
    tag = "UserAuth"
)]
pub async fn put_user_auth(
    Path(user_auth_id): Path<i64>,
    State(state): State<AppState>,
    Json(user_auth_update_request): Json<UserAuthUpdateRequest>
) -> Result<Json<UserAuthResponse>, StatusCode> {
    let user_auth_update_command = UserAuthUpdateCommand {
        id: user_auth_id,
        name: user_auth_update_request.name,
        description: user_auth_update_request.description
    };
    let user_auth_service = UserAuthService::from_app_state(&state);
    let user_auth = user_auth_service.update(user_auth_update_command).await;
    match user_auth {
        Ok(user_auth) => Ok(Json(user_auth)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


#[utoipa::path(
    delete,
    path = "/api/user/auth/{user_auth_id}",
    responses(
        (status = StatusCode::OK, description = "User Auth successfully deleted"),
        (status = StatusCode::NOT_FOUND, description = "User Auth not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR)
    ),
    tag = "UserAuth"
)]
pub async fn delete_user_auth(
    Path(user_auth_id): Path<i64>,
    State(state): State<AppState>
) -> Result<StatusCode, StatusCode> {
    let user_auth_delete_command = UserAuthDeleteCommand{ id: user_auth_id };
    let user_auth_service = UserAuthService::from_app_state(&state);
    let result = user_auth_service.delete(user_auth_delete_command).await;
    match result {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

