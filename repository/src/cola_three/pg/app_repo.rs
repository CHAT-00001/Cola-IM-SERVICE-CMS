// repository/src/cola_three/pg/app_repo.rs  -- 仓储 - THREE - 应用管理 PG
// 2026/8/15 13:10 Created.

////////

use chrono::Utc;
use cola_data::cola_three::entity::app::{ColaAppEntity, COLA_APP_COLUMNS};
use crate::pg_pool;

////////

/// # [REPO] - 应用管理 仓储
pub struct AppRepo;

impl AppRepo {
    ////////

    /// 1. #[REPOSITORY] - 按 app_id 查询应用
    pub async fn find_by_app_id(app_id: &str) -> Result<Option<ColaAppEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_three.cola_app WHERE app_id = $1",
            COLA_APP_COLUMNS
        );
        sqlx::query_as::<_, ColaAppEntity>(&query)
            .bind(app_id)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// 2. #[REPOSITORY] - 创建应用
    pub async fn insert(
        app_id: &str,
        name: &str,
        description: Option<&str>,
        status: i16,
    ) -> Result<ColaAppEntity, sqlx::Error> {
        let pool = pg_pool();
        let now = Utc::now();
        let query = format!(
            r#"INSERT INTO cola_three.cola_app (app_id, name, description, status, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $5)
               RETURNING {}"#,
            COLA_APP_COLUMNS
        );
        sqlx::query_as::<_, ColaAppEntity>(&query)
            .bind(app_id)
            .bind(name)
            .bind(description)
            .bind(status)
            .bind(now)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// 3. #[REPOSITORY] - 列表查询所有应用
    pub async fn list() -> Result<Vec<ColaAppEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_three.cola_app ORDER BY id ASC",
            COLA_APP_COLUMNS
        );
        sqlx::query_as::<_, ColaAppEntity>(&query)
            .fetch_all(&pool)
            .await
    }
}

//////// END