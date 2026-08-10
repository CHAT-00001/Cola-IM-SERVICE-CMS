// port/src/cola_dynamic/danmaku/add.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 弹幕 - 获取
// 2026/8/9 04:46 Created.

////////

use cola_data::cola_video::info::danmaku::DanmakuInfo;

////////

/// # [GET PORTS] - 发布
/// * `desc`: `⏹ 可乐动态 - 弹幕信息获取服务`
#[async_trait::async_trait]
pub trait DynamicDanmakuGetPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 单个信息
    /// * `desc`: `⏹ 可乐动态 - 根据弹幕ID单个获取弹幕信息`
    async fn get_danmaku_info_by_id(
        &self,
        danmaku_id: i64, // 弹幕 ID
    ) -> anyhow::Result<(DanmakuInfo)>;

    ////////

    /// # 2. [PORT] - 批量信息
    /// * `desc`: `⏹ 可乐动态 - 根据弹幕IDs批量获取弹幕信息`
    async fn get_danmaku_infos_by_ids(
        &self,
        danmaku_ids: Vec<i64>, // 弹幕 IDs
    ) -> anyhow::Result<(Vec<DanmakuInfo>)>;

    ////////

    /// # 3. [PORT] - 动态的
    async fn get_danmaku_infos_by_video_id(
        &self,
        uid: i64,        // UID
        dynamic_id: i64, // 动态 ID
        limit: i64,      // 数量
        offset: i64,     // 页码
    ) -> anyhow::Result<(Vec<DanmakuInfo>)>;

    ////////

    /// # 4. [PORT] - 用户的
    async fn get_danmaku_infos_by_user_id(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<DanmakuInfo>)>;

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
