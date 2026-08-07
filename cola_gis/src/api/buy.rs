// cola_gis/src/api/add  -- 可乐GIS - 接口层 - 购买
// 2026-07-07

//////

use cola_data::app::ctx::AppContext;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::cola_gis::command::buy::PoiBuyCommand;
use crate::case::buy::BuyCase;

//////

/// # [APP USE CASE] - 购买接口
pub struct BuyApi;

impl BuyApi {

    ////////

    /// # 1. [API HANDLER] - 添加 - 购买
    pub async fn handler_add_buy(
        user_id: i64,
        poi_id: i64,
        cmd: PoiBuyCommand,
        ctx: &AppContext,
    ) -> AppData<String> {
        match BuyCase::case_add_poi_buy(user_id, poi_id, cmd, ctx).await {
            Ok(_) => AppData::ok("POI购买成功".to_string()).with_msg("POI购买成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("POI购买失败: {:?}", e), None),
        }
    }
}

//////