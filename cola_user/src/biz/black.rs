// // store/src/router/black/case.rs
//
// use crate::router::adapter::black::BlacklistRepo; // 确保路径对应你最新的 repo
// use api::app_state::AppState;
// use anyhow::Result;
// use redis::AsyncCommands;
// use std::collections::HashSet;
//
// pub struct BlacklistService {
//     // 既然 Repo 是静态调用且需要 AppState，Service 必须持有它
//     app_state: AppState,
// }
//
// impl BlacklistService {
//     pub fn new(app_state: AppState) -> Self {
//         Self { app_state }
//     }
//
//     const CACHE_KEY_PREFIX: &'static str = "router:blacklist:";
//     const CACHE_TTL: u64 = 360000;
//
//     /// 获取用于过滤的黑名单 (双向)
//     pub async fn get_ids_for_filter(&self, uid: i64) -> Result<Vec<i64>> {
//         let cache_key = format!("{}filter:{}", Self::CACHE_KEY_PREFIX, uid);
//
//         // 1. 尝试从缓存获取 (利用 AppState 里的 redis 逻辑)
//         if let Ok(Some(cached_ids)) = self.app_state.db.get_redis_json::<Vec<i64>>(&cache_key).await {
//             return Ok(cached_ids);
//         }
//
//         // 2. 静态调用 Repo，并传入 service 持有的 app_state
//         // 这样你的业务代码外部就不用传 app_state 了
//         let my_list = BlacklistRepo::repo_get_blacklist_ids_by_uid(&self.app_state, uid).await?;
//         let blocked_me = BlacklistRepo::repo_get_blacklist_ids_by_to_uid(&self.app_state, uid).await?;
//
//         // 3. 合并去重
//         let mut set: HashSet<i64> = my_list.into_iter().collect();
//         set.extend(blocked_me);
//         let ids: Vec<i64> = set.into_iter().collect();
//
//         // 4. 异步写缓存
//         let app_state_clone = self.app_state.clone();
//         let cache_key_clone = cache_key.clone();
//         let ids_clone = ids.clone();
//         tokio::spawn(async move {
//             let _ = app_state_clone.db.set_redis_json(&cache_key_clone, &ids_clone, Self::CACHE_TTL).await;
//         });
//
//         Ok(ids)
//     }
//
//     /// 添加黑名单并清理缓存
//     pub async fn add_blacklist(&self, uid: i64, to_uid: i64) -> Result<()> {
//         // 调用静态 Repo
//         BlacklistRepo::repo_blacklist_upsert(&self.app_state, uid, to_uid).await?;
//         self.clear_cache(uid).await?;
//         Ok(())
//     }
//
//     /// 移除黑名单并清理缓存
//     pub async fn remove_blacklist(&self, uid: i64, to_uid: i64) -> Result<()> {
//         // 调用静态 Repo
//         BlacklistRepo::repo_blacklist_deactivate(&self.app_state, uid, to_uid).await?;
//         self.clear_cache(uid).await?;
//         Ok(())
//     }
//
//     /// 内部清理缓存逻辑
//     async fn clear_cache(&self, uid: i64) -> Result<()> {
//         let manage_key = format!("{}manage:{}", Self::CACHE_KEY_PREFIX, uid);
//         let filter_key = format!("{}filter:{}", Self::CACHE_KEY_PREFIX, uid);
//
//         // 直接从 AppState 获取 redis 连接
//         let mut conn = self.app_state.db.redis_conn.clone();
//         let _: () = conn.del(&[manage_key, filter_key]).await?;
//         Ok(())
//     }
// }