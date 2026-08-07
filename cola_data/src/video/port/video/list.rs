// video/port/video/list.rs
// 视频 - port - 视频 - 列表
// 2026/8/5 00:23 Created.

////////

use crate::video::info::video::VideoInfo;

////////

/// # [LIST SERVICE] -  列表
/// * `desc`: `视频列表服务接口`
#[async_trait::async_trait]
pub trait VideoListPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 新的
    async fn get_new_list(
        &self,
        uid: i64,    // UID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<VideoInfo>)>;

    ////////

    /// # 2. [PORT] - 热门
    async fn get_hot_list(
        &self,
        uid: i64,    // UID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<VideoInfo>)>;

    ////////

    /// # 3. [PORT] - 推荐
    async fn get_recommend_list(
        &self,
        uid: i64,    // UID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<VideoInfo>)>;

    ////////

    /// # 4. [PORT] - 分类
    async fn get_category_list(
        &self,
        uid: i64,         // UID
        category_id: i64, // 分类ID
        limit: i64,       // 数量
        offset: i64,      // 页码
    ) -> anyhow::Result<(Vec<VideoInfo>)>;

    ////////

    /// # 5. [PORT] - 频道
    async fn get_channel_list(
        &self,
        uid: i64,        // UID
        channel_id: i64, // 通道 ID
        limit: i64,      // 数量
        offset: i64,     // 页码
    ) -> anyhow::Result<(Vec<VideoInfo>)>;

    ////////

    /// # 6. [PORT] - 同城
    async fn get_city_list(
        &self,
        uid: i64,     // UID
        city_id: i64, // 城市 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<VideoInfo>)>;

    ////////

    /// # 7. [PORT] - 附近
    async fn get_nearby_list(
        &self,
        lat: f64,   // 纬度
        lng: f64,   // 经度
        range: f64, // 范围
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<VideoInfo>)>;

    ////////

    /// # 8. [PORT] - 搜索
    async fn get_search_list(
        &self,
        uid: i64,                // UID
        keyword: Option<String>, // 关键词
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> anyhow::Result<(Vec<VideoInfo>)>;
}

//////// END
