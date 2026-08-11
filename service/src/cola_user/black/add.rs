// service/src/cola_user/black/add.rs
// 服务 - 可乐用户 - 黑名单 - 发布
// 2026/8/3 14:28 Created.

////////

use anyhow::{Result, anyhow};
use cola_data::cola_video::entity::collect::{VIDEO_COLLECT_COLUMNS, VideoCollectEntity};
use repository::cola_user::pg::black::add::UserBlackAddRepo;
use repository::cola_user::pg::black::get::UserBlackGetRepo;

////////

/// # [BLACK ADD SERVICE] - 发布
/// * `desc`: `🗣 可乐用户 - 👤 用户黑名单发布服务`
pub struct BlackGetService;

impl BlackGetService {
    //

    ////////

    /// # 1. [SERVICE] - 保存/取消拉黑记录
    /// * `uid` 当前用户
    /// * `user_id` 目标用户
    /// * `status` 1=拉黑, 0=取消
    pub async fn save_black_record(
        uid: i64,
        user_id: i64,
        remark: String,
        status: i16,
    ) -> Result<u64> {
        let rows = UserBlackAddRepo::save_add_black(uid, user_id, remark, status)
            .await
            .map_err(|e| anyhow!("[🤐 BLACK SERVICE]: ❌️ 保存黑名单记录失败: {}", e))?;

        tracing::info!(
            "[🗣️ BLACK SERVICE]: ✅️ 拉黑操作成功, uid={}, target={}, status={}",
            uid,
            user_id,
            status
        );
        Ok(rows)
    }

    ////////

    /// # 3. [SERVICE] - 获取被拉黑列表(谁拉黑了我)
    pub async fn get_blacker_ids(user_id: i64, offset: i64, limit: i64) -> Result<Vec<i64>> {
        let ids = UserBlackGetRepo::find_blacker_ids_by_user_id(user_id, limit, offset)
            .await
            .map_err(|e| anyhow!("[🤐 BLACK SERVICE]: ❌️ 查询被拉黑列表失败: {}", e))?;

        tracing::info!(
            "[🗣️ BLACK SERVICE]: ✅️ 被拉黑查询成功, user_id={}, count={}",
            user_id,
            ids.len()
        );
        Ok(ids)
    }
}

//////// END
