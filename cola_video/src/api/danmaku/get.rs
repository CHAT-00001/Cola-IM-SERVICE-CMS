// cola_video/src/api/danmaku/get.rs -- VIDEO - api - 弹幕 - 获取接口
// 2026/9/7 15:22 Created.

////////

use crate::case::danmaku::get::DanmakuGetCase;
use crate::model::vo::danmaku::DanmakuListResponse;
use cola_data::app::data::AppData;
use cola_data::app::error;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::auth::info::auth::AuthContext;
use port::app::ctx::AppContext;

////////

/// # [ADD HANDLE] - 视频弹幕获取接口
/// * `DESC`: `COLA VIDEO - Danmaku Get Api`
pub struct DanmakuGetApi;

// 构造函数
impl DanmakuGetApi {
    //

    ////////

    /// # 1. [API HANDLE] - 获取弹幕
    /// * `DESC`: `用户根据视频ID获取弹幕列表`
    pub async fn get_danmaku(
        auth: AuthContext,      // 可信会话
        url: ApiGatewayRequest, // 网关请求
        ctx: &AppContext,       // 应用上下文
    ) -> AppData<DanmakuListResponse> {
        // 1. 解析业务命令节点（兼容 { cmd: {...} } 包装与平铺两种 Body 格式）
        let cmd_value = url
            .body
            .as_ref()
            .map(|body| body.get("cmd").cloned().unwrap_or_else(|| body.clone()))
            .unwrap_or_default();

        // 2. 播放时间：优先 URL params，其次 Body 的 cmd.play_time
        let play_time = url
            .params
            .get("play_time")
            .and_then(|value| value.parse::<i32>().ok())
            .or_else(|| {
                cmd_value
                    .get("play_time")
                    .and_then(serde_json::Value::as_i64)
                    .map(|value| value as i32)
            })
            .unwrap_or(0);

        let qty = url.qty.unwrap_or(20).clamp(1, 50) as i32;

        // 3. 视频 ID：优先 Body 的 cmd.video_id（播放器显式指定），其次 URL 的 video_id / id
        let cmd_video_id = cmd_value
            .get("video_id")
            .and_then(serde_json::Value::as_i64)
            .unwrap_or(0);
        let video_id = if cmd_video_id > 0 {
            cmd_video_id
        } else if url.video_id > 0 {
            url.video_id
        } else {
            url.id
        };
        if video_id <= 0 {
            return AppData::err(4002, "id 不能为空", None);
        }

        // 4. 获取用例
        match DanmakuGetCase::case_new(auth.uid, video_id, play_time, qty, ctx.clone()).await {
            Ok(resp) => AppData::ok(resp).with_msg("获取最新的弹幕列表成功"),
            Err(e) => AppData::err(
                error::INTERNAL_ERROR,
                format!("获取最新的弹幕列表失败: {:?}", e),
                None,
            ),
        }
    }
}

//////// END
