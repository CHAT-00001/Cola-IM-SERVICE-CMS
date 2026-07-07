// repo/src/three/pg/vendor_repo.rs  -- 仓储 - THREE - 厂商 PG
// 2026/6/30 05:01

////////

use chrono::Utc;
use cola_data::three::entity::vendor::{ThreeVendorEntity, THREE_VENDOR_COLUMNS};
use crate::pg_pool;

/////////

/// # [REPO] - 厂商 仓储
pub struct VendorRepo;

impl VendorRepo {

    /////////

    /// 1. #[REPOSITORY] - 插入或更新
    /// * upsert: INSERT ... ON CONFLICT (code) DO UPDATE
    /// * 新建: add_time, upd_time, created_at, updated_at 都是当前时间
    /// * 更新: 仅 upd_time, updated_at 更新
    pub async fn upsert(code: &str, name: &str, sort: i16, status: i16) -> Result<ThreeVendorEntity, sqlx::Error> {
        let pool = pg_pool();
        let now = Utc::now().timestamp();
        let query = format!(
            r#"INSERT INTO three_vendor (code, name, name_zh, sort, status, owner, add_time, upd_time)
               VALUES ($1, $2, $2, $3, $4, 0, $5, $5)
               ON CONFLICT (code) DO UPDATE SET name=$2, name_zh=$2, sort=$3, status=$4, upd_time=EXCLUDED.upd_time, updated_at=NOW()
               RETURNING {}"#,
            THREE_VENDOR_COLUMNS
        );
        sqlx::query_as::<_, ThreeVendorEntity>(&query)
            .bind(code).bind(name).bind(sort).bind(status).bind(now)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// 2. #[REPOSITORY] - 列表
    pub async fn list() -> Result<Vec<ThreeVendorEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM three_vendor ORDER BY sort ASC",
            THREE_VENDOR_COLUMNS
        );
        sqlx::query_as::<_, ThreeVendorEntity>(&query)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// 3. #[REPOSITORY] - 按 code 查询
    pub async fn find_by_code(code: &str) -> Result<Option<ThreeVendorEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM three_vendor WHERE code = $1 LIMIT 1",
            THREE_VENDOR_COLUMNS
        );
        sqlx::query_as::<_, ThreeVendorEntity>(&query)
            .bind(code)
            .fetch_optional(&pool)
            .await
    }

    /////////

    /// 4. #[REPOSITORY] - 按 ID 查询
    pub async fn find_by_id(id: i64) -> Result<Option<ThreeVendorEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM three_vendor WHERE id = $1 LIMIT 1",
            THREE_VENDOR_COLUMNS
        );
        sqlx::query_as::<_, ThreeVendorEntity>(&query)
            .bind(id)
            .fetch_optional(&pool)
            .await
    }
}

//////// END
