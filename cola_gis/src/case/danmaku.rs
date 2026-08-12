// cola_gis/src/case/danmaku.rs  -- GIS - 用例层 - 弹幕
// 2026-07-07

////////

use crate::assembler::danmaku::build_danmaku_list_response;
use crate::model::vo::poi_danmaku::DanmakuListResponse;
use anyhow::Result;
use cola_data::cola_gis::command::danmaku::PoiDanmakuCommand;
use port::app::ctx::AppContext;
use repository::cola_gis::service::poi_danmaku::GisDanmakuService;

////////

/// # [CASE] - 弹幕 用例
pub struct DanmakuCase;

//////
impl DanmakuCase {
    //

    ////////

    /// # 1. [CASE] - 发表弹幕
    pub async fn case_add_danmaku(uid: i64, poi_id: i64, cmd: PoiDanmakuCommand) -> Result<String> {
        let visibility = 5;

        GisDanmakuService::save_danmaku_and_update_count(uid, poi_id, cmd, visibility).await?;

        Ok("ok".to_string())
    }

    ////////

    /// # 2. [CASE] - 浏览兴趣点弹幕列表
    pub async fn case_poi_danmakus(
        uid: i64,
        poi_id: i64,
        play_time: i32,
        _time_window: i32,
        qty: i64,
        ctx: &AppContext,
    ) -> Result<DanmakuListResponse> {
        let (danmaku_list, total) = ctx
            .gis
            .danmaku
            .get_danmaku_by_poi_id(uid, poi_id, play_time, qty as i32)
            .await?;

        let current_uid = if uid > 0 { Some(uid) } else { None };
        let video_author_id = 0i64;

        let response =
            build_danmaku_list_response(danmaku_list, current_uid, video_author_id, 1, qty, total)
                .await?;

        Ok(response)
    }

    ////////
}

//////// END
