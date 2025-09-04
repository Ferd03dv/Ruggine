use sqlx::SqlitePool;
use crate::models::Group;
use crate::repositories::group_repository;

pub struct GroupService;

impl GroupService {
    pub async fn create_group(pool: &SqlitePool, name: &str, created_by: i64) -> Result<(), sqlx::Error> {
        group_repository::create_group(pool, name, created_by).await
    }

    pub async fn list_groups_by_user(pool: &SqlitePool, user_id: i64) -> Result<Vec<Group>, sqlx::Error> {
        group_repository::list_groups_by_user(pool, user_id).await
    }
}