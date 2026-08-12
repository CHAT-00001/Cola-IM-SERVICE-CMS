// cola_live/src/api/hotlist/add.rs
// core - LIVE - API - 上热门
// 2026/6/10 08:33

////////

use crate::case::hotlist::add::HotlistCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_video::command::hotlist::HotlistCommand;
use port::app::ctx::AppContext;

////////

/// # [API HANDLER] -  上热门 接口
pub struct HotlistApi;

// 构造函数
impl HotlistApi {
    //

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
}

//////// END
