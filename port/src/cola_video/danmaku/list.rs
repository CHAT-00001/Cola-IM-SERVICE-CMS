// cola_video/port/danmaku/list.rs -- 端口 - ▶ 可乐视频 - 弹幕 - 列表
// 2026/8/5 00:06 Created

////////

use cola_data::cola_video::info::danmaku::DanmakuInfo;

////////

/// # [LIST PORT] - 列表
/// * `desc`: `获取弹幕列表`
#[async_trait::async_trait]
pub trait VideoDanmakuListPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 获取视频ID的弹幕
    async fn get_danmaku_by_video_id(
        &self,
        uid: i64,       // 操作者 ID
        video_id: i64,  // 视频 ID
        play_time: i32, // 播放时间
        qty: i32,       // 数量
    ) -> anyhow::Result<(Vec<DanmakuInfo>, i64)>;

    ////////

    /// # 2. [PORT] - 获取用户ID的弹幕
    async fn get_danmaku_by_user_id(
        &self,
        uid: i64,     // 操作者 ID
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<DanmakuInfo>, i64)>;
}

//////// END
