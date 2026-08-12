// cola_gis/src/api/danmaku.rs
// core - 可乐GIS - 接口层 - 弹幕
// 2026-07-07 10:50 Created.

////////

use crate::case::danmaku::DanmakuCase;
use crate::model::vo::poi_danmaku::DanmakuListResponse;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::cola_auth::info::auth::AuthContext;
use cola_data::cola_gis::command::danmaku::PoiDanmakuCommand;
use port::app::ctx::AppContext;

////////

/// # [API] - 弹幕 接口
pub struct DanmakuApi;

//
impl DanmakuApi {
    //

    ////////

    /// # 1. [API HANDLER] - 发布弹幕
    pub async fn handler_add_danmaku(
        auth: AuthContext,
        poi_id: i64,
        cmd: PoiDanmakuCommand,
    ) -> AppData<String> {
        match DanmakuCase::case_add_danmaku(auth.uid, poi_id, cmd).await {
            Ok(_) => AppData::ok("发布弹幕成功".to_string()).with_msg("发布成功"),
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("发布弹幕失败: {:?}", e),
                None,
            ),
        }
    }

    ////////

    /// # 2. [API HANDLER] - 获取弹幕列表
    pub async fn handler_get_danmaku(
        uid: i64,
        poi_id: i64,
        play_time: i32,
        ctx: &AppContext,
    ) -> AppData<DanmakuListResponse> {
        let segment_size = 5000;

        match DanmakuCase::case_poi_danmakus(
            uid,
            poi_id,
            play_time,
            play_time + segment_size,
            20,
            ctx,
        )
        .await
        {
            Ok(list) => AppData::ok(list),
            Err(e) => {
                tracing::error!("Get Danmaku Error: {:?}", e);
                AppData::err(5002, "获取弹幕列表失败", None)
            }
        }
    }
}

////// END
