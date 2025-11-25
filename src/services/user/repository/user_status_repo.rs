use anyhow::{Error, Result};
use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::services::user::model::user_model::UserStatus;
use crate::shared::database::mysql::{
    GenericRepository,
    MySqlParam,
};
use crate::shared::repository::crud_repository::CrudRepository;


#[async_trait]
pub trait UserStatusRepositoryInterface {
    async fn get_user_status(&self, user_status_id: i64) -> Result<Option<UserStatus>, Error>;

    async fn create_user_status(&self, user_status: UserStatus) -> Result<UserStatus, Error>;

    async fn update_user_status(&self, user_status_id: i64, user_status: UserStatus) -> Result<Option<UserStatus>, Error>;

    async fn delete_user_status(&self, user_status_id: i64) -> Result<(), Error>;

    async fn get_all_user_status(&self) -> Result<Vec<UserStatus>, Error>;
}

#[derive(Clone)]
pub struct UserStatusRepository {
    pool: MySqlPool,
}

impl UserStatusRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

/// Hook this repo into the generic MySQL infrastructure.
impl GenericRepository<UserStatus> for UserStatusRepository {
    fn get_pool(&self) -> &MySqlPool {
        &self.pool
    }
}

#[async_trait]
impl UserStatusRepositoryInterface for UserStatusRepository {
    async fn get_user_status(&self, user_status_id: i64) -> Result<Option<UserStatus>, Error> {
        let params = vec![
            MySqlParam::from(user_status_id),
            MySqlParam::from(None::<i64>), // meta_user
        ];
        
        self.call_procedure_for_optional("app_user_status_get_by_id", params).await
    }

    async fn create_user_status(&self, user_status: UserStatus) -> Result<UserStatus, Error> {
        let params = vec![
            MySqlParam::from(user_status.name),
            MySqlParam::from(user_status.description),
            MySqlParam::from(None::<i64>), // meta_user
        ];

        self.call_procedure_for_one("app_user_status_insert", params).await
    }

    async fn update_user_status(
        &self,
        user_status_id: i64,
        user_status: UserStatus,
    ) -> Result<Option<UserStatus>, Error> {
        let params = vec![
            MySqlParam::from(user_status_id),
            MySqlParam::from(user_status.name),
            MySqlParam::from(user_status.description),
            MySqlParam::from(None::<i64>), // meta_user
        ];

        self.call_procedure_for_optional("app_user_status_update", params).await
    }

    async fn delete_user_status(&self, user_status_id: i64) -> Result<(), Error> {
        let params = vec![
            MySqlParam::from(user_status_id),
            MySqlParam::from(None::<i64>), // meta_user
        ];
        
        self.call_procedure("app_user_status_delete", params).await
    }

    async fn get_all_user_status(&self) -> Result<Vec<UserStatus>, Error> {
        let params = vec![
            MySqlParam::from(None::<i64>), // meta_user
        ];
        
        self.call_procedure_for_list("app_user_status_get_all", params).await
    }
}
