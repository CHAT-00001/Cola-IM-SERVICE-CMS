// basic/src/case/app.rs -- BASIC - 用例层 - 应用
// 2026/8/14 14:00 Created.

////////

use anyhow::Result;
use cola_data::app::page::ListResponse;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::basic::command::core::app::AppCreateCmd;
use cola_data::basic::info::core::app::AppInfo;
use port::app::ctx::AppContext;
use tracing::info;
////////

pub struct AppCase;

impl AppCase {
    //

    ////////

    /// # 1. [CASE] - 创建应用
    /// * `desc`: `业务编排 - 调用 ctx 的 trait 实现`
    pub async fn case_add_app(
        _uid: i64,
        cmd: AppCreateCmd,
        ctx: &AppContext,
    ) -> Result<serde_json::Value> {
        let mut cmd = cmd;
        cmd.complete_defaults();

        let app_entity = ctx.basic.app.add.create_app(cmd).await?;

        info!("[🗣️ CASE] - ✅️ 应用创建成功: app_id={}", app_entity.id);

        Ok(serde_json::to_value(&app_entity)?)
    }

    ////////

    /// # 2. [CASE] - 查询应用
    /// * `desc`: `业务编排 - 按 app_id 查询`
    pub async fn case_get_app(app_id: String, ctx: &AppContext) -> Result<serde_json::Value> {
        // 1. 调用 adapter 查询应用
        let app_entity = ctx
            .basic
            .app
            .get
            .get_app_by_app_id(&app_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("应用不存在: {}", app_id))?;

        info!("[🗣️ CASE] - ✅️ 应用查询成功: app_id={}", app_id);

        Ok(serde_json::to_value(&app_entity)?)
    }

    ////////

    /// # 3. [CASE] - 管理员分页查询应用
    /// * `desc`: `使用 page/qty 计算 limit/offset，返回总数`
    pub async fn case_get_app_list(
        url: ApiGatewayRequest, // 网关请求参数
        ctx: &AppContext,       // 全局上下文
    ) -> Result<ListResponse<AppInfo>> {
        let page = url.page.unwrap_or(1).max(1);
        let qty = url.qty.unwrap_or(10).clamp(1, 50);
        let offset = (page - 1) * qty;
        let app_id = url.params.get("app_id").map(String::as_str);
        let keyword = if url.keyword.trim().is_empty() {
            None
        } else {
            Some(url.keyword.as_str())
        };

        let (list, total) = ctx
            .basic
            .app
            .list
            .admin_find_page(app_id, keyword, qty, offset)
            .await?;

        let list_size = list.len() as i64;
        let has_more = offset + list_size < total;
        info!(
            "[🗣️ CASE] - ✅️ 管理员应用列表查询成功: page={}, qty={}, count={}, total={}",
            page, qty, list_size, total
        );

        Ok(ListResponse {
            list,
            page: Some(page),
            size: Some(list_size),
            qty: Some(qty),
            total: Some(total),
            has_more: Some(has_more),
        })
    }
}

//////// END
