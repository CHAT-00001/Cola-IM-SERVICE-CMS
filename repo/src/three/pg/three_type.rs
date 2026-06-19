// repo/src/three/pg/three_type.rs  -- 仓储 - THREE - 类型 PG
// 2026/6/18

//////

use cola_data::three::entity::three_type::{ThreeTypeEntity, THREE_TYPE_COLUMNS};
use crate::pg_pool;

//////

/// # [REPO] - 服务类型 仓储
pub struct TypeRepo;

impl TypeRepo {

    /// 1. 插入或更新
    pub async fn upsert(code: &str, name: &str, sort: i16, status: i16) -> Result<ThreeTypeEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            r#"INSERT INTO three_type (code, name, sort, status)
               VALUES ($1, $2, $3, $4)
               ON CONFLICT (code) DO UPDATE SET name=$2, sort=$3, status=$4
               RETURNING {}"#,
            THREE_TYPE_COLUMNS
        );
        sqlx::query_as::<_, ThreeTypeEntity>(&query)
            .bind(code).bind(name).bind(sort).bind(status)
            .fetch_one(&pool)
            .await
    }

    /// 2. 列表
    pub async fn list() -> Result<Vec<ThreeTypeEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM three_type ORDER BY sort ASC",
            THREE_TYPE_COLUMNS
        );
        sqlx::query_as::<_, ThreeTypeEntity>(&query)
            .fetch_all(&pool)
            .await
    }

    /// 3. 按 code 查询
    pub async fn find_by_code(code: &str) -> Result<Option<ThreeTypeEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM three_type WHERE code = $1 LIMIT 1",
            THREE_TYPE_COLUMNS
        );
        sqlx::query_as::<_, ThreeTypeEntity>(&query)
            .bind(code)
            .fetch_optional(&pool)
            .await
    }
}
