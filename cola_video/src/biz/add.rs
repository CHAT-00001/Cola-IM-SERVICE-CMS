// cola_video/src/biz/add.rs  -- VIDEO - 逻辑层 - 发布
// 2026-06-05 00:35

////////

use anyhow::{Context, Result};
use futures_util::TryFutureExt;
use tracing::info;
use cola_data::risk::rick_check;
use cola_data::video::command::buy::BuyCommand;
use cola_data::video::command::collect::CollectCommand;
use cola_data::video::command::comment::CommentCommand;
use cola_data::video::command::danmaku::DanmakuCommand;
use cola_data::video::command::hotlist::HotlistCommand;
use cola_data::video::command::recommend::RecommendCommand;
use cola_data::video::command::report::ReportCommand;
use cola_data::video::command::share::ShareCommand;
use cola_data::video::command::video::VideoCommand; // 👈 确保这行路径对齐你移动后的位置
use cola_data::video::entity::collect::CollectEntity;
use cola_data::video::entity::comment::CommentEntity;
use cola_data::video::entity::danmaku::DanmakuEntity;
use crate::model::vo::video::VideoSingleResponse;
use repo::video::service::video::VideoService;      // 👈 统一注入核心服务层适配器

////////

pub struct AddLogic;

impl AddLogic {

    /// # 1. [LOGIC] - 发布视频
    pub async fn logic_add_publish(uid: i64, cmd: VideoCommand) -> Result<VideoSingleResponse, anyhow::Error> {
        // 1. 内容风控（标题 + 简介 联合过滤）
        let check_text = format!("{} {}", cmd.title, cmd.description);

        // 执行风控服务
        let visibility = rick_check(check_text)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 风控服务异常: {}", e))?;

        // 2. 核心数据持久化与计数更新 (直接走 Service 静态调用)
        VideoService::save_video_and_update_count(uid, cmd, visibility)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 视频发布持久化失败: {}", e))?;

        info!("BIZ - 视频发布成功: uid={}, visibility={}", uid, visibility);

        Ok(VideoSingleResponse {
            info: Default::default(),
        })
    }

    ////////

    /// # 2. [LOGIC] - 发布评论
    pub async fn logic_add_comment(
        uid: i64,
        cmd: CommentCommand,
    ) -> Result<CommentEntity> {
        // 剥离 Trait，直接下探到 Service 层的纯静态方法
        let mut saved = VideoService::save_comment_and_update_count(uid, cmd)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 评论保存失败: {}", e))?;

        let entity = saved.pop().context("BIZ: 数据库未返回保存后的实体数据")?;

        info!("BIZ - 评论发布成功: uid={}", uid);
        Ok(entity)
    }

    ////////

    /// # 3. [LOGIC] - 发布弹幕
    pub async fn logic_add_danmaku(
        uid: i64,
        cmd: DanmakuCommand,
    ) -> Result<DanmakuEntity> {
        // 剥离老旧 Context 的多余包裹，直接进行静态服务处理
        let mut saved = VideoService::save_danmaku_and_update_count(uid, cmd)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 弹幕保存失败: {}", e))?;

        let entity = saved.pop().context("BIZ: 数据库未返回保存后的实体数据")?;
        info!("BIZ - 弹幕发布成功: uid={}", uid);
        Ok(entity)
    }

    ////////

    /// # 4. [LOGIC] - 收藏
    pub async fn logic_add_collect(
        uid: i64,
        video_id: i64,
        cmd: CollectCommand,
    ) -> Result<CollectEntity> {
        let entity = VideoService::save_collect_and_update_count(uid, video_id, cmd)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 收藏失败: {}", e))?;

        info!("BIZ - 收藏成功: uid={}, video_id={}", uid, video_id);
        Ok(entity)
    }

    ////////

    /// # 5. [LOGIC] - 分享
    pub async fn logic_add_share(
        uid: i64,
        video_id: i64,
        cmd: ShareCommand,
    ) -> Result<()> {
        VideoService::save_share_and_update_count(uid, video_id, cmd)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 分享失败: {}", e))?;

        info!("BIZ - 分享成功: uid={}, video_id={}", uid, video_id);
        Ok(())
    }

    ////////

    /// # 6. [LOGIC] - 上热门
    pub async fn logic_add_hotlist(
        uid: i64,
        _video_id: i64,
        cmd: HotlistCommand,
    ) -> Result<()> {
        VideoService::save_hotlist_and_update_count(uid, cmd)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 上热门失败: {}", e))?;

        info!("BIZ - 上热门成功: uid={}", uid);
        Ok(())
    }

    ////////

    /// # 7. [LOGIC] - 推荐
    pub async fn logic_add_recommend(
        uid: i64,
        _video_id: i64,
        cmd: RecommendCommand,
    ) -> Result<()> {
        VideoService::save_recommend_and_update_count(uid, cmd)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 推荐失败: {}", e))?;

        info!("BIZ - 推荐成功: uid={}", uid);
        Ok(())
    }

    ////////

    /// # 8. [LOGIC] - 举报
    pub async fn logic_add_report(
        uid: i64,
        _video_id: i64,
        cmd: ReportCommand,
    ) -> Result<()> {
        VideoService::save_report_info(uid, cmd)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 举报失败: {}", e))?;

        info!("BIZ - 举报成功: uid={}", uid);
        Ok(())
    }

    ////////

    /// # 9. [LOGIC] - 购买
    pub async fn logic_add_buy(
        uid: i64,
        _video_id: i64,
        cmd: BuyCommand
    ) -> Result<()> {
        VideoService::save_buy_and_update_count(uid, cmd)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 购买失败: {}", e))?;

        info!("BIZ - 购买成功: uid={}", uid);
        Ok(())
    }
}

//////// END