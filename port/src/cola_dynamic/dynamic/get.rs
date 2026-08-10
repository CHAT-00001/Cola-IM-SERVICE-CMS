// port/src/cola_dynamic/dynamic/get.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 动态 - 获取
// 2026/8/5 00:00 Created.

////////

use cola_data::cola_dynamic::info::dynamic::DynamicInfo;

////////

/// # [GET PORTS] -  获取
/// * `desc`: `⏹ 可乐动态 - 动态获取端口`
#[async_trait::async_trait]
pub trait GetPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 我的
    /// * `desc`: `⏹ 可乐动态 - 根据UID 获取自己发布的动态`
    /// * `condition`: `无视状态/权限`
    async fn get_my_list(
        &self,
        uid: i64,                // UID
        keyword: Option<String>, // 关键词
        limit: i64,              // 数量
        offset: i64,             // 页码
        is_liked: bool,
    ) -> anyhow::Result<(Vec<DynamicInfo>)>;

    ////////

    /// # 2. [PORT] - TA的
    /// * `desc`: `⏹ 可乐动态 - 根据用户ID 获取他人发布的动态`
    /// * `condition`: `受限状态/权限`
    async fn get_he_list(
        &self,
        user_id: i64,            // 用户 ID
        keyword: Option<String>, // 关键词
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> anyhow::Result<(Vec<DynamicInfo>)>;

    ////////

    /// # 8. [PORT] - 附近
    async fn get_nearby_list(
        &self,
        lat: f64,    // 纬度
        lng: f64,    // 经度
        range: f64,  // 范围
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<DynamicInfo>)>;
}

//////// END
