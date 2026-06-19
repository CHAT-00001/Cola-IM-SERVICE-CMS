// repo/src/three/pg/three_biz_binding.rs  -- 仓储 - THREE - 绑定 PG
// 2026/6/18

//////

use cola_data::three::entity::three_biz_binding::{ThreeBizBindingEntity, THREE_BIZ_BINDING_COLUMNS};
use crate::pg_pool;

//////

/// # [REPO] - 业务绑定 仓储
pub struct BindingRepo;

impl BindingRepo {

    /// 1. 插入或更新（UNIQUE 冲突则更新）
    pub async fn upsert(
        three_config_id: i64, biz_module: &str, biz_type: &str, status: i16,
    ) -> Result<ThreeBizBindingEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            r#"INSERT INTO three_biz_binding (three_config_id, biz_module, biz_type, status)
               VALUES ($1, $2, $3, $4)
               ON CONFLICT (biz_module, biz_type) DO UPDATE SET three_config_id=$1, status=$4, updated_at=NOW()
               RETURNING {}"#,
            THREE_BIZ_BINDING_COLUMNS
        );
        sqlx::query_as::<_, ThreeBizBindingEntity>(&query)
            .bind(three_config_id).bind(biz_module).bind(biz_type).bind(status)
            .fetch_one(&pool)
            .await
    }

    /// 2. 列表
    pub async fn list() -> Result<Vec<ThreeBizBindingEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM three_biz_binding ORDER BY biz_module, biz_type",
            THREE_BIZ_BINDING_COLUMNS
        );
        sqlx::query_as::<_, ThreeBizBindingEntity>(&query)
            .fetch_all(&pool)
            .await
    }

    /// 3. 按业务模块+类型查询
    pub async fn find_by_biz(biz_module: &str, biz_type: &str) -> Result<Option<ThreeBizBindingEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM three_biz_binding WHERE biz_module = $1 AND biz_type = $2 LIMIT 1",
            THREE_BIZ_BINDING_COLUMNS
        );
        sqlx::query_as::<_, ThreeBizBindingEntity>(&query)
            .bind(biz_module).bind(biz_type)
            .fetch_optional(&pool)
            .await
    }
}
