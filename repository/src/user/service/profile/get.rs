// repository/src/user/service/profile/get.rs
// 仓储 - USER - service - profile - 获取
// 2026/8/3 14:32 Created.

////////

use crate::user::pg::profile::get::UserProfileGetRepo;
use cola_data::user::entity::profile::UserProfileEntity;

////////

/// # [GET SERVICE] - 用户 资料 前台 服务
pub struct UserProfileGetService;

// 构造函数
impl UserProfileGetService {
    //

    ////////

    /// # 1. [SERVICE] - 查找最新的资料列表
    pub async fn get_new_profile_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserProfileEntity>, anyhow::Error> {
        UserProfileGetRepo::pg_find_new_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[👤 SERVICE]: ❌️ 获取最新资料列表失败: {}", e))
    }

    ////////

    /// # 2. [SERVICE] - 查找热门的资料列表
    pub async fn get_hot_profile_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserProfileEntity>, anyhow::Error> {
        UserProfileGetRepo::pg_find_hot_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[👤 SERVICE]: ❌️ 获取热门资料列表失败: {}", e))
    }

    ////////

    /// # 3. [SERVICE] - 查找推荐的资料列表
    pub async fn get_recommend_profile_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserProfileEntity>, anyhow::Error> {
        UserProfileGetRepo::pg_find_recommend_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[👤 SERVICE]: ❌️ 获取推荐资料列表失败: {}", e))
    }

    ////////

    /// # 4. [SERVICE] - 查找同城的资料列表
    pub async fn get_city_profile_list(
        city_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserProfileEntity>, anyhow::Error> {
        UserProfileGetRepo::pg_find_city_list(city_id, limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[👤 SERVICE]: ❌️ 获取同城资料列表失败: {}", e))
    }

    ////////

    /// # 5. [SERVICE] - 搜索资料列表
    pub async fn get_keyword_profile_list(
        keyword: &String,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserProfileEntity>, anyhow::Error> {
        UserProfileGetRepo::pg_find_keyword_list(keyword, limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[👤 SERVICE]: ❌️ 搜索资料列表失败: {}", e))
    }
}

//////// END