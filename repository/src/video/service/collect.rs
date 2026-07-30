// service/collect.rs  -- -- 仓储中心 - VIDEO - 服务 - 评论
// 2026/6/10 07:06

////////

use crate::video::pg::collect::CollectRepo;
use crate::video::pg::comment::CommentRepo;
use crate::video::pg::count::CountRepo;
use crate::video::pg::user::UserRepo;
use crate::video::pg::video::{VideoRepo};
use anyhow::Error;
use cola_data::video::command::collect::CollectCommand;
use cola_data::video::command::recommend::RecommendCommand;
use cola_data::video::entity::collect::CollectEntity;
use cola_data::video::entity::video::VideoEntity;
use tracing::log;

////////

/// # [SERVICE] - 视频收藏服务
pub struct CollectService;

impl CollectService {
    ////////

    ////////

    /// # 1. [SERVICE] - 保存收藏 + 更新计数
    pub async fn save_collect_and_update_count(
        uid: i64,
        _video_id: i64,
        _cmd: CollectCommand,
    ) -> Result<CollectEntity, anyhow::Error> {
        let collect_entity = CollectEntity::default();

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
            CollectRepo::find_collect_ids_by_user_id(user_id, keyword, offset, limit).await?;

        // 直接返回
        Ok(collect_ids)
    }

    ////////

    ////////

    /// # 4. [SERVICE] - 删除收藏 + 更新计数
    pub async fn del_collect_and_update_count(
        uid: i64,
        video_id: i64,
    ) -> Result<(), anyhow::Error> {
        // 1. 删除记录
        // 如果不需要返回具体的 handler，这里直接执行删除操作即可
        CollectRepo::delete_collect_by_video_id(uid, video_id).await?;

        // 2. 更新视频计数
        // 删除收藏对应计数器操作：通常为 -1 (increment = -1)
        let increment = -1;
        CountRepo::pg_update_video_collects(video_id, increment).await?;

        Ok(())
    }

    ////////


    ////////

    ////////

    /// # 7. [SERVICE] - 推荐
    pub async fn save_recommend_and_update_count(
        _uid: i64,
        _cmd: RecommendCommand,
    ) -> Result<(), anyhow::Error> {
        // TODO: 创作者通过特殊权益将视频送上推荐流记录

        Ok(())
    }

    ////////

    /// # 8. [SERVICE] - 添加评论点赞
    /// * `uid` 用户 ID
    /// * `comment_id` 评论 ID
    /// * `is_liked` 是否点赞
    /// # 8. [SERVICE] - 添加/取消评论点赞
    /// # 6. [SERVICE] - 更新评论点赞状态
    /// * `uid` 用户ID
    /// * `comment_id` 评论ID
    /// * `is_liked` 是否点赞
    pub async fn update_comment_like_by_id(
        uid: i64,
        comment_id: i64,
        is_liked: bool,
    ) -> Result<(), anyhow::Error> {
        // 2. 更新点赞状态（幂等）
        CommentRepo::update_comment_like_by_id(uid, comment_id, is_liked).await?;

        Ok(())
    }

    ////////

    /// # 9. [SERVICE] - 检查评论状态
    pub async fn check_comment_state(_uid: i64, comment_id: i64) -> Result<(), anyhow::Error> {
        // TODO: 购买付费视频/电商挂载商品落单逻辑

        Ok(())
    }

    ////////

    /// # 10. [SERVICE] - 查找最新的视频列表
    pub async fn find_new_video_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, anyhow::Error> {
        VideoRepo::find_new_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 获取最新视频列表失败: {}", e))
    }

    ////////

    /// # 11. [SERVICE] - 查找热门的视频列表
    pub async fn find_hot_video_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, anyhow::Error> {
        VideoRepo::find_hot_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 获取热门视频列表失败: {}", e))
    }

    ////////

}

//////// END
