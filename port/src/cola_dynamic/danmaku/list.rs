// port/src/cola_dynamic/danmaku/list.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 弹幕 - 列表
// 2026/8/5 00:06 Created

////////

use cola_data::cola_video::info::danmaku::DanmakuInfo;

////////

/// # [LIST PORTS] - 列表
/// * `desc`: `⏹ 可乐动态 - 弹幕前台列表端口`
#[async_trait::async_trait]
pub trait DynamicDanmakuListPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 新的
    async fn get_new_danmaku_infos(
        &self,
        uid: i64,        // UID
        dynamic_id: i64, // 动态 ID
        play_time: i32,  // 时间
        limit: i64,      // 数量
        offset: i64,     // 页码
    ) -> anyhow::Result<(Vec<DanmakuInfo>)>;

    ////////

    /// # 2. [PORT] - 热门
    async fn get_hot_danmaku_infos(
        &self,
        uid: i64,        // UID
        dynamic_id: i64, // 动态 ID
        play_time: i32,  // 时间
        limit: i64,      // 数量
        offset: i64,     // 页码
    ) -> anyhow::Result<(Vec<DanmakuInfo>)>;

    ////////

    /// # 3. [PORT] - 同城
    async fn get_city_danmaku_infos(
        &self,
        uid: i64,        // UID
        dynamic_id: i64, // 动态 ID
        play_time: i32,  // 时间
        limit: i64,      // 数量
        offset: i64,     // 页码
    ) -> anyhow::Result<(Vec<DanmakuInfo>)>;

    ////////


    /// # 4. [PORT] - 回复
    async fn get_relay_danmaku_infos(
        &self,
        uid: i64,        // UID
        dynamic_id: i64, // 动态 ID
        limit: i64,      // 数量
        offset: i64,     // 页码
    ) -> anyhow::Result<(Vec<DanmakuInfo>)>;
}

//////// END