// cola_music/src/api/album/add.rs -- 可乐音乐 - 接口层 - 专辑 - 发布
// 2026-07-08 14:52 Created.

////////

use crate::case::album::add::MusicAlbumAddCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::music::command::album::add::CreateMusicAlbumCmd;
use cola_data::music::command::album::edit::UpdateMusicAlbumCmd;
use cola_data::music::vo::album::MusicAlbumSingleResponse;
use port::app::ctx::AppContext;

////////

/// # [API] - 发布 接口
/// * `desc`: `可乐音乐 - 专辑发布接口`
pub struct MusicAlbumAddApi;

impl MusicAlbumAddApi {
    //

    ////////

    /// # 1. [API HANDLER] - 发布
    pub async fn api_add_album(
        user_id: i64,             // 操作者 ID
        cmd: CreateMusicAlbumCmd, // 命令
        ctx: AppContext,          // 应用上下文
    ) -> AppData<MusicAlbumSingleResponse> {
        // Call CASE ..
        match MusicAlbumAddCase::case_add_album(user_id, cmd, ctx).await {
            Ok(resp) => AppData::ok(resp).with_msg("[🗣️ API] - ✅️ 创建专辑成功"),
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("[🤐 API] - ❌️ 创建专辑失败: {:?}", e),
                None,
            ),
        }
    }

    ////////

    /// # 2. [API HANDLER] - 编辑
    pub async fn api_edit_album(
        uid: i64,                 // 操作者 ID
        url: ApiGatewayRequest,   // 网关请求参数
        cmd: UpdateMusicAlbumCmd, // 修改命令
        ctx: AppContext,          // 应用上下文
    ) -> AppData<MusicAlbumSingleResponse> {
        
        let album_id = url.id;

        // Call CASE ..
        match MusicAlbumAddCase::case_edit_album(uid, album_id, cmd, ctx).await {
            Ok(resp) => AppData::ok(resp).with_msg("[🗣️ API] - ✅️ 编辑专辑信息成功"),
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("[🤐 API] - ❌️ 编辑专辑信息失败: {:?}", e),
                None,
            ),
        }
    }
}

//////// END
