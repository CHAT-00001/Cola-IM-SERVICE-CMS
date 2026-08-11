// repo_adapter/src/cola_user/black/get.rs
// 🔌 适配器 - USER - 黑名单 - 获取
// 2026/8/6 解耦: 获取黑名单IDs

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_user::black::get::UserBlackGetPort;
use repository::cola_user::pg::black::get::UserBlackGetRepo;
use tracing::{error, info};

////////

/// # [GET ADAPTER] - 获取
/// * `desc`: `用户黑名单获取服务适配器`
pub struct BlackGetAdapter;

#[async_trait]
impl UserBlackGetPort for BlackGetAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 用户拉黑的
    /// * ``: `根据用户ID` - `获取主动拉黑的用户IDs列表`
    async fn get_black_ids(
        &self,
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<i64>> {
        // 目标ID转换
        let target_uid = user_id;

        // 1. Call The Repo ..
        let result = UserBlackGetRepo::find_black_ids_by_uid(target_uid, offset, limit).await;

        match result {
            Ok(ids) => {
                info!(
                    "[🔌 ADAPTER] - ✅️ 获取我的黑名单IDs成功: uid = {}, count = {}",
                    target_uid,
                    ids.len()
                );
                Ok(ids)
            }
            Err(e) => {
                error!(
                    "[🔌 ADAPTER] - ❌️ 获取我的黑名单IDs失败: uid = {}, err = {:?}",
                    target_uid, &e
                );
                // 💡 修正：将 sqlx::Error 转换为 anyhow::Error
                Err(e.into())
            }
        }
    }

    ////////

    /// # 2. [ADAPTER] - 获取黑名单IDs列表
    /// * `desc`: `根据用户ID` - `获取拉黑用户的用户IDs列表`
    async fn get_black_me_ids(&self, user_id: i64, limit: i64, offset: i64) -> Result<(Vec<i64>)> {
        let target_uid = user_id;

        let result = UserBlackGetRepo::find_black_ids_by_uid(target_uid, offset, limit).await;

        match result {
            Ok(ids) => {
                info!(
                    "[🔌 ADAPTER] - ✅️ 获取拉黑我的黑名单IDs成功: uid = {}, count = {}",
                    target_uid,
                    ids.len()
                );
                Ok(ids)
            }
            Err(e) => {
                error!(
                    "[🔌 ADAPTER] - ❌️ 获取拉黑我的黑名单IDs失败: uid = {}, err = {:?}",
                    target_uid, &e
                );
                // 💡 修正：将 sqlx::Error 转换为 anyhow::Error
                Err(e.into())
            }
        }
    }
}

//////// END
