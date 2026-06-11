// store src/gift/info/music  -- 礼物 DTO
// 2026-01-07 14:20:02

use serde::{Deserialize, Serialize};


/// ## 礼物数据模型
#[derive(Serialize, Deserialize, Debug)]
pub struct GiftModel {
    pub id: i64,                       // id
    pub mark: Option<i32>,             // 标识
    pub r#type: Option<i32>,           // 作者id
    pub sid: Option<String>,           // 标题
    pub giftname: String,              // 礼物名称
    pub giftname_en: String,           // 礼物名称 - 英文
    pub description: Option<String>,   // 描述
    pub need_coin: Option<i32>,         // 价格（旧版）
    pub gift_icon: Option<String>,          // 图标
    pub list_order: Option<String>,        // 排序（旧版）
    pub add_time: Option<i32>,           // 创建时间戳
    pub swf_type: i32,                    // 礼物类型
    pub swf: Option<String>,         // 礼物 url
    pub swf_time: Option<f64>,           // 礼物动画时间
    pub is_plat_gift: i32,                   // 状态：0. 审核中  1. 通过
    pub sticker_id: Option<i32>,         // 媒体
}



// use crate::repository::gift_repository as gift_repo;
// use crate::models::gift_model::GiftVo;
// use api::db_service::DbService; // 确保导入了 DbService
// use sqlx::PgPool;
// use tracing::{info, warn};
//
// #[tracing::instrument(skip(db))]
// pub async fn get_gifts_service(
//     db: &DbService, // 直接传入我们的 DbService
// ) -> Result<Vec<GiftVo>, anyhow::Error> {
//     let cache_key = "live:gift:list:v1";
//
//     // 1️⃣ 尝试从 Redis 缓存获取 (使用封装好的异步方法)
//     // 泛型会根据 GiftVo 自动处理反序列化
//     if let Ok(Some(cached_gifts)) = db.get_redis_json::<Vec<GiftVo>>(cache_key).await {
//         return Ok(cached_gifts);
//     }
//
//     warn!("Cache miss or error: {}", cache_key);
//
//     // 2️⃣ 缓存未命中，查询 PostgreSQL 数据库
//     // 注意：pool 现在直接从 db.pg_pool 获取
//     let entities = gift_repo::repo_get_active_gifts(&db.pg_pool, 100, 0).await?;
//
//     let gifts: Vec<GiftVo> = entities.into_iter().map(GiftVo::from).collect();
//
//     // 3️⃣ 异步写回缓存
//     if !gifts.is_empty() {
//         // 设置随机 TTL 防止缓存雪崩
//         let ttl = 3600 + rand::random::<u64>() % 300;
//
//         // 直接调用异步写入，无需 spawn_blocking
//         let _ = db.set_redis_json(cache_key, &gifts, ttl).await;
//         info!("Cache refilled for: {}", cache_key);
//     }
//
//     Ok(gifts)
// }
