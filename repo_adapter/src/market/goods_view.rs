// repo_adapter/src/market/goods_view.rs  -- 适配器 - 商品浏览记录
// 2026/6/18

//////

use async_trait::async_trait;
use chrono::Utc;
use cola_data::market::info::address::AddressInfo;
use cola_data::market::port::goods_view::GoodsViewPort;
use repository::pg_pool;

//////

/// # [ADAPTER] - 商品浏览记录 端口适配器
pub struct GoodsViewAdapter;

#[async_trait]
impl GoodsViewPort for GoodsViewAdapter {

    async fn save_view_record(&self, uid: i64, goods_id: i64) -> anyhow::Result<()> {
        let pool = pg_pool();
        let now = Utc::now().timestamp() as i32;
        sqlx::query(
            "INSERT INTO shop_goods_visit (uid, goods_id, add_time, upd_time) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING"
        )
        .bind(uid).bind(goods_id).bind(now).bind(now)
        .execute(&pool).await?;
        Ok(())
    }

    async fn delete_view_record(&self, uid: i64, goods_id: i64) -> anyhow::Result<()> {
        let pool = pg_pool();
        sqlx::query("DELETE FROM shop_goods_visit WHERE uid = $1 AND goods_id = $2")
            .bind(uid).bind(goods_id)
            .execute(&pool).await?;
        Ok(())
    }

    async fn change_status(&self, _uid: i64, _goods_id: i64) -> anyhow::Result<()> {
        Ok(())
    }

    async fn delete_view(&self, uid: i64, goods_id: i64) -> anyhow::Result<()> {
        let pool = pg_pool();
        sqlx::query("DELETE FROM shop_goods_visit WHERE uid = $1 AND goods_id = $2")
            .bind(uid).bind(goods_id)
            .execute(&pool).await?;
        Ok(())
    }

    async fn get_view_ids_by_user_id(&self, uid: i64, offset: i64, limit: i64) -> anyhow::Result<Vec<i64>> {
        let pool = pg_pool();
        let rows: Vec<(i64,)> = sqlx::query_as(
            "SELECT goods_id FROM shop_goods_visit WHERE uid = $1 ORDER BY add_time DESC LIMIT $2 OFFSET $3"
        )
        .bind(uid).bind(limit).bind(offset)
        .fetch_all(&pool).await?;
        Ok(rows.into_iter().map(|r| r.0).collect())
    }

    async fn get_view_record_by_goods_id(&self, _uid: i64, _goods_id: i64) -> anyhow::Result<AddressInfo> {
        Ok(AddressInfo::not_found())
    }

    async fn delete_view_by_user_id(&self, _uid: i64, user_id: i64) -> anyhow::Result<()> {
        let pool = pg_pool();
        sqlx::query("DELETE FROM shop_goods_visit WHERE uid = $1")
            .bind(user_id)
            .execute(&pool).await?;
        Ok(())
    }
}
