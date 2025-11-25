use axum::{Router, routing::{get}, extract::{Path, State}, Json, http::StatusCode};
use crate::shared::state::AppState;
use crate::services::user::command::user_status_command::{UserStatusCreateCommand, UserStatusDeleteCommand, UserStatusGetCommand, UserStatusListCommand, UserStatusUpdateCommand};
use crate::services::user::dto::user_status_dto::{UserStatusCreateRequest, UserStatusResponse};
use crate::services::user::service::user_status_service::{UserStatusService, UserStatusServiceInterface};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_user_statuses).post(post_user_status))
        .route("/{user_status_id}", get(get_user_status_by_id).put(put_user_status).delete(delete_user_status))
}


#[utoipa::path(
    get,
    path = "/api/user/status",
    responses(
        (status = StatusCode::OK, description = "List of User Status", body = Vec<UserStatusResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR)
    ),
    tag = "UserStatus"
)]
pub async fn get_user_statuses(State(state): State<AppState>) -> Result<Json<Vec<UserStatusResponse>>, StatusCode> {
    let user_status_list_command = UserStatusListCommand { pagination: None };
    let user_status_service = UserStatusService::from_app_state(&state);
    let user_statuses = user_status_service.get_all(user_status_list_command).await;
    match user_statuses {
        Ok(user_statuses) => Ok(Json(user_statuses)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


#[utoipa::path(
    post,
    path = "/api/user/status",
    responses(
        (status = StatusCode::CREATED, description = "User Status successfully created", body = UserStatusResponse),
        (status = StatusCode::BAD_REQUEST),
        (status = StatusCode::INTERNAL_SERVER_ERROR)
    ),
    tag = "UserStatus"
)]
pub async fn post_user_status(
    State(state): State<AppState>,
    Json(user_status_create_request): Json<UserStatusCreateRequest>
) -> Result<Json<UserStatusResponse>, StatusCode> {
    let user_status_create_command = UserStatusCreateCommand {
        name: user_status_create_request.name,
        description: user_status_create_request.description
    };
    let user_status_service = UserStatusService::from_app_state(&state);
    let user_status = user_status_service.create(user_status_create_command).await;
    match user_status {
        Ok(user_status) => Ok(Json(user_status)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


#[utoipa::path(
    get,
    path = "/api/user/status/{user_status_id}",
    responses(
        (status = StatusCode::OK, description = "User Status", body = UserStatusResponse),
        (status = StatusCode::NOT_FOUND, description = "User Status not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR)
    ),
    tag = "UserStatus"
)]
pub async fn get_user_status_by_id(
    Path(user_status_id): Path<i64>,
    State(state): State<AppState>
) -> Result<Json<UserStatusResponse>, StatusCode> {
    let user_status_get_command = UserStatusGetCommand{ id: user_status_id };
    let user_status_service = UserStatusService::from_app_state(&state);
    let user_status = user_status_service.get(user_status_get_command).await;
    match user_status {
        Ok(user_status) => {
            match user_status {
                Some(user_status) => Ok(Json(user_status)),
                None => Err(StatusCode::NOT_FOUND),
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


#[utoipa::path(
    put,
    path = "/api/user/status/{user_status_id}",
    responses(
        (status = StatusCode::OK, description = "User Status successfully modified", body = UserStatusResponse),
        (status = StatusCode::NOT_FOUND, description = "User Status not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR)
    ),
    tag = "UserStatus"
)]
pub async fn put_user_status(
    Path(user_status_id): Path<i64>,
    State(state): State<AppState>,
    Json(user_status_update_request): Json<UserStatusCreateRequest>
) -> Result<Json<UserStatusResponse>, StatusCode> {
    let user_status_update_command = UserStatusUpdateCommand {
        id: user_status_id,
        name: user_status_update_request.name,
        description: user_status_update_request.description
    };
    let user_status_service = UserStatusService::from_app_state(&state);
    let user_status = user_status_service.update(user_status_update_command).await;
    match user_status {
        Ok(user_status) => Ok(Json(user_status)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


#[utoipa::path(
    delete,
    path = "/api/user/status/{user_status_id}",
    responses(
        (status = StatusCode::OK, description = "User Status successfully deleted"),
        (status = StatusCode::NOT_FOUND, description = "User Status not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR)
    ),
    tag = "UserStatus"
)]
pub async fn delete_user_status(
    Path(user_status_id): Path<i64>,
    State(state): State<AppState>
) -> Result<StatusCode, StatusCode> {
    let user_status_delete_command = UserStatusDeleteCommand{ id: user_status_id };
    let user_status_service = UserStatusService::from_app_state(&state);
    let result = user_status_service.delete(user_status_delete_command).await;
    match result {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

