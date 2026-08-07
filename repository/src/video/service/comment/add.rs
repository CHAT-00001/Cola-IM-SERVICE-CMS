// repository/src/new/service/comment/active
// 仓储 - VIDEO - service - comment - add 发布评论
// 2026/8/2 17:15 Created.

////////


use crate::video::pg::comment::comment::CommentRepo;
use crate::video::pg::video::count::VideoCountRepo;
use crate::video::pg::video::video::VideoRepo;
use anyhow::Error;
use cola_data::video::command::comment::CommentCommand;
use cola_data::video::entity::video::video::VideoEntity;
use cola_data::video::info::comment::VideoCommentInfo;
use tracing::log;

////////

/// # [SERVICE] - 视频 评论 发布 服务
pub struct CommentAddService;

impl CommentAddService {
    //

    ////////

    /// # 1. [SERVICE] - 保存评论 + 更新计数
    pub async fn save_comment_and_update_count(
        uid: i64,            // 用户 ID
        video_id: i64,       // 视频 ID
        cmd: CommentCommand, // 评论创建命令
        visibility: i16,     // 风控可见性
    ) -> Result<VideoCommentInfo, anyhow::Error> {
        // 1. 保存评论
        let comment_entity = CommentRepo::save_comment_by_video_id(uid, video_id, cmd, visibility)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 保存视频评论失败: {}", e))?;

        // 2. 更新计数
        let async_video_id = video_id;
        tokio::spawn(async move {
            if let Err(e) = VideoCountRepo::pg_update_video_comments(async_video_id, 1).await {
                tracing::error!(
                    "[🔌 ADAPTER]: ▶ 异步更新视频评论计数失败: video_id={}, err={:?}",
                    async_video_id,
                    e
                );
            }
        });

        // 3. ✅ 修复 [E0308]：显式进行 Entity -> Info 的类型转换
        let comment_info = VideoCommentInfo::from_entity(comment_entity);

        Ok(comment_info)
    }

    ////////

    /// # 2. [SERVICE] - 删除评论 + 更新计数
    pub async fn delete_comment_and_update_count(
        uid: i64,        // 用户 ID
        comment_id: i64, // 评论 ID
    ) -> Result<bool, anyhow::Error> {
        // 🌟 听哥们的, 简单直接返回 bool

        // 1. 调用底层仓储删除评论，顺便返回被删除的数据行（包含 video_id）
        let comment_entity = CommentRepo::user_del_comment_by_id(uid, comment_id)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 删除视频评论失败: {}", e))?;

        // 2. 🛡️ 安全拦截：从刚删掉的数据里拿到它属于哪个视频
        let target_video_id = comment_entity.video_id;

        // 3. 联动异步更新计数器：评论数 -1 (用 move 彻底带走 target_video_id)
        tokio::spawn(async move {
            if let Err(e) = VideoCountRepo::pg_update_video_comments(target_video_id, -1).await {
                tracing::error!(
                    "[🔌 ADAPTER]: ▶ 异步更新视频评论计数失败: video_id={}, err={:?}",
                    target_video_id,
                    e
                );
            }
        });

        // 4. ✅ 搞定收工，直接返回 true
        Ok(true)
    }

    ////////

    /// # 3. [SERVICE] - 获取视频的评论
    /// * `video_id`  视频 ID
    pub async fn find_comments_by_video_id(
        video_id: i64,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<VideoCommentInfo>, anyhow::Error> {
        let entities = CommentRepo::find_new_comments_by_video_id(video_id, offset, limit).await?;

        // handler -> info
        let infos: Vec<VideoCommentInfo> = entities
            .into_iter()
            .map(VideoCommentInfo::from_entity)
            .collect();

        Ok(infos)
    }

    ////////

    /// # 4. [SERVICE] - 查找用户的评论
    /// * `user_id`  用户 ID
    pub async fn find_comments_by_user_id(
        video_id: i64,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<VideoCommentInfo>, anyhow::Error> {
        let entities = CommentRepo::find_comments_by_user_id(video_id, offset, limit).await?;

        // handler -> info
        let infos: Vec<VideoCommentInfo> = entities
            .into_iter()
            .map(VideoCommentInfo::from_entity)
            .collect();

        Ok(infos)
    }

    ////////

    /// # 5. [SERVICE] - 浏览我发布评论
    /// * `user_id`  视频 ID
    pub async fn view_my_publish_comments(
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<VideoCommentInfo>, anyhow::Error> {
        let entities = CommentRepo::find_comments_by_user_id(user_id, offset, limit).await?;

        // handler -> info
        let infos: Vec<VideoCommentInfo> = entities
            .into_iter()
            .map(VideoCommentInfo::from_entity)
            .collect();

        Ok(infos)
    }

    ////////

    /// # 6. [SERVICE] - 点赞
    /// * `desc` 点赞评论
    pub async fn update_comment_like_by_id(
        uid: i64,
        comment_id: i64,
        is_liked: bool,
    ) -> Result<(), anyhow::Error> {
        // 2. 更新点赞状态（幂等）
        CommentRepo::update_comment_like_by_id(uid, comment_id, is_liked).await?;

        Ok(())
    }

    /// # 8. [SERVICE] - 不喜欢
    /// * `desc` 点赞评论
    pub async fn update_comment_unlike_by_id(
        uid: i64,
        comment_id: i64,
        is_unliked: bool,
    ) -> Result<(), anyhow::Error> {
        // 2. 更新点赞状态（幂等）
        CommentRepo::update_comment_unlike_by_id(Some(uid), comment_id, is_unliked).await?;

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
            .map_err(|e| anyhow::anyhow!("[🔌 ADAPTER]: ▶ 获取最新视频列表失败: {}", e))
    }

    ////////

    /// # 11. [SERVICE] - 查找热门的视频列表
    pub async fn find_hot_video_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, anyhow::Error> {
        VideoRepo::find_hot_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[🔌 ADAPTER]: ▶ 获取热门视频列表失败: {}", e))
    }

    ////////

    /// # 12. [SERVICE] - 查找推荐的视频列表
    pub async fn find_recommend_video_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, anyhow::Error> {
        VideoRepo::find_recommend_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[🔌 ADAPTER]: ▶ 获取推荐视频列表失败: {}", e))
    }

    ////////
}

//////// END
