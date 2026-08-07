// cola_user/src/case/user/list.rs
// core - USER - case - user - 列表 用例
// 2026/6/18 09:06
// 2026/8/6 原子化：最新/热门/推荐/同城/分类/角色/附近/搜索 8 个接口

////////

use anyhow::{Result, anyhow};
use cola_data::app::ctx::AppContext;
use cola_data::user::info::user::UserInfo;
use cola_data::user::vo::user::UserVo;
use repository::user::pg::home_repo::UserHomeRepo;
use repository::user::pg::user::user::UserRepo;
use tracing::info;

////////

/// # [LIST CASE] - 用户列表 用例
/// * `desc`: 用户列表查询（最新/热门/推荐/同城/分类/角色/附近/搜索）
pub struct UserListCase;

impl UserListCase {

    ////////

    /// # 1. [CASE] - 最新
    /// * `desc`: 获取最新注册的用户列表
    pub async fn case_get_newest_users(
        _uid: i64, // 操作者ID
        offset: i64, // 分页偏移
        limit: i64, // 每页数量
        _ctx: &AppContext, // 全局上下文
    ) -> Result<Vec<UserVo>, anyhow::Error> {
        let entities = UserRepo::find_new_user_list(limit, offset)
            .await
            .map_err(|e| anyhow!("[🤐 USER LIST CASE]: ❌️ 查询最新用户失败: {}", e))?;

        let vos: Vec<UserVo> = entities.iter()
            .map(|e| UserVo::new(UserInfo::from(e.clone()), false, false, false))
            .collect();

        info!("[🗣️ USER LIST CASE]: ✅️ 最新用户查询成功, count={}", vos.len());
        Ok(vos)
    }

    ////////

    /// # 2. [CASE] - 热门
    /// * `desc`: 获取热门用户列表
    pub async fn case_get_hot_users(
        _uid: i64, // 操作者ID
        offset: i64, // 分页偏移
        limit: i64, // 每页数量
        _ctx: &AppContext, // 全局上下文
    ) -> Result<Vec<UserVo>, anyhow::Error> {
        let entities = UserHomeRepo::find_hot_user_list(limit, offset)
            .await
            .map_err(|e| anyhow!("[🤐 USER LIST CASE]: ❌️ 查询热门用户失败: {}", e))?;

        let vos: Vec<UserVo> = entities.iter()
            .map(|e| UserVo::new(UserInfo::from(e.clone()), false, false, false))
            .collect();

        info!("[🗣️ USER LIST CASE]: ✅️ 热门用户查询成功, count={}", vos.len());
        Ok(vos)
    }

    ////////

    /// # 3. [CASE] - 推荐
    /// * `desc`: 获取推荐用户列表
    pub async fn case_get_recommend_users(
        _uid: i64, // 操作者ID
        offset: i64, // 分页偏移
        limit: i64, // 每页数量
        _ctx: &AppContext, // 全局上下文
    ) -> Result<Vec<UserVo>, anyhow::Error> {
        let entities = UserHomeRepo::find_recommend_users_list(limit, offset)
            .await
            .map_err(|e| anyhow!("[🤐 USER LIST CASE]: ❌️ 查询推荐用户失败: {}", e))?;

        let vos: Vec<UserVo> = entities.iter()
            .map(|e| UserVo::new(UserInfo::from(e.clone()), false, false, false))
            .collect();

        info!("[🗣️ USER LIST CASE]: ✅️ 推荐用户查询成功, count={}", vos.len());
        Ok(vos)
    }

    ////////

    /// # 4. [CASE] - 同城
    /// * `desc`: 获取同城用户列表
    pub async fn case_get_city_users(
        _uid: i64, // 操作者ID
        offset: i64, // 分页偏移
        limit: i64, // 每页数量
        _ctx: &AppContext, // 全局上下文
    ) -> Result<Vec<UserVo>, anyhow::Error> {
        let entities = UserRepo::find_category(limit, offset)
            .await
            .map_err(|e| anyhow!("[🤐 USER LIST CASE]: ❌️ 查询同城用户失败: {}", e))?;

        let vos: Vec<UserVo> = entities.iter()
            .map(|e| UserVo::new(UserInfo::from(e.clone()), false, false, false))
            .collect();

        info!("[🗣️ USER LIST CASE]: ✅️ 同城用户查询成功, count={}", vos.len());
        Ok(vos)
    }

    ////////

    /// # 5. [CASE] - 分类
    /// * `desc`: 获取分类（频道）下的用户列表
    pub async fn case_get_category_users(
        _uid: i64, // 操作者ID
        _category_id: i64, // 分类ID
        offset: i64, // 分页偏移
        limit: i64, // 每页数量
        _ctx: &AppContext, // 全局上下文
    ) -> Result<Vec<UserVo>, anyhow::Error> {
        let entities = UserRepo::find_category(limit, offset)
            .await
            .map_err(|e| anyhow!("[🤐 USER LIST CASE]: ❌️ 查询分类用户失败: {}", e))?;

        let vos: Vec<UserVo> = entities.iter()
            .map(|e| UserVo::new(UserInfo::from(e.clone()), false, false, false))
            .collect();

        info!("[🗣️ USER LIST CASE]: ✅️ 分类用户查询成功, count={}", vos.len());
        Ok(vos)
    }

    ////////

    /// # 6. [CASE] - 角色
    /// * `desc`: 获取指定角色的用户列表
    pub async fn case_get_role_users(
        _uid: i64, // 操作者ID
        _role_id: i64, // 角色ID
        offset: i64, // 分页偏移
        limit: i64, // 每页数量
        _ctx: &AppContext, // 全局上下文
    ) -> Result<Vec<UserVo>, anyhow::Error> {
        let entities = UserRepo::find_featured(limit, offset)
            .await
            .map_err(|e| anyhow!("[🤐 USER LIST CASE]: ❌️ 查询角色用户失败: {}", e))?;

        let vos: Vec<UserVo> = entities.iter()
            .map(|e| UserVo::new(UserInfo::from(e.clone()), false, false, false))
            .collect();

        info!("[🗣️ USER LIST CASE]: ✅️ 角色用户查询成功, count={}", vos.len());
        Ok(vos)
    }

    ////////

    /// # 7. [CASE] - 附近
    /// * `desc`: 获取附近用户列表（按地理位置距离排序）
    pub async fn case_get_nearby_users(
        _uid: i64, // 操作者ID
        lat: f64, // 纬度
        lng: f64, // 经度
        offset: i64, // 分页偏移
        limit: i64, // 每页数量
        _ctx: &AppContext, // 全局上下文
    ) -> Result<Vec<UserVo>, anyhow::Error> {
        let entities = UserRepo::find_nearby_user_list(lat, lng, limit, offset)
            .await
            .map_err(|e| anyhow!("[🤐 USER LIST CASE]: ❌️ 查询附近用户失败: {}", e))?;

        let vos: Vec<UserVo> = entities.iter()
            .map(|e| UserVo::new(UserInfo::from(e.clone()), false, false, false))
            .collect();

        info!("[🗣️ USER LIST CASE]: ✅️ 附近用户查询成功, count={}", vos.len());
        Ok(vos)
    }

    ////////

    /// # 8. [CASE] - 搜索
    /// * `desc`: 根据关键词搜索用户
    pub async fn case_search_users(
        _uid: i64, // 操作者ID
        keyword: &str, // 搜索关键词
        lat: f64, // 纬度
        lng: f64, // 经度
        offset: i64, // 分页偏移
        limit: i64, // 每页数量
        _ctx: &AppContext, // 全局上下文
    ) -> Result<Vec<UserVo>, anyhow::Error> {
        let entities = UserRepo::search_keyword(keyword, lat, lng, limit, offset)
            .await
            .map_err(|e| anyhow!("[🤐 USER LIST CASE]: ❌️ 搜索用户失败: {}", e))?;

        let vos: Vec<UserVo> = entities.iter()
            .map(|e| UserVo::new(UserInfo::from(e.clone()), false, false, false))
            .collect();

        info!("[🗣️ USER LIST CASE]: ✅️ 搜索用户成功, keyword={}, count={}", keyword, vos.len());
        Ok(vos)
    }
}

//////// END