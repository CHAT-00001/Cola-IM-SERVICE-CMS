// service/src/cola_video/danmaku/get.rs -- 服务层 - VIDEO - 弹幕 - 获取服务
// 2026/8/2 18:56 Created.

////////

use anyhow::Error;
use cola_data::cola_video::command::buy::VideoBuyCommand;
use cola_data::cola_video::command::collect::CollectCommand;
use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::command::danmaku::DanmakuCommand;
use cola_data::cola_video::command::share::ShareCommand;
use cola_data::cola_video::entity::danmaku::DanmakuEntity;
use cola_data::cola_video::entity::video::video::VideoEntity;
use cola_data::cola_video::info::comment::CommentInfo;
use cola_data::cola_video::info::danmaku::DanmakuInfo;
use repository::cola_gis::pg::user::UserRepo;
use repository::video::pg::danmaku::add::DanmakuAddRepo;
use repository::video::pg::danmaku::danmaku::DanmakuRepo;
use repository::video::pg::danmaku::get::DanmakuGetRepo;
use repository::video::pg::video::home::VideoRepo;
use tracing::log;
////////

/// # [GET SERVICE] - 视频弹幕获取服务
/// * `desc`: `VIDEO - Danmaku Add Service`
pub struct VideoDanmakuGetService;

// 构造实现
impl VideoDanmakuGetService {
    //

    ////////

    /// # 1. [SERVICE] - 视频的弹幕
    /// * `desc`: `根据视频ID和播放器轨道时间获取弹幕列表`
    pub async fn get_danmaku_infos_by_video_id(
        video_id: i64,
        play_time: i32,
        time_window: i32,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<DanmakuInfo>, anyhow::Error> {
        ////////

        // 💡 - ENTITIES - 弹幕实体（⚠️ repo 入参顺序为 limit, offset，此处不可颠倒）
        let entities =
            DanmakuRepo::find_danmaku_by_video_id(video_id, play_time, time_window, limit, offset)
                .await?;

        ////////

        // 💡 - INFO - 数据转换
        let infos: Vec<DanmakuInfo> = entities.into_iter().map(DanmakuInfo::from_entity).collect();

        ////////

        // ✅️ - Ok
        Ok(infos)
    }

    ////////

    /// # 2. [SERVICE] - 用户的弹幕
    /// * `desc` 根据用户ID和获取弹幕列表
    pub async fn get_danmaku_infos_by_user_id(
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<DanmakuInfo>, anyhow::Error> {
        ////////

        // 💡 - ENTITIES - 弹幕实体
        let entities = DanmakuRepo::find_danmaku_by_user_id(user_id, offset, limit).await?;

        ////////

        // 💡 - INFOS - 数据转换
        let infos: Vec<DanmakuInfo> = entities.into_iter().map(DanmakuInfo::from_entity).collect();

        ////////

        // ✅️ - Ok
        Ok(infos)
    }

    ////////

    /// # 3. [SERVICE] - 单个获取
    /// * `desc` 根据用户ID和获取弹幕列表
    pub async fn get_danmaku_info_by_danmaku_id(
        danmaku_id: i64, // 弹幕 ID
    ) -> Result<Vec<DanmakuInfo>, anyhow::Error> {
        ////////

        // 💡 - ENTITIES - 弹幕实体
        let entities = DanmakuGetRepo::find_one_by_id(danmaku_id).await?;

        ////////

        // 💡 - INFOS - 数据转换
        let infos: Vec<DanmakuInfo> = entities.into_iter().map(DanmakuInfo::from_entity).collect();

        ////////

        // ✅️ - Ok
        Ok(infos)
    }

    ////////

    /// # 4. [SERVICE] - 批量获取
    /// * `desc` 根据用户ID和获取弹幕列表
    pub async fn get_danmaku_infos_by_danmaku_ids(
        danmaku_ids: Vec<i64>, // 弹幕 IDs
    ) -> Result<Vec<DanmakuInfo>, anyhow::Error> {
        ////////

        // 💡 - ENTITIES - 弹幕实体
        let entities = DanmakuGetRepo::find_all_by_ids(danmaku_ids).await?;

        ////////

        // 💡 - INFOS - 数据转换
        let infos: Vec<DanmakuInfo> = entities.into_iter().map(DanmakuInfo::from_entity).collect();

        ////////

        // ✅️ - Ok
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
