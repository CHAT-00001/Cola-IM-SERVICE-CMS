// cola_video/src/api/danmaku/add.rs -- VIDEO - API - 弹幕发布
// 2026/9/7 Created.

////////

use crate::case::danmaku::add::DanmakuAddCase;
use crate::model::vo::danmaku::DanmakuSingleResponse;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::auth::info::auth::AuthContext;
use cola_data::cola_video::command::danmaku::DanmakuCommand;
use port::app::ctx::AppContext;

////////

/// # [API HANDLE] - 视频弹幕发布接口
pub struct DanmakuAddApi;

impl DanmakuAddApi {
    //

    ////////

    /// # 1. [API HANDLE] - 发布弹幕
    /// * `desc`: 业务 `id` 指代视频 ID，其他视频 ID 字段仅作兼容兜底
    pub async fn add_danmaku(
        auth: AuthContext,      // 可信会话
        url: ApiGatewayRequest, // 网关请求
        ctx: &AppContext,       // 应用上下文
    ) -> AppData<DanmakuSingleResponse> {
        // 请求体解析
        let cmd_value = url
            .body
            .as_ref()
            .map(|body| body.get("cmd").cloned().unwrap_or_else(|| body.clone()))
            .unwrap_or_default();

        // 查找发布命令
        let cmd: DanmakuCommand = match serde_json::from_value(cmd_value) {
            Ok(value) => value,
            Err(error) => return AppData::err(4002, format!("弹幕参数解析失败: {error}"), None),
        };

        // 快速检查必参
        let video_id = if url.id > 0 {
            url.id
        } else if url.video_id > 0 {
            url.video_id
        } else {
            cmd.video_id
        };
        if video_id <= 0 || cmd.content.trim().is_empty() {
            return AppData::err(4002, "id 和 content 不能为空", None);
        }

        // Call CASE ..
        match DanmakuAddCase::case_send(auth.uid, video_id, cmd, ctx.clone()).await {
            Ok(response) => AppData::ok(response).with_msg("弹幕发布成功"),
            Err(error) => AppData::err(5000, format!("弹幕发布失败: {error}"), None),
        }
    }
}

//////// END
