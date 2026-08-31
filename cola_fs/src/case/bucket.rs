// cola_fs/src/case/bucket.rs -- FS - 用例层 - 存储桶
// 2026/8/14 14:00 Created.

////////

use anyhow::Result;
use cola_data::app::page::ListResponse;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_fs::command::bucket::CreateBucketCmd;
use cola_data::cola_fs::info::bucket::BucketInfo;
use port::app::ctx::AppContext;
use tracing::info;

////////

pub struct FsBucketCase;

impl FsBucketCase {
    //

    ////////

    /// # 1. [CASE] - 创建存储桶
    /// * `desc`: `业务编排 - 调用 ctx 的 trait 实现`
    pub async fn case_add_bucket(
        _uid: i64,
        cmd: CreateBucketCmd,
        ctx: &AppContext,
    ) -> Result<serde_json::Value> {
        let mut cmd = cmd;
        cmd.complete_defaults();

        let bucket_entity = ctx.fs.bucket.add.create_bucket(cmd).await?;

        info!(
            "[🗣️ CASE] - ✅️ 存储桶创建成功: bucket_id={}",
            bucket_entity.id
        );

        Ok(serde_json::to_value(&bucket_entity)?)
    }

    ////////

    /// # 2. [CASE] - 查询存储桶
    /// * `desc`: `业务编排 - 按 app_id 查询`
    pub async fn case_get_bucket(app_id: String, ctx: &AppContext) -> Result<serde_json::Value> {
        // 1. 调用 adapter 查询存储桶
        let bucket_entity = ctx
            .fs
            .bucket
            .get
            .get_bucket_by_app_id(&app_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("存储桶不存在: {}", app_id))?;

        info!("[🗣️ CASE] - ✅️ 存储桶查询成功: app_id={}", app_id);

        Ok(serde_json::to_value(&bucket_entity)?)
    }

    ////////

    /// # 3. [CASE] - 管理员分页查询存储桶
    /// * `desc`: `使用 page/qty 计算 limit/offset，返回总数`
    pub async fn case_get_bucket_list(
        url: ApiGatewayRequest, // 网关请求参数
        ctx: &AppContext,       // 全局上下文
    ) -> Result<ListResponse<BucketInfo>> {
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
            .fs
            .bucket
            .list
            .admin_find_page(app_id, keyword, qty, offset)
            .await?;

        let list_size = list.len() as i64;
        let has_more = offset + list_size < total;
        info!(
            "[🗣️ CASE] - ✅️ 管理员存储桶列表查询成功: page={}, qty={}, count={}, total={}",
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
