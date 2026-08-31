// cola_music/src/api/like/add.rs -- MUSIC - 接口层 - 点赞 - 发布接口
// 2026-07-08 14:52 Created.

////////

use crate::case::like::add::MusicLikeAddCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use port::app::ctx::AppContext;

////////

/// # [ADD API] - 发布 接口
/// * `desc`: `音乐点赞 - 发布接口`
pub struct MusicLikeAddApi;

impl MusicLikeAddApi {
    //

    ////////

    /// # 1. [API HANDLER] - 音乐点赞
    /// * `desc`: `用户点赞一个音乐，返回操作结果`
    pub async fn api_add_like(
        uid: i64,        // 操作者 ID
        music_id: i64,   // 音乐 ID
        ctx: AppContext, // 应用上下文
    ) -> AppData<bool> {
        // 调用 Case 层
        match MusicLikeAddCase::case_add_like(uid, music_id, ctx).await {
            Ok(()) => AppData::ok(true).with_msg("[🗣️ API] - ✅️ 添加点赞成功"),
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("[🤐 API] - ❌️ 添加点赞失败: {:?}", e),
                None,
            ),
        }
    }

    ////////

    /// # 2. [API HANDLER] - 取消点赞
    /// * `desc`: `用户取消点赞一个音乐，返回操作结果`
    pub async fn api_del_like(
        uid: i64,        // 操作者 ID
        music_id: i64,   // 音乐 ID
        ctx: AppContext, // 应用上下文
    ) -> AppData<bool> {
        // 调用 Case 层
        match MusicLikeAddCase::case_del_like(uid, music_id, ctx).await {
            Ok(()) => AppData::ok(false).with_msg("[🗣️ API] - ✅️ 取消点赞成功"),
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("[🤐 API] - ❌️ 取消点赞失败: {:?}", e),
                None,
            ),
        }
    }
}

//////// END