// music/src/api/user/add.rs -- 可乐音乐 - 接口层 - 用户统计数据 - 发布
// 2026-07-08 14:52 Created.

////////

use crate::case::user::add::MusicUserAddCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::music::command::user::MusicUserCreateCommand;
use cola_data::music::info::user::MusicUserInfo; // 引入精简后的统计 Info
use port::app::ctx::AppContext;

////////

/// # [API] - 发布 接口
/// * `desc`: `可乐音乐 - 用户统计数据发布接口`
pub struct MusicUserAddApi;

impl MusicUserAddApi {
    //

    ////////

    /// # 1. [API HANDLER] - 发布
    pub async fn api_add_music_user(
        uid: i64,
        cmd: MusicUserCreateCommand, // 创建命令
        ctx: AppContext,             // 应用上下文
    ) -> AppData<MusicUserInfo> {
        // Call CASE ..
        match MusicUserAddCase::case_add_music_user(uid, cmd, ctx).await {
            Ok(user_info) => {
                AppData::ok(user_info).with_msg("[🗣️ API] - ✅️ 创建音乐用户统计数据成功")
            }
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("[🤐 API] - ❌️ 创建音乐用户统计数据失败: {:?}", e),
                None,
            ),
        }
    }

    ////////

    /// # 2. [API HANDLER] - 编辑
    pub async fn api_edit_music_user(
        uid: i64,                    // 操作者 ID
        url: ApiGatewayRequest,      // 网关参数
        cmd: MusicUserCreateCommand, // 编辑命令
        ctx: AppContext,             // 应用上下文
    ) -> AppData<MusicUserInfo> {
        let id = url.id;

        // Call CASE ..
        match MusicUserAddCase::case_edit_music_user(uid, id, cmd, ctx).await {
            Ok(user_info) => {
                AppData::ok(user_info).with_msg("[🗣️ API] - ✅️ 编辑音乐用户统计数据成功")
            }
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("[🤐 API] - ❌️ 编辑音乐用户统计数据失败: {:?}", e),
                None,
            ),
        }
    }
}

//////// END
