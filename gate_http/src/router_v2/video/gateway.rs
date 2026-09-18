// gate_http/router_v2/video/gateway.rs -- HTTP网关 - VIDEO - 业务网关
// 2026/6/13 10:21

////////

use crate::kits::response::IntoApi;
use crate::ping::ping;
use actix_web::{HttpRequest, Responder, web};
use app_config::app_state::AppState;
use cola_auth::api::seesion::state::SessionStateApi;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_video::command::video::new::VideoNewCommand;
use cola_video::api::comment::add::CommentAddApi;
use cola_video::api::comment::get::CommentGetApi;
use cola_video::api::danmaku::add::DanmakuAddApi;
use cola_video::api::danmaku::get::DanmakuGetApi;
use cola_video::api::video::add::VideoContentAddApi;
use cola_video::api::video::get::VideoContentGetApi;
use cola_video::api::video::home::HomeApi;
use std::time::Instant;

////////

/// # [GATEWAY] - 解析视频发布命令
/// * `desc`: `兼容 Body 直接传命令和 { cmd: {...} } 包装格式`
fn extract_video_new_cmd(request: &ApiGatewayRequest) -> Result<VideoNewCommand, String> {
    let body = request
        .body
        .as_ref()
        .ok_or_else(|| "视频发布请求缺少 JSON Body".to_string())?;

    let value = body.get("cmd").cloned().unwrap_or_else(|| body.clone());
    serde_json::from_value(value).map_err(|error| format!("视频发布参数解析失败: {error}"))
}

////////

/// # [ROUTER] - 短视频 - 路由器
pub fn video_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // by
        // * /new/xxxx
        web::scope("/video")
            // 默认
            .route("/", web::get().to(ping))
            // 网关
            .route("/gateway", web::get().to(video_gateway))
            .route("/gateway", web::post().to(video_gateway)),
    );
}

////////

/// # [GATEWAY] - 可乐视频网关
pub async fn video_gateway(
    req: HttpRequest,
    url: web::Query<ApiGatewayRequest>,
    body: web::Bytes,
    state: web::Data<AppState>,
) -> impl Responder {
    // 开始时间
    let start = Instant::now();

    let url_req = url.into_inner();
    let mut api_req = if body.is_empty() {
        url_req
    } else {
        let body_value: serde_json::Value = match serde_json::from_slice(&body) {
            Ok(value) => value,
            Err(error) => {
                return AppData::<()>::err(
                    4001,
                    format!("[🌐 GATEWAY]: ❌️ 视频 Body JSON 解析失败: {}", error),
                    None,
                )
                .finish(&req, start);
            }
        };
        // Body 负责覆盖 URL 参数；完整 Body 继续透传给 API，用于解析业务 cmd。
        let mut body_req: ApiGatewayRequest = match serde_json::from_value(body_value.clone()) {
            Ok(value) => value,
            Err(error) => {
                return AppData::<()>::err(
                    4001,
                    format!("[🌐 GATEWAY]: ❌️ 视频 Body 参数解析失败: {}", error),
                    None,
                )
                .finish(&req, start);
            }
        };
        body_req.body = Some(body_value);
        url_req.merge(body_req)
    };
    let auth_request = match api_req.auth.clone() {
        Some(auth) => auth,
        None => {
            return AppData::<()>::err(4010, "[🌐 GATEWAY]: ❌️ 缺少登录认证信息", None)
                .finish(&req, start);
        }
    };

    let session = match SessionStateApi::verify_login(&auth_request, &state.ctx.auth).await {
        AppData {
            data: Some(session),
            ..
        } => session,
        response => {
            return response.rebind::<()>().finish(&req, start);
        }
    };

    let uid = session.uid;
    api_req.uid = Some(uid);
    api_req = api_req.build();
    let auth = cola_data::auth::info::auth::AuthContext {
        uid: session.uid,
        access_token: session.access_token,
        refresh_token: auth_request.refresh_token.clone().unwrap_or_default(),
        device_id: session.device_id,
        iam_roles: session.iam_roles,
        is_anonymous: session.is_anonymous,
    };

    //////// MATCH

    // 🌟 对齐到 service 字符串进行业务路由分发
    match api_req.service.clone().unwrap_or_default().as_str() {
        //////// HOME

        // 1001 最新
        "home_new" => HomeApi::home_new(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start),

        // 1002 热门
        "home_hot" => HomeApi::home_hot(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start),

        // 1003 推荐
        "home_recommend" => HomeApi::home_recommend(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start),

        // 1004 同城
        "home_city" => HomeApi::home_city(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start),

        // 1005 分类
        "home_category" => HomeApi::home_category(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start),

        // 1006 精选
        "home_featured" => HomeApi::home_featured(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start),

        // 1007 搜索
        "home_search" => HomeApi::home_search(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start),

        // 查看视频详情 - (测试接口,不可删除)
        "view" => {
            let video_id = api_req.video_id;
            let data = serde_json::json!({
                "id": video_id,
                "user_id": 1,
                "title": "测试视频标题",
                "description": "这是一个测试视频描述",
                "href": "https://example.com/new/1001",
                "cover": "https://example.com/cover/1001.jpg",
                "views": 12345,
                "likes": 678,
                "comments": 90,
                "duration": 120.5,
                "width": 1920,
                "height": 1080,
                "status": 1,
                "created_at": "2026-06-12T07:00:00Z"
            });
            AppData::ok(data).finish(&req, start)
        }

        // 发布视频(测试使用,不可删除)
        "publish_video" => {
            // 发布视频接口转发
            let data = serde_json::json!({
                "video_id": 12345,
                "user_id": uid,
                "title": "示例视频标题",
                "status": "published"
            });
            AppData::ok(data).finish(&req, start)
        }

        // 发布视频
        "add_video" => {
            let cmd = match extract_video_new_cmd(&api_req) {
                Ok(cmd) => cmd,
                Err(error) => {
                    return AppData::<()>::err(4002, error, None).finish(&req, start);
                }
            };

            VideoContentAddApi::add_video(auth.uid, cmd, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 获取视频
        "get_video" => VideoContentGetApi::get_video(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start),

        //////// 评论

        // 发送评论
        "send_comment" => CommentAddApi::add_comment(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start),

        // 获取评论
        "get_comment" => CommentGetApi::get_comment(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start),

        //////// 弹幕

        // 发送弹幕
        "send_danmaku" => DanmakuAddApi::add_danmaku(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start),

        // 获取弹幕
        "get_danmaku" => DanmakuGetApi::get_danmaku(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start),

        //////// (测试接口, 不可删除)
        "publish_comment" => {
            // 发布评论接口转发
            let data = serde_json::json!({
                "comment_id": 67890,
                "user_id": uid,
                "video_id": api_req.video_id,
                "content": "示例评论内容"
            });
            AppData::ok(data).finish(&req, start)
        }

        //////// 兜底错误
        _ => AppData::<()>::err(
            2004,
            format!(
                "[🌐 GATEWAY]: ⚠️ Unknown The [▶ VIDEO] service: {}",
                api_req.service.unwrap_or_default()
            ),
            None,
        )
        .finish(&req, start),
    }
}

//////// END
