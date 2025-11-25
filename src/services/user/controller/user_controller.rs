use axum::{Router, routing::{get}, extract::{Path, State}, Json, http::StatusCode};
use axum::extract::Query;
use crate::shared::state::AppState;
use crate::services::user::command::user_command::{UserCreateCommand, UserDeleteCommand, UserGetByCountryCommand, UserGetBySearchCommand, UserGetByTitleCommand, UserGetByUsernameCommand, UserGetCommand, UserListCommand, UserUpdateCommand, UserUpdatePasswordCommand};
use crate::services::user::dto::user_dto::{SearchCountryRequest, SearchTitleRequest, UserCreateRequest, UserResponse, UserUpdateRequest};
use crate::services::user::service::user_service::{UserService, UserServiceInterface};
use crate::shared::models::response::PaginationRequest;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_users).post(post_user))
        .route("/{user_id}", get(get_user_by_id).put(put_user).delete(delete_user))
}


#[utoipa::path(
    get,
    path = "/api/user",
    params(
        PaginationRequest,
        SearchCountryRequest,
        SearchTitleRequest
    ),
    responses(
        (status = StatusCode::OK, description = "List of User", body = Vec<UserResponse>),
        (status = StatusCode::INTERNAL_SERVER_ERROR)
    ),
    tag = "User"
)]
pub async fn get_users(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationRequest>,
    Query(search_country): Query<SearchCountryRequest>,
    Query(search_title): Query<SearchTitleRequest>
) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    let user_list_command = UserListCommand { pagination: Some(pagination) };
    let user_service = UserService::from_app_state(&state);

    if search_country.country.is_some() && search_title.title.is_some() {
        let user_get_by_search = UserGetBySearchCommand{ country: search_country.country, title: search_title.title };

        let users = user_service.get_by_search(user_get_by_search, user_list_command).await;

        match users {
            Ok(users) => return Ok(Json(users)),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else if search_country.country.is_some() {
        let user_get_by_country_command = UserGetByCountryCommand{ country: search_country.country.unwrap() };

        let users = user_service.get_by_country(user_get_by_country_command, user_list_command).await;

        match users {
            Ok(users) => return Ok(Json(users)),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else if search_title.title.is_some() {
        let user_get_by_title_command = UserGetByTitleCommand{ title: search_title.title.unwrap() };

        let users = user_service.get_by_title(user_get_by_title_command, user_list_command).await;

        match users {
            Ok(users) => return Ok(Json(users)),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        let users = user_service.get_all(user_list_command).await;

        match users {
            Ok(users) => Ok(Json(users)),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}


#[utoipa::path(
    post,
    path = "/api/user",
    responses(
        (status = StatusCode::OK, description = "User successfully created", body = UserResponse),
        (status = StatusCode::BAD_REQUEST),
        (status = StatusCode::INTERNAL_SERVER_ERROR)
    ),
    tag = "User"
)]
pub async fn post_user(
    State(state): State<AppState>,
    Json(user_create_request): Json<UserCreateRequest>
) -> Result<Json<UserResponse>, StatusCode> {
    let user_create_command = UserCreateCommand {
        first_name: user_create_request.first_name,
        last_name: user_create_request.last_name,
        username: user_create_request.username,
        password: user_create_request.password,
        auth: user_create_request.auth,
        status: user_create_request.status,
        hired_date: user_create_request.hired_date,
        title: user_create_request.title,
        address: user_create_request.address,
        country: user_create_request.country,
        phone: user_create_request.phone,
    };
    let user_service = UserService::from_app_state(&state);
    let user = user_service.create(user_create_command).await;
    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


#[utoipa::path(
    get,
    path = "/api/user/{user_id}",
    responses(
        (status = StatusCode::OK, description = "User found successfully", body = UserResponse),
        (status = StatusCode::NOT_FOUND, description = "User not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR)
    ),
    tag = "User"
)]
pub async fn get_user_by_id(
    Path(user_id): Path<i64>,
    State(state): State<AppState>
) -> Result<Json<UserResponse>, StatusCode> {
    let user_get_command = UserGetCommand{ id: user_id };
    let user_service = UserService::from_app_state(&state);
    let user = user_service.get(user_get_command).await;
    match user {
        Ok(user) => {
            match user {
                Some(user) => Ok(Json(user)),
                None => Err(StatusCode::NOT_FOUND),
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


#[utoipa::path(
    put,
    path = "/api/user/{user_id}",
    responses(
        (status = StatusCode::OK, description = "User not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR)
    ),
    tag = "User"
)]
pub async fn put_user(
    Path(user_id): Path<i64>,
    State(state): State<AppState>,
    Json(user_update_request): Json<UserUpdateRequest>
) -> Result<Json<UserResponse>, StatusCode> {
    let user_update_command = UserUpdateCommand {
        id: user_id,
        first_name: user_update_request.first_name,
        last_name: user_update_request.last_name,
        hired_date: user_update_request.hired_date,
        title: user_update_request.title,
        address: user_update_request.address,
        country: user_update_request.country,
        phone: user_update_request.phone,
    };
    let user_service = UserService::from_app_state(&state);
    let user = user_service.update(user_update_command).await;
    match user {
        Ok(user) => {
            match user {
                Some(user) => Ok(Json(user)),
                None => Err(StatusCode::NOT_FOUND),
            }
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


#[utoipa::path(
    delete,
    path = "/api/user/{user_id}",
    responses(
        (status = StatusCode::OK, description = "User successfully deleted"),
        (status = StatusCode::NOT_FOUND, description = "User not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR)
    ),
    tag = "User"
)]
pub async fn delete_user(
    Path(user_id): Path<i64>,
    State(state): State<AppState>
) -> Result<StatusCode, StatusCode> {
    let user_delete_command = UserDeleteCommand{ id: user_id };
    let user_service = UserService::from_app_state(&state);
    let result = user_service.delete(user_delete_command).await;
    match result {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

