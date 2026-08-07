// repo_adapter/src/user/black/del.rs
// 🔌 适配器 - USER - 黑名单 - 删除
// 2026/8/6 22:30 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::user::port::black::del::BlackDelPort;
use repository::user::service::black::BlacklistService;

////////

/// # [DEL ADAPTER] - 删除
/// * `desc`: `黑名单记录删除服务适配器`
pub struct BlackDelAdapter;

#[async_trait]
impl BlackDelPort for BlackDelAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 单个软删除
    async fn single_soft_del(
        &self,
        uid: i64,       // 操作者ID
        target_id: i64, // 目标用户ID
    ) -> Result<u16> {
        let _ = BlacklistService::save_black_record(uid, target_id, String::new(), 0).await?;
        Ok(1)
    }

    ////////

    /// # 2. [ADAPTER] - 批量软删除
    async fn batch_soft_del(
        &self,
        uid: i64,      // 操作者ID
        ids: Vec<i64>, // 目标用户ID列表
    ) -> Result<u16> {
        for id in &ids {
            let _ = BlacklistService::save_black_record(uid, *id, String::new(), 0).await;
        }
        Ok(ids.len() as u16)
    }
}

//////// END
