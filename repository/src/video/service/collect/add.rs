// repository/src/video/service/collect.rs
// 仓储 - VIDEO - service - 收藏
// 2026/8/2 17:03 Created.

////////

use crate::video::pg::collect::get::CollectGetRepo;
use crate::video::pg::collect::manage::CollectManageRepo;
use crate::video::pg::comment::comment::CommentRepo;
use crate::video::pg::user::user_repo::UserRepo;
use crate::video::pg::video::count::VideoCountRepo;
use crate::video::pg::video::video::VideoRepo;
use anyhow::Error;
use cola_data::video::command::collect::CollectCommand;
use cola_data::video::command::recommend::RecommendCommand;
use cola_data::video::entity::collect::VideoCollectEntity;
use cola_data::video::entity::video::video::VideoEntity;
use tracing::log;

////////

/// # [ADD SERVICE] - 视频 收藏 添加 服务
pub struct VideoCollectAddService;

// 构造实现
impl VideoCollectAddService {
    //

    ////////

    /// # 1. [SERVICE] - 保存收藏 + 更新计数
    pub async fn save_collect_and_update_count(
        uid: i64,
        _video_id: i64,
        _cmd: CollectCommand,
    ) -> Result<VideoCollectEntity, anyhow::Error> {
        let collect_entity = VideoCollectEntity::default();

        // 联动更新计数器：收藏的视频数量 + 1
        let async_uid = uid;
        tokio::spawn(async move {
            // 收藏字段在第四位：publish, liked, total_favorited, collected
            if let Err(e) = UserRepo::update_user_count(async_uid, 0, 0, 0, 1, 0, 0).await {
                log::error!(
                    "SERVICE_ASYNC: 异步更新用户收藏计数失败: uid={}, err={:?}",
                    async_uid,
                    e
                );
            }
        });

        Ok(collect_entity)
    }

    ////////

    /// # 2. [SERVICE] - 根据用户ID查找收藏记录IDs
    /// * `user_id`  用户 ID
    /// * `offset`
    /// * `limit`
    pub async fn find_collect_ids_by_user_id(
        user_id: i64,
        keyword: Option<String>,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<i64>, anyhow::Error> {
        // 直接调用 Repo 获取 Vec<i64>，并使用 ? 传播错误
        let collect_ids =
            CollectGetRepo::find_collect_ids_by_user_id(user_id, keyword, offset, limit).await?;

        // 直接返回
        Ok(collect_ids)
    }

    ////////

    /// # 4. [SERVICE] - 删除收藏 + 更新计数
    pub async fn del_collect_and_update_count(
        uid: i64,
        video_id: i64,
    ) -> Result<(), anyhow::Error> {
        // 1. 删除记录
        // 如果不需要返回具体的 handler，这里直接执行删除操作即可
        CollectManageRepo::soft_delete_collect_by_video_id(uid, video_id).await?;

        // 2. 更新视频计数
        // 删除收藏对应计数器操作：通常为 -1 (increment = -1)
        let increment = -1;
        VideoCountRepo::pg_update_video_collects(video_id, increment).await?;

        Ok(())
    }
}

//////// END
