use anyhow::{Error, Result};
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
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
use crate::shared::database::redis::{delete_key, get_key, set_key};

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
    redis_pool: Option<Pool<RedisConnectionManager>>,
}

impl UserAuthService {
    pub fn new(user_auth_repo: UserAuthRepository, redis_pool: Option<Pool<RedisConnectionManager>>) -> Self {
        Self {
            user_auth_repo,
            redis_pool
        }
    }
    
    pub fn from_app_state(app_state: &AppState) -> Self {
        let auth_repo = UserAuthRepository::new(app_state.mysql_pool.clone());
        Self::new(auth_repo, Option::from(app_state.redis_pool.clone()))
    }

    pub fn redis_key_single_ttl(&self) -> Option<u64> {
        Some(60*60)
    }

    pub fn form_redis_key_single(&self, key: &i64) -> String {
        format!("user:auth:{}", key)
    }

    pub fn redis_key_list_ttl(&self) -> Option<u64> {
        Some(60*60)
    }

    pub fn form_redis_key_list(&self) -> String {
        "user:auth:list".to_string()
    }
}

#[async_trait]
impl UserAuthServiceInterface for UserAuthService {

    async fn get(&self, user_auth_get_command: UserAuthGetCommand) -> Result<Option<UserAuthResponse>, Error> {
        if let Some(redis_pool) = &self.redis_pool {
            let key = self.form_redis_key_single(&user_auth_get_command.id);
            let user_auth_cache: Option<UserAuthResponse> = get_key(&redis_pool, key.as_str()).await?;
            if let Some(user_auth_cache) = user_auth_cache {
                return Ok(Some(user_auth_cache));
            }
        }

        let user_auth = self.user_auth_repo.get_user_auth(user_auth_get_command.id).await;
        match user_auth {
            Ok(user_auth) => {
                match user_auth {
                    Some(user_auth) => {
                        if let Some(redis_pool) = &self.redis_pool {
                            let key = self.form_redis_key_single(&user_auth_get_command.id);
                            let _: () = set_key(&redis_pool, key.as_str(), &user_auth.clone(), self.redis_key_single_ttl()).await?;
                        }
                        Ok(Some(UserAuthResponse::from(user_auth)))
                    },
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
            Ok(user_auth) => {
                let user_auth_response = UserAuthResponse::from(user_auth);
                if let Some(redis_pool) = &self.redis_pool {
                    let key = self.form_redis_key_single(&user_auth_response.id);
                    let _: () = set_key(&redis_pool, key.as_str(), &user_auth_response, self.redis_key_single_ttl()).await?;
                }
                Ok(user_auth_response)
            },
            Err(_) => Err(Error::msg("Error creating user auth")),
        }
    }

    async fn update(&self, user_auth_update_command: UserAuthUpdateCommand) -> Result<UserAuthResponse, Error> {
        let user_auth_update = UserAuth::new(user_auth_update_command.name, user_auth_update_command.description);
        let user_auth = self.user_auth_repo.update_user_auth(user_auth_update_command.id, user_auth_update).await;
        match user_auth {
            Ok(user_auth) => {
                match user_auth {
                    Some(user_auth) => {
                        let user_auth_response = UserAuthResponse::from(user_auth);
                        if let Some(redis_pool) = &self.redis_pool {
                            let key = self.form_redis_key_single(&user_auth_response.id);
                            let _: () = set_key(&redis_pool, key.as_str(), &user_auth_response, self.redis_key_single_ttl()).await?;
                        }
                        Ok(user_auth_response)
                    },
                    None => Err(Error::msg("User auth not found")),
                }           
            },
            Err(_) => Err(Error::msg("Error updating user auth")),
        }
    }

    async fn delete(&self, user_auth_delete_command: UserAuthDeleteCommand) -> Result<(), Error> {
        let user_auth = self.user_auth_repo.delete_user_auth(user_auth_delete_command.id).await;
        if let Some(redis_pool) = &self.redis_pool {
            let key = self.form_redis_key_single(&user_auth_delete_command.id);
            let _: () = delete_key(&redis_pool, key.as_str()).await?;
        }
        match user_auth {
            Ok(()) => Ok(()),
            Err(_) => Err(Error::msg("Error deleting user auth")),
        }
    }

    async fn get_all(&self, _: UserAuthListCommand) -> Result<Vec<UserAuthResponse>, Error> {
        if let Some(redis_pool) = &self.redis_pool {
            let user_auths_cache: Option<Vec<UserAuthResponse>> = get_key(&redis_pool, self.form_redis_key_list().as_str()).await?;
            if let Some(user_auths_cache) = user_auths_cache {
                return Ok(user_auths_cache);
            }
        }

        let user_auths = self.user_auth_repo.get_all_user_auths().await;
        match user_auths {
            Ok(user_auths) => {
                let user_auths_response: Vec<UserAuthResponse> = user_auths.into_iter().map(UserAuthResponse::from).collect();
                if let Some(redis_pool) = &self.redis_pool {
                    let _: () = set_key(&redis_pool, self.form_redis_key_list().as_str(), &user_auths_response, self.redis_key_list_ttl()).await?;
                }
                Ok(user_auths_response)
            },
            Err(_) => Err(Error::msg("Error getting all user auths")),
        }
    }

}
