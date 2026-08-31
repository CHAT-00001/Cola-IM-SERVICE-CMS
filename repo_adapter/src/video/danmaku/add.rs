// repo_adapter/src/video/danmaku/add.rs -- 🔌 适配器 - VIDEO - 弹幕 - 发布
// 2026/8/9 22:12 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::command::danmaku::DanmakuCommand;
use cola_data::cola_video::info::danmaku::DanmakuInfo;
use port::cola_video::danmaku::add::VideoDanmakuAddPort;

////////

/// # [ADD ADAPTER] - danmaku add
/// * `desc`: `VIDEO - 视频弹幕发布适配器`
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
    ) -> Result<(DanmakuInfo)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 编辑弹幕
    /// * `desc`: `根据用户ID + 弹幕ID` - `编辑弹幕记录`
    async fn edit_danmaku(
        &self,
        uid: i64,            // UID
        danmaku_id: i64,     // 弹幕 ID
        cmd: DanmakuCommand, // 命令
    ) -> Result<(DanmakuInfo)> {
        todo!()
    }
}

//////// END
