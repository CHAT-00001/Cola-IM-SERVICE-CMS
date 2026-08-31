// cola_live/src/api/category/add.rs
// LIVE - api - 分类 - 管理接口
// 2026/8/20 21:20 Created.

////////

use crate::case::category::manage::LiveCategoryManageCase;
use cola_data::app::data::AppData;
use cola_data::auth::info::auth::AuthContext;
use cola_data::cola_live::command::class::LiveClassCommand;
use port::app::ctx::AppContext;
use serde_json::Value;
use tracing::{error, info};

////////

/// # 1. [API HANDLER] - 直播分类管理
pub struct LiveCateAddApi;

impl LiveCateAddApi {
    fn verify_admin(auth: &AuthContext) -> Result<(), AppData<Value>> {
        if auth.uid <= 0 {
            return Err(AppData::err(4001, "请先登录", None));
        }
        let allowed = auth.iam_roles.iter().any(|role| {
            matches!(
                role.to_ascii_lowercase().as_str(),
                "admin" | "administrator" | "operator" | "运营" | "管理员"
            )
        });
        if !allowed {
            return Err(AppData::err(4030, "仅管理员或运营人员可管理直播分类", None));
        }
        Ok(())
    }

    /// # 1. [API HANDLER] - 创建直播分类
    pub async fn api_add_cate(
        auth: AuthContext,
        command: LiveClassCommand,
        ctx: &AppContext,
    ) -> AppData<Value> {
        if let Err(response) = Self::verify_admin(&auth) {
            return response;
        }
        match LiveCategoryManageCase::create(auth.uid, command, ctx).await {
            Ok(info) => {
                info!("[🗣️ API] - ✅️ 创建直播分类成功: uid={}", auth.uid);
                AppData::ok(serde_json::to_value(info).unwrap_or_default())
            }
            Err(err) => {
                error!("[🤐 API] - ❌️ 创建直播分类失败: {}", err);
                AppData::err(5000, "创建直播分类失败", Some(err.to_string()))
            }
        }
    }

    /// # 2. [API HANDLER] - 修改直播分类
    pub async fn api_edit_cate(
        auth: AuthContext,
        command: LiveClassCommand,
        ctx: &AppContext,
    ) -> AppData<Value> {
        if let Err(response) = Self::verify_admin(&auth) {
            return response;
        }
        match LiveCategoryManageCase::edit(auth.uid, command, ctx).await {
            Ok(info) => AppData::ok(serde_json::to_value(info).unwrap_or_default()),
            Err(err) => {
                error!("[🤐 API] - ❌️ 修改直播分类失败: {}", err);
                AppData::err(5000, "修改直播分类失败", Some(err.to_string()))
            }
        }
    }

    /// # 3. [API HANDLER] - 启用/禁用直播分类
    pub async fn api_change_status(
        auth: AuthContext,
        id: i64,
        status: i16,
        ctx: &AppContext,
    ) -> AppData<Value> {
        if let Err(response) = Self::verify_admin(&auth) {
            return response;
        }
        match LiveCategoryManageCase::change_status(auth.uid, id, status, ctx).await {
            Ok(info) => AppData::ok(serde_json::to_value(info).unwrap_or_default()),
            Err(err) => {
                error!("[🤐 API] - ❌️ 修改直播分类状态失败: {}", err);
                AppData::err(5000, "修改直播分类状态失败", Some(err.to_string()))
            }
        }
    }

    /// # 4. [API HANDLER] - 删除直播分类
    pub async fn api_delete_cate(auth: AuthContext, id: i64, ctx: &AppContext) -> AppData<Value> {
        if let Err(response) = Self::verify_admin(&auth) {
            return response;
        }
        match LiveCategoryManageCase::delete(auth.uid, id, ctx).await {
            Ok(affected) if affected > 0 => {
                AppData::ok(serde_json::json!({"id": id, "deleted": true}))
            }
            Ok(_) => AppData::err(4004, "直播分类不存在", None),
            Err(err) => {
                error!("[🤐 API] - ❌️ 删除直播分类失败: {}", err);
                AppData::err(5000, "删除直播分类失败", Some(err.to_string()))
            }
        }
    }

    /// # 5. [API HANDLER] - 查询直播分类列表
    pub async fn api_list(
        status: Option<i16>,
        limit: i64,
        offset: i64,
        ctx: &AppContext,
    ) -> AppData<Value> {
        match LiveCategoryManageCase::list(status, limit, offset, ctx).await {
            Ok(list) => AppData::ok(serde_json::to_value(list).unwrap_or_default()),
            Err(err) => {
                error!("[🤐 API] - ❌️ 查询直播分类失败: {}", err);
                AppData::err(5000, "查询直播分类失败", Some(err.to_string()))
            }
        }
    }
}

//////// END
