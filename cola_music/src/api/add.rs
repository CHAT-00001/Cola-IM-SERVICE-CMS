// cola_music/src/api/active  -- 可乐 MUSIC - api - 添加
// 2026-07-08

//////

use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::music::command::music::MusicCommand;
use cola_data::music::vo::music_vo::MusicSingleResponse;
use crate::case::add::MusicAddCase;

//////

/// # [API] - 发布 接口
pub struct MusicAddApi;

impl MusicAddApi {

    ////////

    /// # 1. [API HANDLER] - 发布
    pub async fn handler_add(
        user_id: i64,
        cmd: MusicCommand,
    ) -> AppData<MusicSingleResponse> {

        match MusicAddCase::case_add_publish(user_id, cmd).await {
            Ok(resp) => AppData::ok(resp).with_msg("发布成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("发布失败: {:?}", e), None),
        }
    }

    ////////

    /// # 2. [API HANDLER] - 编辑
    pub async fn handler_edit(
        user_id: i64,
        cmd: MusicCommand,
    ) -> AppData<MusicSingleResponse> {

        match MusicAddCase::case_edit_publish(user_id, cmd).await {
            Ok(resp) => AppData::ok(resp).with_msg("编辑成功"),
            Err(e) => AppData::err(error::INTERNAL_ERROR, format!("编辑失败: {:?}", e), None),
        }
    }
}

////// END