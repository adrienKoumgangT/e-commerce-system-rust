use anyhow::Result;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use sqlx::MySqlPool;
use crate::shared::configuration::AppConfig;
use crate::shared::database::mysql as my_mysql;
use crate::shared::database::redis as my_redis;
// use crate::shared::metrics::prometheus::Metrics;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub mysql_pool: MySqlPool,
    pub redis_pool: Pool<RedisConnectionManager>,
    // pub metrics: Metrics,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self> {
        let config_clone = config.clone();
        
        let mysql_pool = my_mysql::connect(&config_clone.database.mysql.unwrap()).await?;
        let redis_pool = my_redis::connect(&config_clone.database.redis.unwrap()).await?;
        // let metrics = Metrics::new();

        Ok(Self {
            config,
            mysql_pool,
            redis_pool,
            // metrics,
        })
    }
}
