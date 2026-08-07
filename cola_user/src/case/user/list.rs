// cola_user/src/case/cola_user/list.rs
// core - 🗣 可乐用户 - case - cola_user - 列表 用例
// 2026/6/18 09:06

////////

use anyhow::{Result, anyhow};
use cola_data::app::ctx::AppContext;
use cola_data::cola_user::info::user::UserInfo;
use cola_data::cola_user::vo::user::UserVo;
use service::cola_user::user::list::UserListService;
use tracing::info;

////////

/// # [LIST CASE] - 用户列表 用例
/// * `desc`: `用户前台列表`
pub struct UserListCase;

impl UserListCase {
    //

    ////////

    /// # 1. [CASE] - 最新
    /// * `desc`: 获取最新注册的用户列表
    pub async fn case_get_newest_users(
        _uid: i64,         // 操作者ID
        offset: i64,       // 分页偏移
        limit: i64,        // 每页数量
        _ctx: &AppContext, // 全局上下文
    ) -> Result<Vec<UserVo>, anyhow::Error> {
        let entities = UserListService::get_new_user_list(limit, offset)
            .await
            .map_err(|e| anyhow!("[🤐 USER LIST CASE]: ❌️ 查询最新用户失败: {}", e))?;

        let vos: Vec<UserVo> = entities
            .iter()
            .map(|e| UserVo::new(UserInfo::from(e.clone()), false, false, false))
            .collect();

        info!(
            "[🗣️ USER LIST CASE]: ✅️ 最新用户查询成功, count={}",
            vos.len()
        );
        Ok(vos)
    }

    ////////

    /// # 3. [CASE] - 推荐
    /// * `desc`: 获取推荐用户列表
    pub async fn case_get_recommend_users(
        _uid: i64,         // 操作者ID
        offset: i64,       // 分页偏移
        limit: i64,        // 每页数量
        _ctx: &AppContext, // 全局上下文
    ) -> Result<Vec<UserVo>, anyhow::Error> {
        // Call ..
        let entities = UserListService::get_featured_user_list(limit, offset)
            .await
            .map_err(|e| anyhow!("[🤐 USER LIST CASE]: ❌️ 查询推荐用户失败: {}", e))?;

        let vos: Vec<UserVo> = entities
            .iter()
            .map(|e| UserVo::new(UserInfo::from(e.clone()), false, false, false))
            .collect();

        info!(
            "[🗣️ USER LIST CASE]: ✅️ 推荐用户查询成功, count={}",
            vos.len()
        );
        Ok(vos)
    }

    ////////

    /// # 4. [CASE] - 同城(💡 2026-8-8未完成)
    /// * `desc`: `获取GEO附近用户列表`
    pub async fn case_get_city_users(
        _uid: i64,         // 操作者ID
        city_id: i64,          // 分类
        offset: i64,       // 分页偏移
        limit: i64,        // 每页数量
        _ctx: &AppContext, // 全局上下文
    ) -> Result<Vec<UserVo>, anyhow::Error> {
        // Call ..
        let entities = UserListService::get_city_user_list(city_id, limit, offset)
            .await
            .map_err(|e| anyhow!("[🤐 USER LIST CASE]: ❌️ 查询同城用户失败: {}", e))?;

        let vos: Vec<UserVo> = entities
            .iter()
            .map(|e| UserVo::new(UserInfo::from(e.clone()), false, false, false))
            .collect();

        info!(
            "[🗣️ USER LIST CASE]: ✅️ 同城用户查询成功, count={}",
            vos.len()
        );
        Ok(vos)
    }

    ////////

    /// # 5. [CASE] - 分类
    /// * `desc`: 获取分类（频道）下的用户列表
    pub async fn case_get_category_users(
        _uid: i64,         // 操作者ID
        _category_id: i64, // 分类ID
        offset: i64,       // 分页偏移
        limit: i64,        // 每页数量
        _ctx: &AppContext, // 全局上下文
    ) -> Result<Vec<UserVo>, anyhow::Error> {
        let entities = UserListService::get_category_user_list(limit, offset)
            .await
            .map_err(|e| anyhow!("[🤐 USER LIST CASE]: ❌️ 查询分类用户失败: {}", e))?;

        let vos: Vec<UserVo> = entities
            .iter()
            .map(|e| UserVo::new(UserInfo::from(e.clone()), false, false, false))
            .collect();

        info!(
            "[🗣️ USER LIST CASE]: ✅️ 分类用户查询成功, count={}",
            vos.len()
        );
        Ok(vos)
    }

    ////////

    /// # 6. [CASE] - 角色
    /// * `desc`: 获取指定角色的用户列表
    pub async fn case_get_role_users(
        _uid: i64,         // 操作者ID
        _role_id: i64,     // 角色ID
        offset: i64,       // 分页偏移
        limit: i64,        // 每页数量
        _ctx: &AppContext, // 全局上下文
    ) -> Result<Vec<UserVo>, anyhow::Error> {
        let entities = UserListService::get_role_user_list(_role_id, limit, offset)
            .await
            .map_err(|e| anyhow!("[🤐 USER LIST CASE]: ❌️ 查询角色用户失败: {}", e))?;

        let vos: Vec<UserVo> = entities
            .iter()
            .map(|e| UserVo::new(UserInfo::from(e.clone()), false, false, false))
            .collect();

        info!(
            "[🗣️ USER LIST CASE]: ✅️ 角色用户查询成功, count={}",
            vos.len()
        );
        Ok(vos)
    }

    ////////

    /// # 7. [CASE] - 附近
    /// * `desc`: 获取附近用户列表（按地理位置距离排序）
    pub async fn case_get_nearby_users(
        _uid: i64,         // 操作者ID
        lat: f64,          // 纬度
        lng: f64,          // 经度
        offset: i64,       // 分页偏移
        limit: i64,        // 每页数量
        _ctx: &AppContext, // 全局上下文
    ) -> Result<Vec<UserVo>, anyhow::Error> {
        // Call ..
        let entities = UserListService::get_nearby_user_list(lat, lng, limit, offset)
            .await
            .map_err(|e| anyhow!("[🤐 USER LIST CASE]: ❌️ 查询附近用户失败: {}", e))?;

        let vos: Vec<UserVo> = entities
            .iter()
            .map(|e| UserVo::new(UserInfo::from(e.clone()), false, false, false))
            .collect();

        info!(
            "[🗣️ USER LIST CASE]: ✅️ 附近用户查询成功, count={}",
            vos.len()
        );
        Ok(vos)
    }

    ////////

    /// # 8. [CASE] - 搜索
    /// * `desc`: 根据关键词搜索用户
    pub async fn case_search_users(
        _uid: i64,         // 操作者ID
        keyword: &str,     // 搜索关键词
        lat: f64,          // 纬度
        lng: f64,          // 经度
        offset: i64,       // 分页偏移
        limit: i64,        // 每页数量
        _ctx: &AppContext, // 全局上下文
    ) -> Result<Vec<UserVo>, anyhow::Error> {
        // Call ..
        let entities = UserListService::search_user_keyword(keyword, lat, lng, limit, offset)
            .await
            .map_err(|e| anyhow!("[🤐 USER LIST CASE]: ❌️ 搜索用户失败: {}", e))?;

        let vos: Vec<UserVo> = entities
            .iter()
            .map(|e| UserVo::new(UserInfo::from(e.clone()), false, false, false))
            .collect();

        info!(
            "[🗣️ USER LIST CASE]: ✅️ 搜索用户成功, keyword={}, count={}",
            keyword,
            vos.len()
        );
        Ok(vos)
    }
}

//////// END
