use axum::Router;
use crate::shared::state::AppState;
use crate::services::user::controller::{
    user_controller::routes as user_routes,
    user_auth_controller::routes as user_auth_routes,
    user_status_controller::routes as user_status_routes
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(user_routes())
        // .nest("/", )
        .nest("/auth", user_auth_routes())
        .nest("/status", user_status_routes())
}

