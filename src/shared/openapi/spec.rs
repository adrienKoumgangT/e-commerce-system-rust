
use utoipa::{
    OpenApi,
};

use crate::services::user::controller::{user_controller, user_auth_controller, user_status_controller};
use crate::services::user::dto::user_dto::{UserCreateRequest, UserUpdateRequest, UserResponse, SearchCountryRequest, SearchTitleRequest};
use crate::services::user::dto::user_auth_dto::{UserAuthCreateRequest, UserAuthUpdateRequest, UserAuthResponse};
use crate::services::user::dto::user_status_dto::{UserStatusCreateRequest, UserStatusUpdateRequest, UserStatusResponse};

#[derive(OpenApi)]
#[openapi(
    info(version = "1.0.0", title = "E-Commerce API", description = "E-Commerce API description"),
    tags(
        (name = "User", description = "User API endpoints"),
        (name = "UserAuth", description = "User Auth API endpoints"),
        (name = "UserStatus", description = "User Status API endpoints")
    ),
    paths(
        user_controller::get_users, user_controller::post_user,
        user_controller::get_user_by_id, user_controller::put_user, user_controller::delete_user,
        user_auth_controller::get_user_auths, user_auth_controller::post_user_auth,
        user_auth_controller::get_user_auth_by_id, user_auth_controller::put_user_auth, user_auth_controller::delete_user_auth,
        user_status_controller::get_user_statuses, user_status_controller::post_user_status,
        user_status_controller::get_user_status_by_id, user_status_controller::put_user_status, user_status_controller::delete_user_status
    ),
    components(
        schemas(
            UserCreateRequest, UserUpdateRequest, UserResponse, SearchCountryRequest, SearchTitleRequest,
            UserAuthCreateRequest, UserAuthUpdateRequest, UserAuthResponse,
            UserStatusCreateRequest, UserStatusUpdateRequest, UserStatusResponse
        )
    )
)]
pub struct ApiDoc;
