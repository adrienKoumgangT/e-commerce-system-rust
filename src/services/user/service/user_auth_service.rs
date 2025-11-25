use anyhow::{Error, Result};
use async_trait::async_trait;
use crate::shared::state::AppState;
use crate::services::user::command::user_auth_command::{
    UserAuthCreateCommand, 
    UserAuthDeleteCommand, 
    UserAuthGetCommand, 
    UserAuthListCommand, 
    UserAuthUpdateCommand
};
use crate::services::user::dto::user_auth_dto::{UserAuthResponse};
use crate::services::user::model::user_model::UserAuth;
use crate::services::user::repository::user_auth_repo::{UserAuthRepository, UserAuthRepositoryInterface};

#[async_trait]
pub trait UserAuthServiceInterface {
    async fn get(&self, user_auth_get_command: UserAuthGetCommand) -> Result<Option<UserAuthResponse>, Error>;

    async fn create(&self, user_auth_create_command: UserAuthCreateCommand) -> Result<UserAuthResponse, Error>;
    
    async fn update(&self, user_auth_update_command: UserAuthUpdateCommand) -> Result<UserAuthResponse, Error>;
    
    async fn delete(&self, user_auth_delete_command: UserAuthDeleteCommand) -> Result<(), Error>;
    
    async fn get_all(&self, _: UserAuthListCommand) -> Result<Vec<UserAuthResponse>, Error>;
}


#[derive(Clone)]
pub struct UserAuthService {
    user_auth_repo: UserAuthRepository,
}

impl UserAuthService {
    pub fn new(user_auth_repo: UserAuthRepository) -> Self {
        Self { user_auth_repo }
    }
    
    pub fn from_app_state(app_state: &AppState) -> Self {
        let auth_repo = UserAuthRepository::new(app_state.mysql_pool.clone());
        Self::new(auth_repo)
    }
}

#[async_trait]
impl UserAuthServiceInterface for UserAuthService {

    async fn get(&self, user_auth_get_command: UserAuthGetCommand) -> Result<Option<UserAuthResponse>, Error> {
        let user_auth = self.user_auth_repo.get_user_auth(user_auth_get_command.id).await;
        match user_auth {
            Ok(user_auth) => {
                match user_auth {
                    Some(user_auth) => Ok(Some(UserAuthResponse::from(user_auth))),
                    None => Err(Error::msg("User auth not found")),
                }
            },
            Err(_) => Err(Error::msg("Error during get user auth. Please check if user auth exists in database. If not, please create new user auth using /user/auth/create endpoint. If yes, please check if user auth id is correct. If yes, please try again. If not, please contact support.")),
        }
    }

    async fn create(&self, user_auth_create_command: UserAuthCreateCommand) -> Result<UserAuthResponse, Error> {
        let user_auth_create = UserAuth::new(user_auth_create_command.name, user_auth_create_command.description);
        let user_auth = self.user_auth_repo.create_user_auth(user_auth_create).await;
        match user_auth {
            Ok(user_auth) => Ok(UserAuthResponse::from(user_auth)),
            Err(_) => Err(Error::msg("Error creating user auth")),
        }
    }

    async fn update(&self, user_auth_update_command: UserAuthUpdateCommand) -> Result<UserAuthResponse, Error> {
        let user_auth_update = UserAuth::new(user_auth_update_command.name, user_auth_update_command.description);
        let user_auth = self.user_auth_repo.update_user_auth(user_auth_update_command.id, user_auth_update).await;
        match user_auth {
            Ok(user_auth) => {
                match user_auth {
                    Some(user_auth) => Ok(UserAuthResponse::from(user_auth)),
                    None => Err(Error::msg("User auth not found")),
                }           
            },
            Err(_) => Err(Error::msg("Error updating user auth")),
        }
    }

    async fn delete(&self, user_auth_delete_command: UserAuthDeleteCommand) -> Result<(), Error> {
        let user_auth = self.user_auth_repo.delete_user_auth(user_auth_delete_command.id).await;
        match user_auth {
            Ok(()) => Ok(()),
            Err(_) => Err(Error::msg("Error deleting user auth")),
        }
    }

    async fn get_all(&self, _: UserAuthListCommand) -> Result<Vec<UserAuthResponse>, Error> {
        let user_auths = self.user_auth_repo.get_all_user_auths().await;
        match user_auths {
            Ok(user_auths) => Ok(user_auths.into_iter().map(UserAuthResponse::from).collect()),
            Err(_) => Err(Error::msg("Error getting all user auths")),
        }
    }

}
