// repo/src/three/pg/three_vendor.rs  -- 仓储 - THREE - 厂商 PG
// 2026/6/18

//////

use cola_data::three::entity::three_vendor::{ThreeVendorEntity, THREE_VENDOR_COLUMNS};
use crate::pg_pool;

//////

/// # [REPO] - 厂商 仓储
pub struct VendorRepo;

impl VendorRepo {

    /// 1. 插入或更新
    pub async fn upsert(code: &str, name: &str, sort: i16, status: i16) -> Result<ThreeVendorEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            r#"INSERT INTO three_vendor (code, name, sort, status)
               VALUES ($1, $2, $3, $4)
               ON CONFLICT (code) DO UPDATE SET name=$2, sort=$3, status=$4
               RETURNING {}"#,
            THREE_VENDOR_COLUMNS
        );
        sqlx::query_as::<_, ThreeVendorEntity>(&query)
            .bind(code).bind(name).bind(sort).bind(status)
            .fetch_one(&pool)
            .await
    }

    /// 2. 列表
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

    /// 3. 按 code 查询
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
}
