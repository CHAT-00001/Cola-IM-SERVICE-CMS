// repository/src/market/pg/goods/stat.rs
// 仓储 - MARKET - pg - 商品 - 获取
// 2026/8/11 07:38 Created.

////////

use crate::pg_pool;

////////

/// # [STAT REPOSITORY] - 统计
/// * `desc`: `统计商品数量`
pub struct GoodsStatRepo;

impl GoodsStatRepo {

    /// # 1. [REPOSITORY] - 统计所有商品总数
    pub async fn count_all_goods() -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM market.goods"
        )
            .fetch_one(&pool)
            .await?;

        Ok(row.0 as u64)
    }

    ////////

    /// # 2. [REPOSITORY] - 统计已下架的商品总数（假设 issale = 0 代表下架）
    pub async fn count_offline_goods() -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM market.goods WHERE issale = 0"
        )
            .fetch_one(&pool)
            .await?;

        Ok(row.0 as u64)
    }

    ////////

    /// # 3. [REPOSITORY] - 统计指定用户的商品总数
    pub async fn count_goods_by_uid(uid: i64) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM market.goods WHERE uid = $1"
        )
            .bind(uid)
            .fetch_one(&pool)
            .await?;

        Ok(row.0 as u64)
    }

}

//////// END