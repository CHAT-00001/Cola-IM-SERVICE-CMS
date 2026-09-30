// basic/src/case/binding.rs -- BASIC - 用例层 - 应用绑定
// 2026/09/30 11:30 Created.

////////

use anyhow::Result;
use cola_data::app::page::ListResponse;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::basic::command::core::app::AppCreateCmd;
use cola_data::basic::info::core::app::AppInfo;
use port::app::ctx::AppContext;

////////

/// # [CASE] - 应用绑定
/// * `desc`: `当前阶段复用 App Port，将 app_id 与 App 内存储配置建立绑定`
pub struct BindingCase;

impl BindingCase {
    /// # 1. [CASE] - 创建应用绑定
    /// * `desc`: `通过 Basic App Port 创建应用绑定记录`
    pub async fn case_add_binding(
        _uid: i64,
        mut cmd: AppCreateCmd,
        ctx: &AppContext,
    ) -> Result<serde_json::Value> {
        cmd.complete_defaults();
        let entity = ctx.basic.app.add.create_app(cmd).await?;
        Ok(serde_json::to_value(entity)?)
    }

    /// # 2. [CASE] - 查询应用绑定
    /// * `desc`: `按 app_id 查询应用配置`
    pub async fn case_get_binding(app_id: String, ctx: &AppContext) -> Result<serde_json::Value> {
        let info = ctx
            .basic
            .app
            .get
            .get_app_by_app_id(&app_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("应用绑定不存在: {}", app_id))?;
        Ok(serde_json::to_value(info)?)
    }

    /// # 3. [CASE] - 分页查询应用绑定
    /// * `desc`: `复用 App Port 的分页查询`
    pub async fn case_get_binding_list(
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<ListResponse<AppInfo>> {
        let page = url.page.unwrap_or(1).max(1);
        let qty = url.qty.unwrap_or(10).clamp(1, 50);
        let offset = (page - 1) * qty;
        let app_id = url.params.get("app_id").map(String::as_str);
        let keyword = (!url.keyword.trim().is_empty()).then_some(url.keyword.as_str());
        let (list, total) = ctx
            .basic
            .app
            .list
            .admin_find_page(app_id, keyword, qty, offset)
            .await?;
        let size = list.len() as i64;
        Ok(ListResponse {
            list,
            page: Some(page),
            size: Some(size),
            qty: Some(qty),
            total: Some(total),
            has_more: Some(offset + size < total),
        })
    }

    /// # 4. [CASE] - 删除应用绑定
    /// * `desc`: `通过 App Port 执行应用记录逻辑删除`
    pub async fn case_delete_binding(
        _uid: i64,
        id: i64,
        ctx: &AppContext,
    ) -> Result<serde_json::Value> {
        let affected = ctx.basic.app.del.single_delete(id).await?;
        if affected == 0 {
            return Err(anyhow::anyhow!("应用绑定不存在: {}", id));
        }
        Ok(serde_json::json!({"id": id, "deleted": true}))
    }
}

//////// END
