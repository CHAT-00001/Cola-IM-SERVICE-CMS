// servic/feed.rs  -- 服务层 流
// 2026/6/10 17:44

////////

use std::collections::HashMap;
use anyhow::Error;
use crate::video::pg::user::UserRepo;
use crate::video::pg::video::{VideoRepo};
use cola_data::video::entity::video::VideoEntity;
use cola_data::video::command::video::VideoCommand;
use cola_data::video::command::comment::CommentCommand;
use cola_data::video::command::collect::CollectCommand;
use cola_data::video::command::share::ShareCommand;
use cola_data::video::entity::comment::CommentEntity;
use cola_data::video::entity::collect::CollectEntity;
use tracing::log;

////////

/// # [FEED SERVICE] - 流服务
pub struct FeedService;

impl FeedService {

    ////////

    /// # 5001. [SERVICE] - 保存视频 + 更新计数 (纯静态函数适配器)
    pub async fn get_user_collected_list(
        uid: i64,          // 用户 ID 核心参数
        cmd: VideoCommand, // 视频创建命令
        visibility: i16,   // 风控可见性
    ) -> Result<VideoEntity, anyhow::Error> {
        // 调用底层仓储 - 保存视频并直接返回插入后的实体数据
        let video_entity = VideoRepo::save_video_by_uid(uid, cmd, visibility)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 写入视频主表失败: {}", e))?;

        // 联动更新计数器：发布视频数 + 1
        let async_uid = uid;
        tokio::spawn(async move {
            if let Err(e) = UserRepo::update_user_count(async_uid, 1, 0, 0, 0, 0, 0).await {
                log::error!("SERVICE_ASYNC: 异步更新用户视频计数失败: uid={}, err={:?}", async_uid, e);
            }
        });

        Ok(video_entity)
    }

    ////////

    /// # 2. [SERVICE] - 保存评论 + 更新计数
    pub async fn save_comment_and_update_count(
        uid: i64,
        cmd: CommentCommand,
    ) -> Result<Vec<CommentEntity>, anyhow::Error> {
        // TODO: 替换为你底层的 CommentRepo 真实物理落库
        // 这里返回一组数据，模拟老代码中用 pop 提取实体的行为
        let mock_entity = CommentEntity::default();
        let saved_list = vec![mock_entity];

        // 异步更新用户的评论/互动相关计数（如果后续要在用户表增加 comment_count，在这里改 0）
        let _async_uid = uid;
        tokio::spawn(async move {
            // 目前短视频用户表没有细分评论数，预留
        });

        Ok(saved_list)
    }

    ////////

    /// # 3. [SERVICE] - 检查视频状态
    pub async fn check_video_state(
        video_id: i64,
    ) -> Result<i32, sqlx::Error> {
        let code = 4001;
        Ok(code)
    }


    ////////

    /// # 3. [SERVICE] - 检查视频状态
    // 假设你的项目使用 anyhow 或自定义错误类型
    pub async fn check_user_state(
        video_id: i64,
    ) -> Result<i32, sqlx::Error> {
        let code = 4001;
        Ok(code)
    }

    ////////

    /// # 4. [SERVICE] - 保存收藏 + 更新计数
    pub async fn save_collect_and_update_count(
        uid: i64,
        _video_id: i64,
        _cmd: CollectCommand,
    ) -> Result<CollectEntity, anyhow::Error> {
        // TODO: 替换为你底层的 CollectRepo 真实物理落库
        let collect_entity = CollectEntity::default();

        // 联动更新计数器：收藏的视频数量 + 1
        let async_uid = uid;
        tokio::spawn(async move {
            // 收藏字段在第四位：publish, liked, total_favorited, collected
            if let Err(e) = UserRepo::update_user_count(async_uid, 0, 0, 0, 1, 0, 0).await {
                log::error!("SERVICE_ASYNC: 异步更新用户收藏计数失败: uid={}, err={:?}", async_uid, e);
            }
        });

        Ok(collect_entity)
    }

    ////////

    /// # 5. [SERVICE] - 保存分享 + 更新计数
    pub async fn save_share_and_update_count(
        uid: i64,
        _video_id: i64,
        _cmd: ShareCommand,
    ) -> Result<(), anyhow::Error> {
        // TODO: 落地分享日志记录表

        Ok(())
    }

    ////////


    ////////

    /// # 7001. [SERVICE] - 获取用户发布的视频列表
    pub async fn find_user_publish_list(
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoEntity>, anyhow::Error> {
        VideoRepo::find_new_list_by_user_id(user_id, limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 获取用户{}发布的最新视频列表失败: {}", user_id, e))
    }


}

//////// END