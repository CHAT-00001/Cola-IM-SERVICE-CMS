// cola_data/src/cola_user/port/home2  -- 数据中心 - USER - 端口层 - HOME
// 2026/6/18 09:05

//////

use async_trait::async_trait;
use crate::cola_user::info::user::UserInfo;

//////

/// # [HOME PORT] - 用户主页端口
/// * `desc` 请求最新注册用户等相关功能
#[async_trait]
pub trait HomePort: Send + Sync + 'static {

    ////////

    /// # 1. [PORT] - 获取最新注册的用户
    async fn get_newest_users(
        &self,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<UserInfo>>;

    ////////

    /// # 2. [PORT] - 获取附近的用户（带距离）
    async fn get_nearby_users(
        &self,
        lat: f64,
        lng: f64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<(UserInfo, Option<f64>)>>;
}
