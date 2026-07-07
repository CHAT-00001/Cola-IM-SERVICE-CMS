// cola_gis/src/api/hotlist.rs  -- 可乐GIS - 接口层 - 上热门
// 2026/6/10 08:33

////////

use cola_data::app::ctx::AppContext;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::gis::command::hotlist::HotlistCommand;
use cola_data::gis::command::report::PoiReportCommand;
use crate::case::hotlist::HotlistCase;

////////

/// # [API HANDLER] -  上热门 接口
pub struct HotlistApi;

// 构造函数
impl HotlistApi {
    ////////

    ////////

    /// # 1. [API HANDLER] -  送上热门
    pub async fn handler_add_hotlist(
        uid: i64,
        url: ApiGatewayRequest,
        cmd: HotlistCommand,
        ctx: &AppContext,
    ) -> AppData<String> {

        // Call Case
        match HotlistCase::case_add_hotlist(uid, url, cmd, ctx).await {
            Ok(_) => AppData::ok("上热门成功".to_string()).with_msg("上热门成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("上热门失败: {:?}", e), None),
        }
    }

    ////////

    // /// # 2. [API HANDLER] - 获取规则
    // pub async fn handler_get_rule(
    //     user_id: i64,
    //     query: i64,
    //     ctx: &AppContext,
    // ) -> AppData<String> {
    //
    //
    //     // Call Case
    //     match HotlistCase::case_view_hotlist_rule(user_id, query, ctx).await {
    //         Ok(_) => AppData::ok("获取上热门规则成功".to_string()).with_msg("获取上热门规则成功"),
    //         Err(e) => AppData::err(error::INTERNAL_ERROR, format!("获取上热门规则失败: {:?}", e), None),
    //     }
    // }
}

//////// END
