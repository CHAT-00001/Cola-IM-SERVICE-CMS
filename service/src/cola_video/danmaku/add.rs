// service/src/cola_video/danmaku/add.rs
// 👤 服务 - ▶ 可乐视频 - 弹幕 - 发布
// 2026/8/2 18:54 Created.

////////

use anyhow::Error;
use cola_data::cola_video::command::buy::VideoBuyCommand;
use cola_data::cola_video::command::collect::CollectCommand;
use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::command::danmaku::DanmakuCommand;
use cola_data::cola_video::command::report::VideoReportCommand;
use cola_data::cola_video::command::share::ShareCommand;
use cola_data::cola_video::entity::video::video::VideoEntity;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use cola_data::cola_video::info::danmaku::DanmakuInfo;
use repository::cola_gis::pg::user::UserRepo;
use repository::cola_video::pg::danmaku::danmaku::DanmakuRepo;
use repository::cola_video::pg::video::home::VideoRepo;
use tracing::log;
use cola_data::cola_video::entity::danmaku::DanmakuEntity;

////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `▶可乐视频 - 👤 弹幕发布服务`
pub struct VideoDanmakuAddService;

// 构造实现
impl VideoDanmakuAddService {
    //

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
            .map_err(|e| anyhow::anyhow!("[🔌 ADAPTER]: 💾 写入视频主表失败: {}", e))?;

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
        let entities = DanmakuRepo::find_danmaku_by_user_id(user_id, offset, limit).await?;

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
}

//////// END
