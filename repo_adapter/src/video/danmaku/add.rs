// repo_adapter/src/video/danmaku/add.rs -- 适配器 - VIDEO - 弹幕 - 发布
// 2026/8/9 22:12 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::command::danmaku::DanmakuCommand;
use cola_data::cola_video::info::danmaku::DanmakuInfo;
use port::cola_video::danmaku::add::VideoDanmakuAddPort;
use repository::video::pg::danmaku::add::DanmakuAddRepo;

////////

/// # [ADD ADAPTER] - 视频弹幕发布适配器
/// * `desc`: `VIDEO - Danmaku Add Adapter`
#[derive(Debug, Default, Clone)]
pub struct VideoDanmakuaddAdapter;

// 构造实现
#[async_trait]
impl VideoDanmakuAddPort for VideoDanmakuaddAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 发布弹幕
    /// * `desc`: `根据用户ID + 视频ID` - `发布弹幕记录`
    async fn send_danmaku(
        &self,
        uid: i64,            // UID
        video_id: i64,       // 视频 ID
        cmd: DanmakuCommand, // 命令
        visibility: i16,     // 可见范围
    ) -> Result<(DanmakuInfo)> {
        // Call REPO .. 💾 保存弹幕
        let entity = DanmakuAddRepo::save_danmaku_by_video_id(uid, video_id, cmd, visibility)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 ADD ADAPTER]: 💾 保存弹幕失败: {}", e))?;

        // Call CACHE .. ✅️ 回填缓存
        // todo!()

        Ok(DanmakuInfo::from_entity(entity))
    }

    ////////

    /// # 2. [ADAPTER] - 编辑弹幕
    /// * `desc`: `根据用户ID + 弹幕ID` - `编辑弹幕记录`
    async fn edit_danmaku(
        &self,
        uid: i64,            // UID
        danmaku_id: i64,     // 弹幕 ID
        cmd: DanmakuCommand, // 命令
        visibility: i16,     // 可见范围
    ) -> Result<(DanmakuInfo)> {
        todo!()
    }
}

//////// END
