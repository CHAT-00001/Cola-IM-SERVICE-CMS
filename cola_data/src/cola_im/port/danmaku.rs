// /danmaku.rs  -- 弹幕 服务 端口
// 2026/6/10 23:35

////////

use crate::video::command::danmaku::DanmakuCommand;
use crate::video::info::danmaku::DanmakuInfo;
////////

/// # [PORT] - 弹幕
#[async_trait::async_trait]
pub trait DanmakuRepo: Send + Sync {

    ////////

    /// # 1. [PORT] - 保存
    async fn save_danmaku_record(
        &self,
        uid: i64,
        video_id: i64,
        cmd: DanmakuCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 编辑
    async fn edit_danmaku_record(
        &self,
        uid: i64,
        danmaku_id: i64,
        cmd: DanmakuCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # 3. [PORT] - 删除
    async fn del_danmaku_record(
        &self,
        uid: i64,
        danmaku_id: i64,
    ) -> anyhow::Result<()>;

    ////////

    /// # 4. [PORT] - 批量删除
    async fn del_danmakus_record(
        &self,
        uid: i64,
        danmaku_ids: Vec<i64>,
    ) -> anyhow::Result<()>;

    ////////

    /// # 5. [PORT] - 获取视频ID的弹幕
    async fn get_danmaku_by_video_id(
        &self,
        uid: i64,
        video_id: i64,
        play_time: i32,
        qty: i32,
    ) -> anyhow::Result<(Vec<DanmakuInfo>, i64)>;


    ////////

    /// # 6. [PORT] - 获取用户ID的弹幕
    async fn get_danmaku_by_user_id(
        &self,
        uid: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<DanmakuInfo>, i64)>;
}