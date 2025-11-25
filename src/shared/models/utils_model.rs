use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlRow, Row, Error as SqlxError};
use crate::shared::database::mysql::FromSqlRow;


#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OptIdModel {
    pub id: Option<i64>,
}

impl OptIdModel {
    pub fn new(id: Option<i64>) -> Self {
        Self { id }
    }
}

impl FromSqlRow for OptIdModel {
    fn map_row_to_entity(row: MySqlRow, index_map: &HashMap<String, usize>) -> Result<Self, SqlxError> {
        Ok(OptIdModel { id: row.try_get(index_map["id"])? })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct IdModel {
    pub id: i64,
}

impl IdModel {
    pub fn new(id: i64) -> Self {
        Self { id }
    }
}

impl FromSqlRow for IdModel {
    fn map_row_to_entity(row: MySqlRow, index_map: &HashMap<String, usize>) -> Result<Self, SqlxError> {
        Ok(IdModel { id: row.try_get(index_map["id"])? })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct IdListModel {
    pub ids: Vec<i64>,
}

impl IdListModel {
    pub fn new(ids: Vec<i64>) -> Self {
        Self { ids }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CountModel {
    pub count: Option<i64>,
}

impl CountModel {
    pub fn new(count: Option<i64>) -> Self {
        Self { count }
    }
}

impl FromSqlRow for CountModel {
    fn map_row_to_entity(row: MySqlRow, index_map: &HashMap<String, usize>) -> Result<Self, SqlxError> {
        Ok(CountModel { count: row.try_get(index_map["count"])? })
    }
}


