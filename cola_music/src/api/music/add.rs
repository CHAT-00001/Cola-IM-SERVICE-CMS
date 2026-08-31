// music/src/api/music/add.rs -- 可乐音乐 - 接口层 - 音乐 - 发布
// 2026-07-08 14:52 Created.

////////

use crate::case::music::add::MusicAddCase;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::music::command::music::new::{MusicCreateCommand, MusicUpdateCommand};
use cola_data::music::vo::music::MusicSingleResponse;
use port::app::ctx::AppContext;

////////

/// # [API] - 发布 接口
/// * `desc`: `可乐音乐 - 发布接口`
pub struct MusicAddApi;

impl MusicAddApi {
    //

    ////////

    /// # 1. [API HANDLER] - 发布
    pub async fn api_add_music(
        user_id: i64,
        url: ApiGatewayRequest,  // 网关请求参数
        cmd: MusicCreateCommand, // 音乐更新命令
        ctx: AppContext,         // 应用上下文
    ) -> AppData<MusicSingleResponse> {
        // 1. 检查用户登录会话

        // 2. 检查用户状态

        // 3. 检查用户音乐发布权限

        // 4. 发布业务
        match MusicAddCase::case_add_music(user_id, cmd, ctx).await {
            Ok(resp) => AppData::ok(resp).with_msg("[🗣️ API] - ✅️ 音乐信息发布成功"),
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("[🤐 API] - ❌️ 音乐信息发布失败: {:?}", e),
                None,
            ),
        }
    }

    ////////

    /// # 2. [API HANDLER] - 编辑
    pub async fn api_edit_music(
        uid: i64,
        url: ApiGatewayRequest,  // 网关请求参数
        cmd: MusicUpdateCommand, // 音乐更新命令
        ctx: AppContext,         // 应用上下文
    ) -> AppData<MusicSingleResponse> {
        // 1. 检查用户登录会话

        // 2. 检查用户状态

        // 3. 检查用户音乐发布权限

        // 4. 检查音乐 ID状态
        let music_id = url.id;
        // 4. 编辑业务
        match MusicAddCase::case_edit_music(uid, music_id, cmd, ctx).await {
            Ok(resp) => AppData::ok(resp).with_msg("[🗣️ API] - ✅️ 音乐信息编辑成功"),
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("[🤐 API] - ❌️ 音乐信息编辑失败: {:?}", e),
                None,
            ),
        }
    }
}

//////// END
