// service/danmaku.rs  -- 仓储中心 - VIDEO - 服务层 - 弹幕
// 2026/6/8 19:17

////////

use crate::video::pg::user::UserRepo;
use crate::video::pg::video::{VideoRepo};
use anyhow::Error;
use cola_data::video::command::video::VideoCommand;
use cola_data::video::entity::video::VideoEntity;
// 引入缺少的 command 和 handler 结构
use crate::video::pg::comment::CommentRepo;
use crate::video::pg::danmaku::DanmakuRepo;
use cola_data::video::command::buy::BuyCommand;
use cola_data::video::command::collect::CollectCommand;
use cola_data::video::command::comment::CommentCommand;
use cola_data::video::command::danmaku::DanmakuCommand;
use cola_data::video::command::hotlist::HotlistCommand;
use cola_data::video::command::recommend::RecommendCommand;
use cola_data::video::command::report::ReportCommand;
use cola_data::video::command::share::ShareCommand;
use cola_data::video::entity::collect::CollectEntity;
use cola_data::video::entity::comment::CommentEntity;
use cola_data::video::entity::danmaku::DanmakuEntity;
use cola_data::video::info::comment::CommentInfo;
use cola_data::video::info::danmaku::DanmakuInfo;
use tracing::log;
////////

/// # [SERVICE] - 弹幕服务
pub struct DanmakuService;

impl DanmakuService {
    ////////

    /// # 1. [SERVICE] - 保存弹幕 + 更新计数
    /// * `video_id` 视频 ID
    /// * `cmd` 弹幕命令
    pub async fn save_danmaku_and_update_count(
        uid: i64,            // 用户 ID 核心参数
        video_id: i64,       //  视频ID
        cmd: DanmakuCommand, // 弹幕创建命令
        visibility: i16,     // 风控可见性
    ) -> Result<DanmakuEntity, anyhow::Error> {
        // 调用底层仓储 - 保存视频并直接返回插入后的实体数据
        let video_entity = DanmakuRepo::save_danmaku_by_video_id(uid, video_id, cmd, visibility)
            .await
            .map_err(|e| anyhow::anyhow!("SERVICE: 写入视频主表失败: {}", e))?;

        // 联动更新计数器：发布视频数 + 1
        let async_uid = uid;
        tokio::spawn(async move {
            if let Err(e) = UserRepo::update_user_count(async_uid, 1, 0, 0, 0, 0, 0).await {
                log::error!(
                    "SERVICE_ASYNC: 异步更新用户视频计数失败: uid={}, err={:?}",
                    async_uid,
                    e
                );
            }
        });

        Ok(video_entity)
    }

    ////////

    /// # 2. [SERVICE] - 浏览
    /// * `desc` 根据视频ID和播放器轨道时间获取弹幕列表
    pub async fn get_video_danmaku(
        video_id: i64,
        play_time: i32,
        time_window: i32,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<DanmakuInfo>, anyhow::Error> {
        // 弹幕实体
        let entities =
            DanmakuRepo::find_danmaku_by_video_id(video_id, play_time, time_window, offset, limit)
                .await?;

        // handler -> info
        let infos: Vec<DanmakuInfo> = entities.into_iter().map(DanmakuInfo::from_entity).collect();

        Ok(infos)
    }

    /// # 2. [SERVICE] - 获取用户发布的弹幕
    /// * `desc` 根据用户ID和获取弹幕列表
    pub async fn get_user_danmaku(
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<DanmakuInfo>, anyhow::Error> {
        // 弹幕实体
        let entities =
            DanmakuRepo::find_danmaku_by_user_id(user_id, offset, limit)
                .await?;

        // handler -> info
        let infos: Vec<DanmakuInfo> = entities.into_iter().map(DanmakuInfo::from_entity).collect();

        Ok(infos)
    }

    ////////

    /// # 4. [SERVICE] - 删除弹幕 + 更新计数
    /// * `uid` 用户ID
    pub async fn delete_danmaku_and_update_count(
        uid: i64,
        danmaku_id: i64,
    ) -> Result<(), anyhow::Error> {

        // 1. 先查弹幕（用于获取 video_id + 校验权限）
        let danmaku = DanmakuRepo::find_by_id(danmaku_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("danmaku not found"))?;

        // 2. 权限校验（只能删自己的 or 管理员）
        if danmaku.user_id != uid {
            return Err(anyhow::anyhow!("no permission to delete danmaku"));
        }

        let video_id = danmaku.video_id;

        // 3. 删除弹幕
        DanmakuRepo::user_del_danmaku_by_video_id(danmaku_id).await?;

        // 4. 视频弹幕数 -1
        VideoRepo::sync_decrement_danmaku_count_by_num(video_id, 1).await?;

        Ok(())
    }

    ////////

    /// # 6. [SERVICE] - 添加点赞弹幕 + 更新计数
    pub async fn add_like_and_update_count(
        uid: i64,
        danmaku_id: i64,
        is_liked: bool,
    ) -> Result<CollectEntity, anyhow::Error> {
        // TODO: 替换为你底层的 CollectRepo 真实物理落库
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

    /// # 6. [SERVICE] - 添加不喜欢弹幕 + 更新计数
    pub async fn add_unlike_and_update_count(
        uid: i64,
        danmaku_id: i64,
        is_unliked: bool,
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

    /// # 8. [SERVICE] - 添加收藏弹幕 + 更新计数
    pub async fn add_collect_and_update_count(
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

    /// # 6. [SERVICE] - 上热门 + 扣费扣积分
    pub async fn save_hotlist_and_update_count(
        _uid: i64,
        _cmd: HotlistCommand,
    ) -> Result<(), anyhow::Error> {
        // TODO: 生成热门加热订单，扣除对应虚拟币

        Ok(())
    }

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

    /// # 8. [SERVICE] - 记录举报信息
    pub async fn save_report_info(_uid: i64, _cmd: ReportCommand) -> Result<(), anyhow::Error> {
        // TODO: 写入后台内容风控待人工审核表

        Ok(())
    }

    ////////

    /// # 9. [SERVICE] - 购买内容
    pub async fn save_buy_and_update_count(
        _uid: i64,
        _cmd: BuyCommand,
    ) -> Result<(), anyhow::Error> {
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


}

//////// END
