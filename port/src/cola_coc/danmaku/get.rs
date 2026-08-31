// cola_video/port/danmaku/get.rs
// ⏩️ 端口 - ▶ 可乐视频 - 弹幕 - 获取
// 2026/8/5 00:05 Created.

////////

use cola_data::cola_video::info::danmaku::DanmakuInfo;

////////

/// # [GET PORTS] - 发布
/// * `desc`: `弹幕发布服务`
#[async_trait::async_trait]
pub trait VideoDanmakuGetPort: Send + Sync {
    //

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

//////// END
