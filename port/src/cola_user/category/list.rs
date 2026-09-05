// port/src/user/category/list.rs -- 端口 - USER - 分类 - 列表端口
// 2026/8/5 21:58 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::category::UserCategoryInfo;
use cola_data::cola_user::info::config::UserConfigInfo;
use cola_data::cola_user::info::user::UserInfo;
////////

/// # [LIST PORT]
/// * `desc`: `USER - 用户分类列表端口`
#[async_trait]
pub trait UserCategoryListPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 最新
    async fn get_new_list(
        &self,
        limit: i64,  // 数量
        offset: i64, // 页数
    ) -> anyhow::Result<(Vec<UserCategoryInfo>)>;

    ////////

    /// # 2. [PORT] - 热门
    async fn get_hot_list(
        &self,
        limit: i64,  // 数量
        offset: i64, // 页数
    ) -> anyhow::Result<(Vec<UserCategoryInfo>)>;
}

//////// END
