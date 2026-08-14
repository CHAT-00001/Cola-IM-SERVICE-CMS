// repository/src/market/pg/goods/check.rs
// 仓储 - MARKET - pg - 商品 - 状态检查
// 2026/8/11 07:38 Created.

////////

use crate::pg_pool;
use cola_data::market::entity::goods::goods::{GOODS_COLUMNS, GoodsEntity};

////////

/// # [CHECK REPOSITORY] - 检查
/// * `desc`: `检查商品的状态`
pub struct GoodsChangeRepo;

impl GoodsChangeRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 健康
    pub async fn check_health(id: i64, // 商品 ID
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let res = sqlx::query_scalar::<_, i64>(
            "SELECT health FROM cola_market.goods WHERE id = $1",
        )
            .bind(id)
            .fetch_optional(&pool)
            .await;

        match res {
            Ok(Some(val)) => Ok(val as u64),
            Ok(None) => Ok(0),
            Err(e) => {
                tracing::error!(target: "market::goods", "check_health error for id {}: {:?}", id, e);
                Err(e)
            }
        }
    }

    ////////

    /// # 2. [REPOSITORY] - 状态
    pub async fn check_status(id: i64, // 商品 ID
    ) -> Result<GoodsEntity, sqlx::Error> {
        let pool = pg_pool();
        let sql = format!("SELECT {} FROM cola_market.goods WHERE id = $1", GOODS_COLUMNS);
        let res = sqlx::query_as::<_, GoodsEntity>(&sql)
            .bind(id)
            .fetch_optional(&pool)
            .await;

        match res {
            Ok(Some(goods)) => Ok(goods),
            Ok(None) => {
                let err = sqlx::Error::RowNotFound;
                tracing::error!(target: "market::goods", "check_status not found for id {}", id);
                Err(err)
            }
            Err(e) => {
                tracing::error!(target: "market::goods", "check_status error for id {}: {:?}", id, e);
                Err(e)
            }
        }
    }

    ////////

    /// # 3. [REPOSITORY] - 归属
    pub async fn check_owner(
        user_id: i64, // 用户 ID
        id: i64,      // 商品 ID
    ) -> Result<bool, sqlx::Error> {
        let pool = pg_pool();
        let res = sqlx::query_scalar::<_, i64>(
            "SELECT 1 FROM cola_market.goods WHERE id = $1 AND uid = $2",
        )
            .bind(id)
            .bind(user_id)
            .fetch_optional(&pool)
            .await;

        match res {
            Ok(row) => Ok(row.is_some()),
            Err(e) => {
                tracing::error!(target: "market::goods", "check_owner error for id {}, user_id {}: {:?}", id, user_id, e);
                Err(e)
            }
        }
    }
}

//////// END