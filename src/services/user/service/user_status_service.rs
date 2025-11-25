use anyhow::{Error, Result};
use async_trait::async_trait;
use crate::services::user::command::user_status_command::{UserStatusCreateCommand, UserStatusDeleteCommand, UserStatusGetCommand, UserStatusListCommand, UserStatusUpdateCommand};
use crate::services::user::dto::user_status_dto::{UserStatusResponse};
use crate::services::user::model::user_model::UserStatus;
use crate::services::user::repository::user_status_repo::{UserStatusRepository, UserStatusRepositoryInterface};

#[async_trait]
pub trait UserStatusServiceInterface {
    async fn get(&self, user_status_get_command: UserStatusGetCommand) -> Result<Option<UserStatusResponse>, Error>;

    async fn create(&self, user_status_create_command: UserStatusCreateCommand) -> Result<UserStatusResponse, Error>;

    async fn update(&self, user_status_update_command: UserStatusUpdateCommand) -> Result<UserStatusResponse, Error>;

    async fn delete(&self, user_status_delete_command: UserStatusDeleteCommand) -> Result<(), Error>;

    async fn get_all(&self, _: UserStatusListCommand) -> Result<Vec<UserStatusResponse>, Error>;
}

#[derive(Clone)]
pub struct UserStatusService {
    user_status_repo: UserStatusRepository,
}

impl UserStatusService {
    pub fn new(user_status_repo: UserStatusRepository) -> Self {
        Self { user_status_repo }
    }
    
    pub fn from_app_state(app_state: &crate::shared::state::AppState) -> Self {
        let status_repo = UserStatusRepository::new(app_state.mysql_pool.clone());
        Self::new(status_repo)
    }
}

#[async_trait]
impl UserStatusServiceInterface for UserStatusService {

    async fn get(&self, user_status_get_command: UserStatusGetCommand) -> Result<Option<UserStatusResponse>, Error> {
        let user_status = self.user_status_repo.get_user_status(user_status_get_command.id).await;
        match user_status {
            Ok(user_status) => {
                match user_status {
                    Some(user_status) => Ok(Some(UserStatusResponse::from(user_status))),
                    None => Err(Error::msg("User status not found")),
                }
            },
            Err(_) => Err(Error::msg("Error during get user status. Please check if user status exists in database. If not, please create new user status using /user-status/create endpoint. If yes, please check if user status id is correct. If yes, please try again. If not, please contact support.")),
        }
    }

    async fn create(&self, user_status_create_command: UserStatusCreateCommand) -> Result<UserStatusResponse, Error> {
        let user_status_create = UserStatus::new(user_status_create_command.name, user_status_create_command.description);
        let user_status = self.user_status_repo.create_user_status(UserStatus::from(user_status_create)).await;
        match user_status {
            Ok(user_status) => Ok(UserStatusResponse::from(user_status)),
            Err(_) => Err(Error::msg("Error creating user status")),
        }
    }

    async fn update(&self, user_status_update_command: UserStatusUpdateCommand) -> Result<UserStatusResponse, Error> {
        let user_status_update = UserStatus::new(user_status_update_command.name, user_status_update_command.description);
        let user_status = self.user_status_repo.update_user_status(user_status_update_command.id, user_status_update).await;
        match user_status {
            Ok(user_status) => {
                match user_status {
                    Some(user_status) => Ok(UserStatusResponse::from(user_status)),
                    None => Err(Error::msg("User status not found")),
                }
            },
            Err(_) => Err(Error::msg("Error updating user status")),
        }
    }
    
    async fn delete(&self, user_status_delete_command: UserStatusDeleteCommand) -> Result<(), Error> {
        let user_status = self.user_status_repo.delete_user_status(user_status_delete_command.id).await;
        match user_status {
            Ok(()) => Ok(()),
            Err(_) => Err(Error::msg("Error deleting user status")),
        }
    }
    
    async fn get_all(&self, _: UserStatusListCommand) -> Result<Vec<UserStatusResponse>, Error> {
        let user_statuses = self.user_status_repo.get_all_user_status().await;
        match user_statuses {
            Ok(user_statuses) => Ok(user_statuses.into_iter().map(UserStatusResponse::from).collect()),
            Err(_) => Err(Error::msg("Error getting all user statuses")),
        }
    }

}
