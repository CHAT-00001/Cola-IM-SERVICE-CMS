// cola_gis/src/api/count  -- 可乐GIS - 接口层 - 收藏
// 2026-07-07

//////

use crate::case::collect::CollectCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::cola_gis::command::collect::PoiCollectCommand;

//////

/// # [API] - 兴趣点 收藏 接口
pub struct PoiCollectApi;

impl PoiCollectApi {
    ////////

    /// # 1. [HANDLER] - 添加收藏
    pub async fn handler_add_collect(
        user_id: i64,
        poi_id: i64,
        cmd: PoiCollectCommand,
    ) -> AppData<String> {
        match CollectCase::case_add_collect(user_id, poi_id, cmd).await {
            Ok(_) => AppData::ok("收藏成功".to_string()).with_msg("收藏成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("收藏失败: {:?}", e), None),
        }
    }

    ////////

    /// # 2. [HANDLER] - 删除收藏
    pub async fn handler_del_collect(user_id: i64, poi_id: i64) -> AppData<String> {
        match CollectCase::case_del_collect(user_id, poi_id).await {
            Ok(_) => AppData::ok("删除成功".to_string()).with_msg("删除成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("删除失败: {:?}", e), None),
        }
    }
}
