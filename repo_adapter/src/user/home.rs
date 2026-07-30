// repo_adapter/src/user/home.rs -- 适配器层 - 用户 HomePort 实现
// 2026/6/18 09:05

//////

use async_trait::async_trait;
use cola_data::user::info::user::UserInfo;
use cola_data::user::port::home::HomePort;
use repository::user::pg::user::UserRepo;

//////

/// # [ADAPTER] - Home 端口适配器
pub struct UserHomePortAdapter;

#[async_trait]
impl HomePort for UserHomePortAdapter {
    ////////

    /// # 1. [PORT IMPL] - 获取最新注册的用户
    async fn get_newest_users(&self, limit: i64, offset: i64) -> anyhow::Result<Vec<UserInfo>> {
        let entities = UserRepo::find_new_user_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("HOME PORT: 查询最新用户失败: {}", e))?;

        Ok(entities.into_iter().map(UserInfo::from).collect())
    }

    ////////

    /// # 2. [PORT IMPL] - 获取附近的用户（带距离）
    /// * `desc` 使用 SQL 内计算的距离字段
    async fn get_nearby_users(
        &self,
        lat: f64,
        lng: f64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<(UserInfo, Option<f64>)>> {
        let entities = UserRepo::find_nearby_user_list(lat, lng, limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("HOME PORT: 查询附近用户失败: {}", e))?;

        // 由于 UserEntity 没有 distance 字段，我们用 UserInfo 转换后，距离为 None
        // 后续如果有需要可以在 handler 中增加 distance 字段
        let result: Vec<(UserInfo, Option<f64>)> = entities
            .into_iter()
            .map(|e| (UserInfo::from(e), None))
            .collect();

        Ok(result)
    }
}
