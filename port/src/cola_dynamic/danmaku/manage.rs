// port/src/cola_dynamic/danmaku/manage.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 弹幕 - 管理
// 2026/8/5 00:06 Created.

////////

use cola_data::cola_video::info::danmaku::DanmakuInfo;

////////

/// # [MANAGE PORTS] - 管理
/// * `desc`: `⏹ 可乐动态 - 弹幕管理端口`
#[async_trait::async_trait]
pub trait DynamicDanmakuManagePort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 浏览
    /// * `desc`: `获取全部弹幕` - 包含隐藏弹幕
    /// * `condition`: `⚠️ ADMIN 身份` -
    async fn get_all_danmaku_infos(
        &self,
        uid: i64,              // UID
        user_id: Option<i64>,  // 用户 ID (可选)
        video_id: Option<i64>, // 视频 ID (可选)
        limit: i64,            // 数量
        offset: i64,           // 页码
    ) -> anyhow::Result<(u64, Vec<DanmakuInfo>)>;

    ////////

    /// # 3. [PORT] - 删除
    async fn del_danmaku_record(&self, uid: i64, danmaku_id: i64) -> anyhow::Result<()>;

    ////////

    /// # 4. [PORT] - 批量删除
    async fn del_danmakus_record(&self, uid: i64, danmaku_ids: Vec<i64>) -> anyhow::Result<()>;

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