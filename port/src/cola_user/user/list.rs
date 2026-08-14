// port/src/user/user/list.rs
// 用户 - port - 用户 - 列表
// 2026/8/5 22:03 Created.

////////

use cola_data::cola_user::info::user::UserInfo;


////////

/// # [LIST PORTS] - 列表
/// * `desc`: `用户列表端口`
#[async_trait::async_trait]
pub trait UserListPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 最新
    /// * `desc`: `获取最新用户列表`
    async fn get_new_list(
        &self,
        uid: i64,    // 操作者ID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<UserInfo>)>;

    ////////

    /// # 2. [PORT] - 热门
    /// * `desc`: `获取热门用户列表`
    async fn get_hot_list(
        &self,
        uid: i64,    // 操作者ID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<UserInfo>)>;

    ////////

    /// # 3. [PORT] - 推荐
    /// * `desc`: `获取推荐的用户列表`
    async fn get_recommend_list(
        &self,
        uid: i64,    // 操作者ID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<UserInfo>)>;

    ////////

    /// # 4. [PORT] - 同城
    /// * `desc`: ``
    async fn get_city_list(
        &self,
        uid: i64,     // 操作者ID
        city_id: i64, // 城市ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<UserInfo>)>;

    ////////

    /// # 5. [PORT] - 附近
    /// * `desc`: `获取附近的用户列表`
    async fn get_nearby_list(
        &self,
        uid: i64,    // 操作者ID
        lat: f64,    // 经度
        lng: f64,    // 纬度
        range: u32,  // 半径范围(米)
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<UserInfo>)>;

    ////////

    /// # 6. [PORT] - 类别
    /// * `desc`: `获取分类下的用户列表`
    async fn get_category_list(
        &self,
        uid: i64,         // 操作者ID
        category_id: i64, // 分类ID
        limit: i64,       // 数量
        offset: i64,      // 页码
    ) -> anyhow::Result<(Vec<UserInfo>)>;

    ////////

    /// # 7. [PORT] - 搜索
    /// * `desc`: `搜索用户列表`
    async fn get_search_list(
        &self,
        uid: i64,        // 操作者ID
        keyword: String, // 关键词(检索昵称/个签/描述等)
        limit: i64,      // 数量
        offset: i64,     // 页码
    ) -> anyhow::Result<(Vec<UserInfo>)>;
}

//////// END
