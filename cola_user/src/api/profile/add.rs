// cola_user/src/api/profile/add.rs
// core - USER - api - profile - 资料名片 接口
// 2026/8/4 01:02 Created.

////////

use crate::case::profile::add::UserProfileAddCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_user::command::profile::ProfileCommand;
use port::ctx::AppContext;
use serde_json::Value;
use tracing::{error, info};

////////

/// # [PROFILE API] - 资料名片 接口
pub struct ProfileAddApi;

impl ProfileAddApi {
    //

    ////////

    /// # 1. [API HANDLER] - 创建/更新资料名片
    /// * `desc`: 强社交资料模型，独立的资料名片
    pub async fn api_upsert_profile(
        _uid: i64,                 // 操作者ID
        _query: ApiGatewayRequest, // 网关请求
        cmd: ProfileCommand,       // 资料名片命令
        ctx: &AppContext,          // 全局上下文
    ) -> AppData<Value> {
        match UserProfileAddCase::case_upsert_profile(cmd, ctx).await {
            Ok(info) => {
                info!("[🗣️ PROFILE API] - ✅️ 资料名片保存成功!");
                AppData::ok(serde_json::to_value(info).unwrap_or_default())
                    .with_msg("资料名片保存成功")
            }
            Err(e) => {
                error!("[🤐 PROFILE API] - ❌️ 资料名片保存失败!");
                AppData::err(
                    error::INTERNAL_ERROR,
                    format!("资料名片保存失败: {:?}", e),
                    None,
                )
            }
        }
    }

    ////////

    /// # 2. [API HANDLER] - 获取资料名片
    /// * `desc`: 根据用户ID查询资料名片
    pub async fn api_get_profile(
        _uid: i64,                // 操作者ID
        query: ApiGatewayRequest, // 网关请求
        ctx: &AppContext,         // 全局上下文
    ) -> AppData<Value> {
        match UserProfileAddCase::case_get_profile(query.id, ctx).await {
            Ok(Some(info)) => {
                info!("[🗣️ PROFILE API] - ✅️ 资料名片查询成功!");
                AppData::ok(serde_json::to_value(info).unwrap_or_default())
                    .with_msg("资料名片查询成功")
            }
            Ok(None) => {
                info!("[🗣️ PROFILE API] - ✅️ 资料名片不存在");
                AppData::ok(serde_json::Value::Null).with_msg("资料名片不存在")
            }
            Err(e) => {
                error!("[🤐 PROFILE API] - ❌️ 资料名片查询失败!");
                AppData::err(
                    error::INTERNAL_ERROR,
                    format!("资料名片查询失败: {:?}", e),
                    None,
                )
            }
        }
    }
}

//////// END
