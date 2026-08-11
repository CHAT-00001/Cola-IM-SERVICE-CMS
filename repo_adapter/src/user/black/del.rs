// repo_adapter/src/user/black/del.rs
// 🔌 适配器 - USER - 黑名单 - 删除
// 2026/8/6 22:30 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_user::black::del::UserBlackDelPort;
use repository::cola_user::pg::black::get::UserBlackGetRepo;
use tracing::info;

////////

/// # [DEL ADAPTER] - 删除
/// * `desc`: `黑名单记录删除服务适配器`
pub struct BlackDelAdapter;

#[async_trait]
impl UserBlackDelPort for BlackDelAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 单个软删除
    /// * `desc`: 调用 save_black_record 将 status 置为 0 来实现
    async fn single_soft_del(
        &self,
        uid: i64,       // 操作者ID
        target_id: i64, // 目标用户ID
    ) -> Result<u16> {
        let rows_affected =
            UserBlackGetRepo::save_black_record(uid, target_id, "".to_string(), 0).await?;
        info!(
            "[🔌 ADAPTER] - ✅️ 取消拉黑成功: uid={}, target_id={}, affected={}",
            uid, target_id, rows_affected
        );
        Ok(rows_affected as u16)
    }

    ////////

    /// # 2. [ADAPTER] - 批量软删除
    async fn batch_soft_del(
        &self,
        uid: i64,      // 操作者ID
        ids: Vec<i64>, // 目标用户ID列表
    ) -> Result<u16> {
        let mut total_affected = 0;
        for target_id in ids.iter() {
            let rows_affected =
                UserBlackGetRepo::save_black_record(uid, *target_id, "".to_string(), 0).await?;
            total_affected += rows_affected;
        }
        info!(
            "[🔌 ADAPTER] - ✅️ 批量取消拉黑成功: uid={}, count={}, total_affected={}",
            uid,
            ids.len(),
            total_affected
        );
        Ok(total_affected as u16)
    }
}

//////// END
