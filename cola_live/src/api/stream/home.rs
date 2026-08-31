// cola_live/src/api/stream/home.rs
// LIVE - API - 直播场次 - 前台首页
// 2026/8/21 09:44 Created.

////////

use cola_data::app::data::AppData;
use cola_data::app::page::PageInfo;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_live::vo::record::{LiveRecordListVo, LiveRecordVo};
use port::app::ctx::AppContext;
use tracing::{error, info};

////////

/// # 1. [API HANDLER] - 直播首页列表
pub struct LiveStreamHomeApi;

impl LiveStreamHomeApi {
    //

    ////////

    /// # 1. [API HANDLER] - 最新直播
    pub async fn newest(
        url: ApiGatewayRequest, // 网关请求参数
        ctx: &AppContext,       // 应用上下文
    ) -> AppData<LiveRecordListVo> {
        info!(
            "[🗣️ API] - 📡 请求最新直播列表: page={:?}, qty={:?}",
            url.page, url.qty
        );
        Self::list(None, "new", url, ctx).await
    }

    ////////

    /// # 2. [API HANDLER] - 分类直播
    pub async fn category(
        url: ApiGatewayRequest, // 网关请求参数
        ctx: &AppContext,       // 应用上下文
    ) -> AppData<LiveRecordListVo> {
        let category_id = url.category_id;
        info!(
            "[🗣️ API] - 📡 请求分类直播列表: category_id={}, page={}, qty={}",
            category_id,
            url.page.unwrap_or(1),
            url.qty.unwrap_or(10)
        );
        if category_id <= 0 {
            error!("[🤐 API] - ❌️ 非法直播分类 ID: {}", category_id);
            return AppData::err(4002, "非法的直播分类ID", None);
        }
        Self::list(Some(category_id), "new", url, ctx).await
    }

    ////////

    /// # 3. [API HANDLER] - 热门直播
    pub async fn hot(
        url: ApiGatewayRequest, // 网关请求参数
        ctx: &AppContext,       // 应用上下文
    ) -> AppData<LiveRecordListVo> {
        info!(
            "[🗣️ API] - 📡 请求热门直播列表: page={:?}, qty={:?}",
            url.page, url.qty
        );
        Self::list(None, "hot", url, ctx).await
    }

    ////////

    /// # 4. [API HANDLER] - 管理员列表
    async fn list(
        filter: Option<i64>,
        order: &str,
        url: ApiGatewayRequest, // 网关请求参数
        ctx: &AppContext,       // 应用上下文
    ) -> AppData<LiveRecordListVo> {
        let page = url.page.unwrap_or(1);
        let qty = url.qty.unwrap_or(10);
        let limit = url.limit;
        let offset = url.offset;
        info!(
            "[🗣️ API] - 🔎 查询直播列表: filter={:?}, order={}, limit={}, offset={}",
            filter, order, limit, offset
        );
        let result = if order == "hot" {
            ctx.live.stream.list.hot(limit, offset).await
        } else if let Some(category_id) = filter {
            ctx.live
                .stream
                .list
                .category(category_id, limit, offset)
                .await
        } else {
            ctx.live.stream.list.newest(limit, offset).await
        };
        match result {
            Ok(infos) => {
                info!("[🗣️ API] - ✅️ 查询直播列表成功: count={}", infos.len());
                AppData::ok(LiveRecordListVo {
                    list: infos.into_iter().map(LiveRecordVo::from).collect(),
                    page_info: PageInfo {
                        page,
                        qty,
                        has_more: false,
                    },
                })
            }
            Err(err) => {
                error!(
                    "[🤐 API] - ❌️ 获取直播列表失败: filter={:?}, order={}, error={}",
                    filter, order, err
                );
                AppData::err(5000, "获取直播列表失败", Some(err.to_string()))
            }
        }
    }
}

//////// END
