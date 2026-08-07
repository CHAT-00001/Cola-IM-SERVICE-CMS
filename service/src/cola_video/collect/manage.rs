// service/src/cola_video/collect/manage.rs
// 👤 服务 - ▶ 可乐视频 - 收藏 - 管理
// 2026/8/2 16:49 Created.

////////

use repository::cola_video::pg::collect::manage::VideoCollectManageRepo;
use repository::cola_video::pg::video::count::VideoCountRepo;

////////

/// # [MANAGE SERVICE] - 管理
/// * `desc`: `▶ 可乐视频 - 👤 收藏管理服务`
pub struct VideoCollectManageService;

impl VideoCollectManageService {
    //

    ////////

    /// # 1. [SERVICE] - 删除单条收藏记录 + 同步视频收藏数 - 1
    /// * `uid`: 用户ID (或操作者ID/管理员ID)
    /// * `video_id`: 视频ID
    pub async fn soft_delete_single(uid: i64, video_id: i64) -> Result<(), anyhow::Error> {
        // 1. 调用软删除单条记录 (返回受影响行数)
        let rows_affected =
            VideoCollectManageRepo::soft_delete_collect_by_video_id(uid, video_id).await?;

        // 2. 如果确实删除了记录，才同步更新视频的收藏计数 - 1
        if rows_affected > 0 {
            let increment = -1;
            VideoCountRepo::pg_update_video_collects(video_id, increment).await?;
        }

        Ok(())
    }

    ////////

    /// # 2. [SERVICE] - 批量删除收藏记录 + 同步对应视频收藏数
    /// * `uid`: 用户ID (或操作者ID/管理员ID)
    /// * `video_ids`: 视频ID列表
    pub async fn batch_soft_delete(uid: i64, video_ids: &[i64]) -> Result<(), anyhow::Error> {
        if video_ids.is_empty() {
            return Ok(());
        }

        for &video_id in video_ids {
            let rows_affected =
                VideoCollectManageRepo::soft_delete_collect_by_video_id(uid, video_id).await?;
            if rows_affected > 0 {
                let increment = -(rows_affected as i64);
                VideoCountRepo::pg_update_video_collects(video_id, increment as i16).await?;
            }
        }

        Ok(())
    }
}

//////// END
