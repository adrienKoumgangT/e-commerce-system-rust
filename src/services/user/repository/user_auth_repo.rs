use anyhow::{Error, Result};
use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::services::user::model::user_model::{UserAuth};
use crate::shared::database::mysql::{GenericRepository, MySqlParam};
use crate::shared::repository::crud_repository::CrudRepository;

#[async_trait]
pub trait UserAuthRepositoryInterface {
    async fn get_user_auth(&self, user_auth_id: i64) -> Result<Option<UserAuth>, Error>;

    async fn create_user_auth(&self, user_auth: UserAuth) -> Result<UserAuth, Error>;

    async fn update_user_auth(
        &self,
        user_auth_id: i64,
        user_auth: UserAuth,
    ) -> Result<Option<UserAuth>, Error>;

    async fn delete_user_auth(&self, user_auth_id: i64) -> Result<(), Error>;

    async fn get_all_user_auths(&self) -> Result<Vec<UserAuth>, Error>;
}

#[derive(Clone)]
pub struct UserAuthRepository {
    pool: MySqlPool,
}

impl UserAuthRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

impl GenericRepository<UserAuth> for UserAuthRepository {
    fn get_pool(&self) -> &MySqlPool {
        &self.pool
    }
}

#[async_trait]
impl UserAuthRepositoryInterface for UserAuthRepository {
    async fn get_user_auth(&self, user_auth_id: i64) -> Result<Option<UserAuth>, Error> {
        // CALL app_user_auth_get_by_id(?, ?)
        let params = vec![
            MySqlParam::from(user_auth_id),
            MySqlParam::from(None::<i64>), // meta_user
        ];
        
        self.call_procedure_for_optional("app_user_auth_get_by_id", params).await
    }

    async fn create_user_auth(&self, user_auth: UserAuth) -> Result<UserAuth, Error> {
        // CALL app_user_auth_insert(?, ?, ?)
        let params = vec![
            MySqlParam::from(user_auth.name),
            MySqlParam::from(user_auth.description),
            MySqlParam::from(None::<i64>), // meta_user
        ];

        self.call_procedure_for_one("app_user_auth_insert", params).await
    }

    async fn update_user_auth(
        &self,
        user_auth_id: i64,
        user_auth: UserAuth,
    ) -> Result<Option<UserAuth>, Error> {
        // CALL app_user_auth_update(?, ?, ?, ?)
        let params = vec![
            MySqlParam::from(user_auth_id),
            MySqlParam::from(user_auth.name),
            MySqlParam::from(user_auth.description),
            MySqlParam::from(None::<i64>), // meta_user
        ];

        self.call_procedure_for_optional("app_user_auth_update", params).await
    }

    async fn delete_user_auth(&self, user_auth_id: i64) -> Result<(), Error> {
        // CALL app_user_auth_delete(?, ?)
        let params = vec![
            MySqlParam::from(user_auth_id),
            MySqlParam::from(None::<i64>), // meta_user
        ];
        
        self.call_procedure("app_user_auth_delete", params).await
    }

    async fn get_all_user_auths(&self) -> Result<Vec<UserAuth>, Error> {
        // CALL app_user_auth_get_all(?)
        let params = vec![
            MySqlParam::from(None::<i64>), // meta_user
        ];
        
        self.call_procedure_for_list("app_user_auth_get_all", params).await
    }
}
