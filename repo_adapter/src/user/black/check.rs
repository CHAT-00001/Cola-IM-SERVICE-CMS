// repo_adapter/src/user/black/check.rs
// 🔌 适配器 - USER - 黑名单 - 状态检查
// 2026/8/6 解耦: 检查是否在黑名单

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::user::port::black::check::BlackCheckPort;
use repository::user::pg::black::state::UserBlackStateRepo;
use tracing::{error, info};

////////

/// # [CHECK ADAPTER] - 检查
/// * `desc`: `用户检查是否在黑名单`
pub struct BlackCheckAdapter;

// 构造实现
#[async_trait]
impl BlackCheckPort for BlackCheckAdapter {
    //

    ////////

    /// # [ADAPTER] - 检查黑名单状态
    async fn is_blacked(
        &self,
        uid: i64, // UID
        id: i64,  // 目标用户ID
    ) -> anyhow::Result<bool> {
        info!(
            "[🔌 ADAPTER] - 🔍 开始检查黑名单状态: uid = {}, target_id = {}",
            uid, id
        );

        // 🚧 1. 缓存检查 (Cache Check)
        // 提示：如果项目中引入了 Redis 缓存，可在此处优先查询缓存
        // let cache_key = format!("user:black:{}:{}", uid, id);
        // if let Some(cached_status) = CacheClient::get(&cache_key).await? {
        //     info!("[🔌 ADAPTER] - ⚡ 缓存命中黑名单状态: uid = {}, target_id = {}, result = {}", uid, id, cached_status);
        //     return Ok(cached_status);
        // }

        // 🚧 2. PG 检查 (Database Check)
        let is_blacked = UserBlackStateRepo::find_black_state_by_uid_and_user_id(uid, id)
            .await
            .map_err(|e| {
                error!(
                    "[🔌 ADAPTER] - ❌️ PG 查询黑名单状态失败: uid = {}, target_id = {}, err = {:?}",
                    uid, id, e
                );
                e
            })?;

        info!(
            "[🔌 ADAPTER] - 🗄️ PG 查询黑名单状态成功: uid = {}, target_id = {}, result = {}",
            uid, id, is_blacked
        );

        // 🚧 3. 回填缓存 (Cache Backfill)
        // 提示：将 PG 查询到的最新结果异步或同步写入缓存
        // let _ = CacheClient::set(&cache_key, is_blacked, Some(Duration::from_secs(3600))).await;

        Ok(is_blacked)
    }
}

//////// END
