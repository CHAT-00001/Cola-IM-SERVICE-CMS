// repository/src/cola_market/pg/express.rs
// 仓储 - 市场 - pg - 快递
// 2026/6/18

////////

use crate::pg_pool;
use cola_data::cola_market::entity::express::express::ExpressEntity;

////////

/// # [EXPRESS REPO] - 快递公司 仓储
pub struct ExpressRepo;

impl ExpressRepo {
    const COLUMNS: &'static str = r#"
        id, express_name, name_en, express_phone, express_thumb,
        express_status, express_code, sort, add_time, upd_time, create_at, update_at, list_order
    "#;

    /// 1. 查询所有启用的快递
    pub async fn find_enabled() -> Result<Vec<ExpressEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM shop_express WHERE express_status = 1 ORDER BY sort ASC",
            Self::COLUMNS
        );
        sqlx::query_as::<_, ExpressEntity>(&query)
            .fetch_all(&pool)
            .await
    }

    /// 2. 按ID查询
    pub async fn find_by_id(id: i64) -> Result<Option<ExpressEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM shop_express WHERE id = $1 LIMIT 1",
            Self::COLUMNS
        );
        sqlx::query_as::<_, ExpressEntity>(&query)
            .bind(id)
            .fetch_optional(&pool)
            .await
    }
}

//////// END