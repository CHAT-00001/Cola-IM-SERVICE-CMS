// cola_video/api/hotlist/add.rs
// 视频 - api - 热门 - 发布
// 2026/8/4 18:51 Created.

////////

use cola_data::app::ctx::AppContext;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_video::command::hotlist::HotlistCommand;
use crate::case::hotlist::HotlistCase;

////////

/// # [API HANDLER] -  上热门
/// * `desc`: `视频上热门发布接口`
pub struct VideoHotlistAddApi;

// 构造函数
impl VideoHotlistAddApi {
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
