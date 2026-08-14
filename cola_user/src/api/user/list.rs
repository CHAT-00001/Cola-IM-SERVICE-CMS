// user/src/api/user/list.rs
// core - USER - api - user - 列表 接口
// 2026/6/18 09:06 Created.
// 2026/8/6 对齐 case 层：最新/热门/推荐/同城/分类/角色/附近/搜索 8 个接口

////////

use crate::case::user::list::UserListCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_user::vo::user::UserVo;
use port::app::ctx::AppContext;
use tracing::{error, info};

////////

/// # [USER LIST API] - 用户列表接口
pub struct UserListApi;

impl UserListApi {
    ////////

    /// # 1. [API HANDLER] - 最新
    /// * `desc`: 获取最新注册用户列表
    pub async fn api_get_newest(
        _uid: i64,              // 操作者ID
        url: ApiGatewayRequest, // 网关请求
        ctx: &AppContext,       // 全局上下文
    ) -> AppData<Vec<UserVo>> {
        match UserListCase::case_get_newest_users(0, url.offset, url.limit, ctx).await {
            Ok(vos) => {
                info!("[🗣️ USER LIST API] - ✅️ 最新用户查询成功");
                AppData::ok(vos)
            }
            Err(e) => {
                error!("[🤐 USER LIST API] - ❌️ 最新用户查询失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, "最新用户查询失败", None)
            }
        }
    }

    ////////

    /// # 3. [API HANDLER] - 推荐
    /// * `desc`: 获取推荐用户列表
    pub async fn api_get_recommend(
        _uid: i64,              // 操作者ID
        url: ApiGatewayRequest, // 网关请求
        ctx: &AppContext,       // 全局上下文
    ) -> AppData<Vec<UserVo>> {
        match UserListCase::case_get_recommend_users(0, url.offset, url.limit, ctx).await {
            Ok(vos) => {
                info!("[🗣️ USER LIST API] - ✅️ 推荐用户查询成功");
                AppData::ok(vos)
            }
            Err(e) => {
                error!("[🤐 USER LIST API] - ❌️ 推荐用户查询失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, "推荐用户查询失败", None)
            }
        }
    }

    ////////

    /// # 4. [API HANDLER] - 同城
    /// * `desc`: 获取同城用户列表
    pub async fn api_get_city(
        _uid: i64,              // 操作者ID
        url: ApiGatewayRequest, // 网关请求
        ctx: &AppContext,       // 全局上下文
    ) -> AppData<Vec<UserVo>> {
        // 1.
        let city_id = url.id;

        // 2. Call .. CASE
        match UserListCase::case_get_city_users(0, city_id, url.offset, url.limit, ctx).await {
            Ok(vos) => {
                info!("[🗣️ USER LIST API] - ✅️ 同城用户查询成功");
                AppData::ok(vos)
            }
            Err(e) => {
                error!("[🤐 USER LIST API] - ❌️ 同城用户查询失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, "同城用户查询失败", None)
            }
        }
    }

    ////////

    /// # 5. [API HANDLER] - 分类
    /// * `desc`: 获取分类（频道）下的用户列表
    pub async fn api_get_category(
        _uid: i64,              // 操作者ID
        url: ApiGatewayRequest, // 网关请求
        ctx: &AppContext,       // 全局上下文
    ) -> AppData<Vec<UserVo>> {
        match UserListCase::case_get_category_users(0, url.category_id, url.offset, url.limit, ctx)
            .await
        {
            Ok(vos) => {
                info!("[🗣️ USER LIST API] - ✅️ 分类用户查询成功");
                AppData::ok(vos)
            }
            Err(e) => {
                error!("[🤐 USER LIST API] - ❌️ 分类用户查询失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, "分类用户查询失败", None)
            }
        }
    }

    ////////

    /// # 6. [API HANDLER] - 角色
    /// * `desc`: 获取指定角色的用户列表
    pub async fn api_get_role(
        _uid: i64,              // 操作者ID
        url: ApiGatewayRequest, // 网关请求
        ctx: &AppContext,       // 全局上下文
    ) -> AppData<Vec<UserVo>> {
        match UserListCase::case_get_role_users(0, url.id, url.offset, url.limit, ctx).await {
            Ok(vos) => {
                info!("[🗣️ USER LIST API] - ✅️ 角色用户查询成功");
                AppData::ok(vos)
            }
            Err(e) => {
                error!("[🤐 USER LIST API] - ❌️ 角色用户查询失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, "角色用户查询失败", None)
            }
        }
    }

    ////////

    /// # 7. [API HANDLER] - 附近
    /// * `desc`: 获取附近用户列表
    pub async fn api_get_nearby(
        _uid: i64,              // 操作者ID
        url: ApiGatewayRequest, // 网关请求
        ctx: &AppContext,       // 全局上下文
    ) -> AppData<Vec<UserVo>> {
        let lat = url.lat.unwrap_or(0.0);
        let lng = url.lng.unwrap_or(0.0);
        match UserListCase::case_get_nearby_users(0, lat, lng, url.offset, url.limit, ctx).await {
            Ok(vos) => {
                info!("[🗣️ USER LIST API] - ✅️ 附近用户查询成功");
                AppData::ok(vos)
            }
            Err(e) => {
                error!("[🤐 USER LIST API] - ❌️ 附近用户查询失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, "附近用户查询失败", None)
            }
        }
    }

    ////////

    /// # 8. [API HANDLER] - 搜索
    /// * `desc`: 根据关键词搜索用户
    pub async fn api_search(
        _uid: i64,              // 操作者ID
        url: ApiGatewayRequest, // 网关请求
        ctx: &AppContext,       // 全局上下文
    ) -> AppData<Vec<UserVo>> {
        let keyword = &url.keyword;
        let lat = url.lat.unwrap_or(0.0);
        let lng = url.lng.unwrap_or(0.0);
        match UserListCase::case_search_users(0, keyword, lat, lng, url.offset, url.limit, ctx)
            .await
        {
            Ok(vos) => {
                info!("[🗣️ USER LIST API] - ✅️ 搜索用户成功");
                AppData::ok(vos)
            }
            Err(e) => {
                error!("[🤐 USER LIST API] - ❌️ 搜索用户失败: {}", e);
                AppData::err(error::INTERNAL_ERROR, "搜索用户失败", None)
            }
        }
    }
}

//////// END
