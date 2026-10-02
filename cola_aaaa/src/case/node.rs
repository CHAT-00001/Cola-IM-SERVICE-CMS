// cola_basic/src/case/node.rs -- BASIC - case - 节点用例
// 2026/8/11 04:41 Created.

////////

use anyhow::Result;
use cola_data::aaaa::command::core::node::{NodeCreateCmd, NodeUpdateCmd};
use port::app::ctx::AppContext;
use tracing::info;

////////

/// # [CASE] - 节点域名 用例
/// * `desc`: `边缘节点 用例`
pub struct NodeCase;

impl NodeCase {
    //

    ////////

    /// # 1. [CASE] - 创建 CDN 配置
    /// * `desc`: `调用 CDN 配置 Port 创建记录`
    pub async fn case_add_node(
        uid: i64,           // 操作者 ID
        cmd: NodeCreateCmd, // 节点创建命令
        ctx: &AppContext,   // 应用上下文
    ) -> Result<serde_json::Value> {
        ////////

        // 💡 - INFO - 创建节点
        let info = ctx.basic.node.add.create_node(uid, cmd).await?;

        ////////

        // 💡 - LOGGER
        info!("[🗣️ CASE] - ✅️ NODE节点创建成功: node_id={}", info.id);

        // ✅️ - Ok
        Ok(serde_json::to_value(info)?)
    }

    ////////

    /// # 2. [CASE] - 更新 CDN 配置
    /// * `desc`: `调用 CDN 配置 Port 更新记录`
    pub async fn case_update_node(
        uid: i64,           // 操作者 ID
        node_id: i64,       // CDN域名 ID
        cmd: NodeUpdateCmd, // 命令
        ctx: &AppContext,   // 应用上下文
    ) -> Result<serde_json::Value> {
        ////////

        // 💡 - INFO - 更新节点
        let info = ctx.basic.node.add.update_node(uid, node_id, cmd).await?;

        ////////

        // 💡 - LOGGER
        info!("[🗣️ CASE] - ✅️ NODE节点更新成功: node_id={}", info.id);

        // ✅️ - Ok
        Ok(serde_json::to_value(info)?)
    }

    ////////

    /// # 3. [CASE] - 更新 CDN 状态
    /// * `desc`: `调用 CDN 配置 Port 更新状态`
    pub async fn case_change_node_status(
        _uid: i64,
        node_id: i64,
        status: i16,
        ctx: &AppContext,
    ) -> Result<serde_json::Value> {
        let info = ctx.basic.node.config.update_status(node_id, status).await?;

        info!(
            "[🗣️ CASE] - ✅️ CDN状态更新成功: node_id={}, status={}",
            node_id, status
        );

        Ok(serde_json::to_value(info)?)
    }

    ////////

    /// # 4. [CASE] - 逻辑删除 CDN 配置
    /// * `desc`: `调用 CDN 配置 Port 删除记录`
    pub async fn case_delete_node(
        node_id: i64,     // CDN域名 ID
        ctx: &AppContext, // 全局上下文
    ) -> Result<serde_json::Value> {
        let affected = ctx.basic.node.config.delete(node_id).await?;
        if affected == 0 {
            return Err(anyhow::anyhow!("CDN域名不存在: {}", node_id));
        }

        info!("[🗣️ CASE] - ✅️ CDN域名删除成功: node_id={}", node_id);

        Ok(serde_json::json!({"node_id": node_id, "deleted": true}))
    }

    ////////

    /// # 6. [CASE] - 分页查询 Bucket CDN 列表
    /// * `desc`: `管理员视角，从 Bucket 列表读取 node_domain 和总数`
    pub async fn case_get_node_list(
        app_id: Option<String>, // 可选应用 ID
        limit: i64,             // 分页数量
        offset: i64,            // 分页偏移
        ctx: &AppContext,       // 全局上下文
    ) -> Result<serde_json::Value> {
        let (list, total) = ctx
            .basic
            .app
            .list
            .admin_find_page(app_id.as_deref(), None, limit, offset)
            .await?;

        info!(
            "[🗣️ CASE] - ✅️ CDN列表查询成功: app_id={:?}, count={}, total={}",
            app_id,
            list.len(),
            total
        );

        Ok(serde_json::json!({
            "list": list,
            "total": total,
            "limit": limit,
            "offset": offset
        }))
    }

    ////////

    /// # 7. [CASE] - 根据区域ID查找节点
    /// * `desc`: ``
    pub async fn case_get_node_by_region_id(
        region_id: i64,   // 区域 ID
        limit: i64,       // 数量
        offset: i64,      // 页码
        ctx: &AppContext, // 全局上下文
    ) -> Result<serde_json::Value> {
        ////////

        // 💡 - INFO - 节点信息
        let infos = ctx
            .basic
            .node
            .get
            .get_node_infos_by_region_id(region_id, limit, offset)
            .await?;

        if infos.is_empty() {
            return Err(anyhow::anyhow!("区域下暂无节点: {}", region_id));
        }

        ////////

        // 💡 - LOGGER - 打印日志
        info!(
            "[🗣️ NODE CASE] - ✅️ 区域 NODE 查询成功: region_id={}",
            region_id
        );

        Ok(serde_json::to_value(infos)?)
    }

    ////////

    /// # 8. [CASE] - 根据 CDN 配置 ID 查询
    /// * `desc`: `调用 CDN 配置 Port 按 ID 查询旧版 CDN 配置记录`
    pub async fn case_get_node_by_id(
        node_id: i64,     // 存储桶 ID
        ctx: &AppContext, // 全局上下文
    ) -> Result<serde_json::Value> {
        // 1. 调用 adapter 查询CDN域名
        let info = ctx
            .basic
            .node
            .config
            .find_by_id(node_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("CDN域名不存在: {}", node_id))?;

        info!("[🗣️ CASE] - ✅️ CDN域名查询成功: app_id={}", node_id);

        Ok(serde_json::to_value(info)?)
    }
}

//////// END
