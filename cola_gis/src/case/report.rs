// cola_gis/src/case/report.rs  -- 可乐GIS - 用例层 - 举报
// 2026/6/10 19:20

////////

use crate::assembler::poi::build_poi_list_response;
use crate::model::vo::poi::{PoiListResponse, PoiSingleResponse};
use anyhow::{Context, Result};
use cola_data::app::ctx::AppContext;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::app::request::ApiUrlParamsQuery;
use cola_data::gis::command::report::PoiReportCommand;
use futures_util::TryFutureExt;
use tracing::info;

////////

/// # [REPORT CASE] - 举报 用例
pub struct PoiReportCase;

// 构造函数
impl PoiReportCase {
    //

    ////////

    /// # 1. [CASE]] - 举报
    pub async fn case_add_report(
        uid: i64,
        url: ApiGatewayRequest,
        cmd: PoiReportCommand,
        ctx: &AppContext,
    ) -> Result<()> {
        ctx.gis
            .report
            .save_report_record(uid, url.poi_id, cmd)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 保存举报记录失败: {}", e))?;

        info!("BIZ - 保存举报记录成功: uid={}", uid);
        Ok(())
    }

    ////////

    /// # 2. [CASE] - 获取被举报的兴趣点列表
    pub async fn case_get_report_poi(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<PoiListResponse> {
        // Call Service: 解构元组，同时拿到总数 total 和 兴趣点 IDs 数组
        let (total, poi_ids) = ctx
            .gis
            .report
            .get_report_record_ids(url.offset, url.limit)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 获取举报记录ID失败: {}", e))?;

        // 如果没有被举报的兴趣点，直接返回默认的空响应
        if poi_ids.is_empty() {
            return Ok(PoiListResponse::default());
        }

        // Repo: 🌟 顺次升级！用 ids 批量拿到纯净的领域对象 PoiInfo 列表
        let poi_infos = ctx
            .gis
            .view
            .get_poi_list_by_ids(poi_ids)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 批量获取兴趣点信息失败: {}", e))?;

        // Transform: 🌟 完美对齐 5 参数装配器！
        // 传入拿到的总数 total，让前端的分页器完美生效
        let response = build_poi_list_response(
            poi_infos,  // 1. 兴趣点 Info 列表
            None,       // 2. uid (这里是后台管理或不需要操作者上下文，传 None)
            url.offset, // 3. 偏移量
            url.limit,  // 4. 每页限制
            total,      // 5. 🌟 真正的总条数
        )
        .await // 别忘了我们的装配器是 async 的
        .map_err(|e| anyhow::anyhow!("BIZ: 组装兴趣点列表响应体失败: {}", e))?;

        tracing::info!("DOMAIN: 获取被举报的兴趣点列表成功啦~! 总数: {}", total);

        Ok(response)
    }

    ////////
}

//////// END
