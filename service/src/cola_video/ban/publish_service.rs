// servicey/src/cola_video/ban/publish_service.rs
// 👤 服务 - 可乐视频 - 封禁 - 发布服务
// 2026/8/1 16:53

////////

use anyhow::Result;
use chrono::{DateTime, Utc};
use cola_data::cola_video::entity::banned::publish::VideoBannedPublishEntity;
use tracing::error;
use repository::cola_video::pg::ban::publish_repo::VideoBannedPublishRepo;
////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `短视频发布封禁服务`
pub struct VideoPublishBanService;

impl VideoPublishBanService {
    //

    ////////

    /// # 1. [SERVICE] - 添加新的发布封禁记录
    pub async fn add_banned(
        operator_uid: i64,
        uid: i64,
        begin_at: Option<DateTime<Utc>>,
        end_at: Option<DateTime<Utc>>,
        reason: Option<String>,
    ) -> Result<()> {
        VideoBannedPublishRepo::save_banned_by_user_id(operator_uid, uid, begin_at, end_at, reason)
            .await
            .map_err(|err| {
                error!(uid = uid, error = ?err, "add_banned failed");
                err.into()
            })
    }

    ////////

    /// # 2. [SERVICE] - 更新发布封禁记录
    pub async fn update_banned(
        operator_uid: i64,
        uid: i64,
        begin_at: Option<DateTime<Utc>>,
        end_at: Option<DateTime<Utc>>,
        reason: Option<String>,
    ) -> Result<()> {
        VideoBannedPublishRepo::update_banned_by_user_id(
            operator_uid,
            uid,
            begin_at,
            end_at,
            reason,
        )
        .await
        .map_err(|err| {
            error!(uid = uid, error = ?err, "update_banned failed");
            err.into()
        })
    }

    ////////

    /// # 3. [SERVICE] - 删除（软删除）发布封禁记录
    pub async fn delete_banned(uid: i64) -> Result<()> {
        VideoBannedPublishRepo::soft_delete_banned_by_user_id(uid)
            .await
            .map_err(|err| {
                error!(uid = uid, error = ?err, "delete_banned failed");
                err.into()
            })
    }

    ////////

    /// # 4. [SERVICE] - 检查用户发布封禁状态
    /// * `desc`: 传入目标 uid，如果找不到相关记录（或已删除），返回 true（表示未被封禁/正常），否则返回 false
    pub async fn check_banned(uid: i64) -> Result<bool> {
        match VideoBannedPublishRepo::find_banned_by_user_id(uid).await {
            Ok(Some(_)) => Ok(false), // 找到了封禁记录，说明已被封禁
            Ok(None) => Ok(true),     // 没找到记录，说明正常
            Err(err) => {
                error!(uid = uid, error = ?err, "check_banned failed");
                Err(err.into())
            }
        }
    }
}

//////// END
