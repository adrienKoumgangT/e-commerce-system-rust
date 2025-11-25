use anyhow::{Error, Result};
use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::services::user::model::user_model::User;
use crate::shared::database::mysql::{GenericRepository, MySqlParam};
use crate::shared::repository::crud_repository::CrudRepository;

#[async_trait]
pub trait UserRepositoryInterface {
    async fn get_user(&self, user_id: i64) -> Result<Option<User>, Error>;

    async fn create_user(&self, user: User) -> Result<User, Error>;

    async fn update_user(&self, user_id: i64, user: User) -> Result<Option<User>, Error>;

    async fn update_user_password(&self, user_id: i64, user_password: Option<String>) -> Result<Option<User>, Error>;

    async fn update_user_profile_pic_url(&self, user_id: i64, profile_pic_url: Option<String>) -> Result<Option<User>, Error>;

    async fn update_user_status(&self, user_id: i64, status: i64) -> Result<Option<User>, Error>;

    async fn delete_user(&self, user_id: i64) -> Result<User, Error>;

    async fn get_all_users(&self, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<User>, Error>;

    async fn get_user_by_username(&self, username: String) -> Result<Option<User>, Error>;

    async fn get_user_by_title(&self, title: String, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<User>, Error>;

    async fn get_user_by_country(&self, country: String, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<User>, Error>;

    async fn get_user_by_country_and_or_title(&self, country: Option<String>, title: Option<String>, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<User>, Error>;

    async fn search_user_by_username(&self, username: String, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<User>, Error>;

    async fn search_user_by_title(&self, title: String, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<User>, Error>;

    async fn search_user_by_country(&self, country: String, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<User>, Error>;

    async fn get_user_by_auth(&self, auth: i64, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<User>, Error>;

    async fn get_user_by_status(&self, status: i64, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<User>, Error>;
}

#[derive(Clone)]
pub struct UserRepository {
    pool: MySqlPool,
}

impl UserRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

impl GenericRepository<User> for UserRepository {
    fn get_pool(&self) -> &MySqlPool {
        &self.pool
    }
}

#[async_trait]
impl UserRepositoryInterface for UserRepository {
    async fn get_user(&self, user_id: i64) -> Result<Option<User>, Error> {
        // CALL app_user_get_by_id(?, ?)
        let params = vec![
            MySqlParam::from(user_id),
            MySqlParam::from(None::<i64>), // meta_user
        ];

        self.call_procedure_for_optional("app_user_get_by_id", params).await
    }

    async fn create_user(&self, user: User) -> Result<User, Error> {
        //   app_user_insert(
        //                  first_name, last_name,
        //                  username, password,
        //                  profile_pic_url,
        //                  auth, status,
        //                  hired_date, title,
        //                  address, country, phone,
        //                  meta_user)
        let params = vec![
            MySqlParam::from(user.first_name),
            MySqlParam::from(user.last_name),
            MySqlParam::from(user.username),
            MySqlParam::from(user.password),
            MySqlParam::from(user.profile_pic_url),
            MySqlParam::from(user.auth),
            MySqlParam::from(user.status),
            MySqlParam::from(user.hired_date),
            MySqlParam::from(user.title),
            MySqlParam::from(user.address),
            MySqlParam::from(user.country),
            MySqlParam::from(user.phone),
            MySqlParam::from(None::<i64>),   // meta_user
        ];

        self.call_procedure_for_one("app_user_insert", params).await
    }

    async fn update_user(&self, user_id: i64, user: User) -> Result<Option<User>, Error> {
        // app_user_update(id,
        //                  first_name, last_name,
        //                  hired_date, title,
        //                  address, country, phone,
        //                  meta_user)
        let params = vec![
            MySqlParam::from(user_id),
            MySqlParam::from(user.first_name),
            MySqlParam::from(user.last_name),
            MySqlParam::from(user.hired_date),
            MySqlParam::from(user.title),
            MySqlParam::from(user.address),
            MySqlParam::from(user.country),
            MySqlParam::from(user.phone),
            MySqlParam::from(None::<i64>), // meta_user
        ];

        self.call_procedure_for_optional("app_user_update", params).await
    }

    async fn update_user_password(&self, user_id: i64, user_password: Option<String>) -> Result<Option<User>, Error> {
        // app_user_update_password(id,
        //                          user_password,
        //                          meta_user)
        let params = vec![
            MySqlParam::from(user_id),
            MySqlParam::from(user_password),
            MySqlParam::from(None::<i64>), // meta_user
        ];

        self.call_procedure_for_optional("app_user_update_password", params).await
    }

    async fn update_user_profile_pic_url(&self, user_id: i64, profile_pic_url: Option<String>) -> Result<Option<User>, Error> {
        // app_user_update_profile_pic_url(id,
        //                          user_password,
        //                          meta_user)
        let params = vec![
            MySqlParam::from(user_id),
            MySqlParam::from(profile_pic_url),
            MySqlParam::from(None::<i64>), // meta_user
        ];

        self.call_procedure_for_optional("app_user_update_profile_pic_url", params).await
    }

    async fn update_user_status(&self, user_id: i64, status: i64) -> Result<Option<User>, Error> {
        // app_user_update_status(id,
        //                          user_status,
        //                          meta_user)
        let params = vec![
            MySqlParam::from(user_id),
            MySqlParam::from(status),
            MySqlParam::from(None::<i64>), // meta_user
        ];

        self.call_procedure_for_optional("app_user_update_status", params).await
    }

    async fn delete_user(&self, user_id: i64) -> Result<User, Error> {
        let params = vec![
            MySqlParam::from(user_id),
            MySqlParam::from(None::<i64>), // meta_user
        ];

        let existing = self.call_procedure_for_optional("app_user_get_by_id", params.clone()).await;

        match existing {
            Ok(existing_opt) => {
                let user = existing_opt.ok_or_else(|| Error::msg("User not found for delete"))?;

                let result = self.call_procedure("app_user_delete_log", params).await;

                match result {
                    Ok(_) => Ok(user),
                    Err(e) => Err(Error::msg(format!("Failed to delete user: {}", e)))
                }
            },
            Err(e) => Err(Error::msg(format!("Failed to load user for delete: {}", e)))
        }
    }

    async fn get_all_users(&self, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<User>, Error> {
        let params = vec![
            MySqlParam::from(limit),
            MySqlParam::from(offset),
            MySqlParam::from(None::<i64>), // meta_user
        ];

        self.call_procedure_for_list("app_user_get_all", params).await
    }

    async fn get_user_by_username(&self, username: String) -> Result<Option<User>, Error> {
        let params = vec![
            MySqlParam::from(username),
            MySqlParam::from(None::<i64>), // meta_user
        ];

        self.call_procedure_for_optional("app_user_get_by_username", params).await
    }

    async fn get_user_by_title(&self, title: String, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<User>, Error> {
        let params = vec![
            MySqlParam::from(title),
            MySqlParam::from(limit),
            MySqlParam::from(offset),
            MySqlParam::from(None::<i64>), // meta_user
        ];

        self.call_procedure_for_list("app_user_get_by_title", params).await
    }

    async fn get_user_by_country(&self, country: String, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<User>, Error> {
        let params = vec![
            MySqlParam::from(country),
            MySqlParam::from(limit),
            MySqlParam::from(offset),
            MySqlParam::from(None::<i64>), // meta_user
        ];

        self.call_procedure_for_list("app_user_get_by_country", params).await
    }

    async fn get_user_by_country_and_or_title(&self, country: Option<String>, title: Option<String>, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<User>, Error> {
        let params = vec![
            MySqlParam::from(country),
            MySqlParam::from(title),
            MySqlParam::from(limit),
            MySqlParam::from(offset),
            MySqlParam::from(None::<i64>), // meta_user
        ];

        self.call_procedure_for_list("app_user_get_by_country_and_or_title", params).await
    }

    async fn search_user_by_username(&self, username: String, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<User>, Error> {
        let params = vec![
            MySqlParam::from(username),
            MySqlParam::from(limit),
            MySqlParam::from(offset),
            MySqlParam::from(None::<i64>), // meta_user
        ];

        self.call_procedure_for_list("app_user_search_by_username", params).await
    }

    async fn search_user_by_title(&self, title: String, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<User>, Error> {
        let params = vec![
            MySqlParam::from(title),
            MySqlParam::from(limit),
            MySqlParam::from(offset),
            MySqlParam::from(None::<i64>), // meta_user
        ];

        self.call_procedure_for_list("app_user_search_by_title", params).await
    }

    async fn search_user_by_country(&self, country: String, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<User>, Error> {
        let params = vec![
            MySqlParam::from(country),
            MySqlParam::from(limit),
            MySqlParam::from(offset),
            MySqlParam::from(None::<i64>), // meta_user
        ];

        self.call_procedure_for_list("app_user_search_by_country", params).await
    }

    async fn get_user_by_auth(&self, auth: i64, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<User>, Error> {
        let params = vec![
            MySqlParam::from(auth),
            MySqlParam::from(limit),
            MySqlParam::from(offset),
            MySqlParam::from(None::<i64>), // meta_user
        ];

        self.call_procedure_for_list("app_user_get_by_auth", params).await
    }

    async fn get_user_by_status(&self, status: i64, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<User>, Error> {
        let params = vec![
            MySqlParam::from(status),
            MySqlParam::from(limit),
            MySqlParam::from(offset),
            MySqlParam::from(None::<i64>), // meta_user
        ];

        self.call_procedure_for_list("app_user_get_by_status", params).await
    }
}
