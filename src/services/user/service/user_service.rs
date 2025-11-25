use anyhow::{Error, Result};
use async_trait::async_trait;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use crate::services::user::command::user_command::{
    UserCreateCommand, 
    UserDeleteCommand, 
    UserGetByCountryCommand, 
    UserGetBySearchCommand, 
    UserGetByTitleCommand, 
    UserGetByUsernameCommand, 
    UserGetCommand, 
    UserListCommand, 
    UserUpdateCommand, 
    UserUpdatePasswordCommand
};
use crate::services::user::dto::user_dto::UserResponse;
use crate::services::user::model::user_model::User;
use crate::services::user::repository::user_repo::{UserRepository, UserRepositoryInterface};
use crate::shared::database::redis::{delete_key, get_key, set_key};
use crate::shared::state::AppState;

#[async_trait]
pub trait UserServiceInterface {
    async fn get(&self, user_get_command: UserGetCommand) -> Result<Option<UserResponse>, Error>;
    
    async fn create(&self, user_create_command: UserCreateCommand) -> Result<UserResponse, Error>;
    
    async fn update(&self, user_update_command: UserUpdateCommand) -> Result<Option<UserResponse>, Error>;
    
    async fn update_password(&self, user_update_password_command: UserUpdatePasswordCommand) -> Result<Option<UserResponse>, Error>;
    
    async fn update_profile_pic_url(&self, user_id: i64, profile_pic_url: Option<String>) -> Result<Option<UserResponse>, Error>;
    
    async fn update_status(&self, user_id: i64, status: i64) -> Result<Option<UserResponse>, Error>;
    
    async fn delete(&self, user_delete_command: UserDeleteCommand) -> Result<(), Error>;

    async fn get_all(&self, user_list_command: UserListCommand) -> Result<Vec<UserResponse>, Error>;
    
    async fn get_by_username(&self, user_get_by_username_command: UserGetByUsernameCommand) -> Result<Option<UserResponse>, Error>;
    
    async fn get_by_title(&self, user_get_by_title_command: UserGetByTitleCommand, user_list_command: UserListCommand) -> Result<Vec<UserResponse>, Error>;
    
    async fn get_by_country(&self, user_get_by_country_command: UserGetByCountryCommand, user_list_command: UserListCommand) -> Result<Vec<UserResponse>, Error>;
    
    async fn get_by_search(&self, user_get_by_search: UserGetBySearchCommand, user_list_command: UserListCommand) -> Result<Vec<UserResponse>, Error>;
}


#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
    redis_pool: Option<Pool<RedisConnectionManager>>,
}

impl UserService {
    pub fn new(user_repo: UserRepository, redis_pool: Option<Pool<RedisConnectionManager>>) -> Self {
        Self { 
            user_repo, 
            redis_pool 
        }
    }
    
    pub fn from_app_state(app_state: &AppState) -> Self {
        let user_repo = UserRepository::new(app_state.mysql_pool.clone());
        Self::new(user_repo, Option::from(app_state.redis_pool.clone()))
    }

    pub fn redis_key_single_ttl(&self) -> Option<u64> {
        Some(60*60)
    }

    pub fn form_redis_key_single(&self, key: &i64) -> String {
        format!("user:{}", key)
    }

    pub fn redis_key_list_count_ttl(&self) -> Option<u64> {
        Some(60*10)
    }

    pub fn form_redis_key_list_count(&self) -> String {
        "user:list:count".to_string()
    }
}

#[async_trait]
impl UserServiceInterface for UserService {
    async fn get(&self, user_get_command: UserGetCommand) -> Result<Option<UserResponse>, Error> {
        if let Some(redis_pool) = &self.redis_pool {
            let key = self.form_redis_key_single(&user_get_command.id);
            let user_cache: Option<UserResponse> = get_key(&redis_pool, key.as_str()).await?;
            if let Some(user_cache) = user_cache {
                return Ok(Some(user_cache));
            }
        }
        
        let user = self.user_repo.get_user(user_get_command.id).await;
        match user {
            Ok(user) => match user {
                Some(user) => Ok(Some(UserResponse::from(user))),
                None => Err(Error::msg("User not found")),
            },
            Err(_) => Err(Error::msg("Error during get user.")),
        }
    }

    async fn create(&self, user_create_command: UserCreateCommand) -> Result<UserResponse, Error> {
        let user_create = User::new(
            user_create_command.first_name, 
            user_create_command.last_name, 
            user_create_command.username, 
            user_create_command.password.unwrap_or_else(|| "1234567890".to_string()), 
            user_create_command.auth, 
            user_create_command.status, 
            user_create_command.hired_date, 
            user_create_command.title, 
            user_create_command.address, 
            user_create_command.country, 
            user_create_command.phone
        );
        let user = self.user_repo.create_user(user_create).await;
        match user {
            Ok(user) => {
                let user_response = UserResponse::from(user);
                if let Some(redis_pool) = &self.redis_pool {
                    let key = self.form_redis_key_single(&user_response.id);
                    let _: () = set_key(&redis_pool, key.as_str(), &user_response, self.redis_key_single_ttl()).await?;
                }
                Ok(user_response)
            },
            Err(_) => Err(Error::msg("Error during create user.")),
        }
    }

    async fn update(&self, user_update_command: UserUpdateCommand) -> Result<Option<UserResponse>, Error> {
        let user_update = User::new(
            user_update_command.first_name, 
            user_update_command.last_name, 
            "".to_string(),
            "".to_string(), 
            0, 
            0, 
            user_update_command.hired_date, 
            user_update_command.title, 
            user_update_command.address, 
            user_update_command.country, 
            user_update_command.phone
        );
        let user = self.user_repo.update_user(user_update_command.id, user_update).await;
        match user {
            Ok(user) => match user {
                Some(user) => {
                    let user_response = UserResponse::from(user);
                    if let Some(redis_pool) = &self.redis_pool {
                        let key = self.form_redis_key_single(&user_response.id);
                        let _: () = set_key(&redis_pool, key.as_str(), &user_response, self.redis_key_single_ttl()).await?;   
                    }
                    Ok(Some(user_response))
                },
                None => Err(Error::msg("User not found")),
            },
            Err(_) => Err(Error::msg("Error during update user.")),
        }
    }

    async fn update_password(&self, user_update_password_command: UserUpdatePasswordCommand) -> Result<Option<UserResponse>, Error> {
        let user = self.user_repo.get_user_by_username(user_update_password_command.username.clone()).await;
        
        match user {
            Ok(user) => match user {
                Some(user) => {
                    let user = self.user_repo.update_user_password(user.id.unwrap(), user_update_password_command.password).await;
                    match user {
                        Ok(user) => match user {
                            Some(user) => {
                                let user_response = UserResponse::from(user);
                                if let Some(redis_pool) = &self.redis_pool {
                                    let key = self.form_redis_key_single(&user_response.id);
                                    let _: () = set_key(&redis_pool, key.as_str(), &user_response, self.redis_key_single_ttl()).await?;
                                }
                                Ok(Some(user_response))
                            },
                            None => Err(Error::msg("User not found")),
                        },
                        Err(_) => Err(Error::msg("Error during update user password.")),
                    }
                },
                None => Err(Error::msg("User not found")),
            },
            Err(_) => Err(Error::msg("Error during get user by username. "))
        }
    }

    async fn update_profile_pic_url(&self, user_id: i64, profile_pic_url: Option<String>) -> Result<Option<UserResponse>, Error> {
        let user = self.user_repo.update_user_profile_pic_url(user_id, profile_pic_url).await;
        
        match user {
            Ok(user) => match user {
                Some(user) => {
                    let user_response = UserResponse::from(user);
                    if let Some(redis_pool) = &self.redis_pool {
                        let key = self.form_redis_key_single(&user_response.id);
                        let _: () = set_key(&redis_pool, key.as_str(), &user_response, self.redis_key_single_ttl()).await?;
                    }
                    Ok(Some(user_response))
                },
                None => Err(Error::msg("User not found")),
            },
            Err(_) => Err(Error::msg("Error during update user profile pic url.")),
        }
    }

    async fn update_status(&self, user_id: i64, status: i64) -> Result<Option<UserResponse>, Error> {
        let user = self.user_repo.update_user_status(user_id, status).await;
        match user {
            Ok(user) => match user {
                Some(user) => {
                    let user_response = UserResponse::from(user);
                    if let Some(redis_pool) = &self.redis_pool {
                        let key = self.form_redis_key_single(&user_response.id);
                        let _: () = set_key(&redis_pool, key.as_str(), &user_response, self.redis_key_single_ttl()).await?;
                    }
                    Ok(Some(user_response))
                },
                None => Err(Error::msg("User not found")),
            },
            Err(_) => Err(Error::msg("Error during update user status.")),       
        }
    }

    async fn delete(&self, user_delete_command: UserDeleteCommand) -> Result<(), Error> {
        let result = self.user_repo.delete_user(user_delete_command.id).await;
        if let Some(redis_pool) = &self.redis_pool {
            let key = self.form_redis_key_single(&user_delete_command.id);
            let _: () = delete_key(&redis_pool, key.as_str()).await?;
        }
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::msg("Error during delete user.")),       
        }
    }

    async fn get_all(&self, user_list_command: UserListCommand) -> Result<Vec<UserResponse>, Error> {
        let mut limit: Option<u32> = None;
        let mut offset: Option<u32> = None;

        if let Some(pagination) = user_list_command.pagination {
            limit = pagination.page_size;

            if let (Some(page_size), Some(page)) = (pagination.page_size, pagination.page) {
                offset = Some(page * page_size);
            }
        }
        let users = self.user_repo.get_all_users(limit, offset).await;
        match users {
            Ok(users) => Ok(users.into_iter().map(UserResponse::from).collect()),
            Err(_) => Err(Error::msg("Error during get all users.")),       
        }
    }

    async fn get_by_username(&self, user_get_by_username_command: UserGetByUsernameCommand) -> Result<Option<UserResponse>, Error> {
        let user = self.user_repo.get_user_by_username(user_get_by_username_command.username).await;
        match user {
            Ok(user) => match user {
                Some(user) => Ok(Some(UserResponse::from(user))),
                None => Err(Error::msg("User not found")),
            },
            Err(_) => Err(Error::msg("Error during get user by username.")),       
        }
    }

    async fn get_by_title(&self, user_get_by_title_command: UserGetByTitleCommand, user_list_command: UserListCommand) -> Result<Vec<UserResponse>, Error> {
        let mut limit: Option<u32> = None;
        let mut offset: Option<u32> = None;

        if let Some(pagination) = user_list_command.pagination {
            limit = pagination.page_size;

            if let (Some(page_size), Some(page)) = (pagination.page_size, pagination.page) {
                offset = Some(page * page_size);
            }
        }
        
        let users = self.user_repo.get_user_by_title(user_get_by_title_command.title, limit, offset).await;
        match users {
            Ok(users) => Ok(users.into_iter().map(UserResponse::from).collect()),
            Err(_) => Err(Error::msg("Error during get users by title.")),       
        }
    }

    async fn get_by_country(&self, user_get_by_country_command: UserGetByCountryCommand, user_list_command: UserListCommand) -> Result<Vec<UserResponse>, Error> {
        let mut limit: Option<u32> = None;
        let mut offset: Option<u32> = None;

        if let Some(pagination) = user_list_command.pagination {
            limit = pagination.page_size;

            if let (Some(page_size), Some(page)) = (pagination.page_size, pagination.page) {
                offset = Some(page * page_size);
            }
        }
        
        let users = self.user_repo.get_user_by_country(user_get_by_country_command.country, limit, offset).await;
        match users {
            Ok(users) => Ok(users.into_iter().map(UserResponse::from).collect()),
            Err(_) => Err(Error::msg("Error during get users by country.")),       
        }       
    }

    async fn get_by_search(&self, user_get_by_search: UserGetBySearchCommand, user_list_command: UserListCommand) -> Result<Vec<UserResponse>, Error> {
        let mut limit: Option<u32> = None;
        let mut offset: Option<u32> = None;

        if let Some(pagination) = user_list_command.pagination {
            limit = pagination.page_size;

            if let (Some(page_size), Some(page)) = (pagination.page_size, pagination.page) {
                offset = Some(page * page_size);
            }
        }
        
        let users = self.user_repo.get_user_by_country_and_or_title(user_get_by_search.country, user_get_by_search.title, limit, offset).await;

        match users {
            Ok(users) => Ok(users.into_iter().map(UserResponse::from).collect()),
            Err(_) => Err(Error::msg("Error during search users")),
        }
    }
}
