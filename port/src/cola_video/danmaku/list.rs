// cola_video/port/danmaku/list.rs
// ⏩️ 端口 - ▶ 可乐视频 - 弹幕 - 列表
// 2026/8/5 00:06 Created

////////

use cola_data::cola_video::info::danmaku::DanmakuInfo;


////////

/// # [LIST SERVICE] - 列表
/// * `desc`: `获取弹幕列表`
#[async_trait::async_trait]
pub trait VideoDanmakuListPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 获取视频ID的弹幕
    async fn get_danmaku_by_video_id(
        &self,
        uid: i64,
        video_id: i64,
        play_time: i32,
        qty: i32,
    ) -> anyhow::Result<(Vec<DanmakuInfo>, i64)>;

    ////////

    /// # 2. [PORT] - 获取用户ID的弹幕
    async fn get_danmaku_by_id(
        &self,
        uid: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<DanmakuInfo>, i64)>;
}

//////// END