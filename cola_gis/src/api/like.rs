// cola_gis/src/api/like.rs  -- 可乐GIS - 应用层 - 点赞
// 2026-07-07

//////

use crate::case::like::LikeCase;
use cola_data::app::data::AppData;
use cola_data::app::error;

//////

/// # [API] - 兴趣点 点赞
pub struct PoiLikeApi;

impl PoiLikeApi {
    ////////

    /// # 1. [API HANDLER] - 点赞
    pub async fn handler_add_poi_like(uid: i64, poi_id: i64, is_liked: bool) -> AppData<()> {
        match LikeCase::case_add_poi_like(uid, poi_id, is_liked).await {
            Ok(resp) => AppData::ok(resp).with_msg("点赞成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("点赞失败: {:?}", e), None),
        }
    }

    ////////

    /// # 2. [API HANDLER] - 不喜欢
    pub async fn handler_add_poi_unlike(uid: i64, poi_id: i64, is_like: bool) -> AppData<()> {
        match LikeCase::case_add_poi_unlike(uid, poi_id, is_like).await {
            Ok(resp) => AppData::ok(resp).with_msg("操作成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("操作失败: {:?}", e), None),
        }
    }
}

////// END
