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
use service::cola_user::permission::query::UserPermissionQueryService;
use std::time::Instant;
use tracing::{error, info, warn};

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
    // 🆕 auth 兜底：未携带 auth 时按匿名会话处理，不直接拒绝
    // ✨ 只有需要权限 >= 2 的写操作才会在后续权限检查中被拦截
    let auth_request = match api_req.auth.clone() {
        Some(auth) => auth,
        None => {
            info!("[🌐 GATEWAY]: 👤 未携带 auth，按游客访问处理 - 权限等级=1");
            cola_data::auth::request::session::AuthSessionRequest::default()
        }
    };

    // 🆕 检查是否有 access_token（游客=1，登录用户>=2）
    let has_token = auth_request.has_token();

    // 🆕 构建权限上下文
    // ✨ auth 为空 / token 无效 / 过期 / 错误 → 统一从权限中心拿游客兜底（权限=1）
    let (session, perm_ctx, uid_final) = if has_token {
        match SessionStateApi::verify_session(&auth_request, &state.ctx.auth).await {
            AppData {
                data: Some(session),
                ..
            } if !session.is_anonymous => {
                // ✅ Token 有效且已登录：按 uid 查询权限（查询失败自动兜底权限=1）
                let uid = session.uid;
                let perm_ctx = UserPermissionQueryService::resolve_permission_context(uid).await;
                (session, perm_ctx, uid)
            }
            _ => {
                // ⚠️ Token 无效/过期/错误 → 权限中心游客兜底（权限=1）
                warn!("[🌐 GATEWAY]: ⚠️  access_token 失效/异常，降级为游客兜底 - 权限等级=1");
                let guest_session = cola_data::auth::request::session::SessionContext {
                    uid: 0,
                    iam_roles: vec![],
                    device_id: String::new(),
                    is_anonymous: true,
                    access_token: String::new(),
                };
                let perm_ctx = UserPermissionQueryService::resolve_permission_context(0).await;
                (guest_session, perm_ctx, 0)
            }
        }
    } else {
        // 无 token（auth 为空）：权限中心游客兜底（权限=1）
        info!("[🌐 GATEWAY]: 👤 游客访问 - 权限等级=1");
        let guest_session = cola_data::auth::request::session::SessionContext {
            uid: 0,
            iam_roles: vec![],
            device_id: String::new(),
            is_anonymous: true,
            access_token: String::new(),
        };
        let perm_ctx = UserPermissionQueryService::resolve_permission_context(0).await;
        (guest_session, perm_ctx, 0)
    };

    let uid = uid_final;
    api_req.uid = Some(uid);
    api_req = api_req.build();
    let auth = cola_data::auth::info::auth::AuthContext {
        uid: session.uid,
        access_token: session.access_token,
        refresh_token: auth_request.refresh_token.clone().unwrap_or_default(),
        device_id: session.device_id,
        iam_roles: session.iam_roles,
        is_anonymous: session.is_anonymous,
        permission_context: Some(perm_ctx),
    };

    //////// MATCH

    // 🌟 对齐到 service 字符串进行业务路由分发
    match api_req.service.clone().unwrap_or_default().as_str() {
        //////// HOME - 权限 >= 1 (游客可访问)

        // 1001 最新 - 权限 >= 1
        "home_new" => {
            if !auth.has_permission_level(1) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足", None)
                    .finish(&req, start);
            }
            HomeApi::home_new(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start)
        }

        // 1002 热门 - 权限 >= 1
        "home_hot" => {
            if !auth.has_permission_level(1) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足", None)
                    .finish(&req, start);
            }
            HomeApi::home_hot(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start)
        }

        // 1003 推荐 - 权限 >= 1
        "home_recommend" => {
            if !auth.has_permission_level(1) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足", None)
                    .finish(&req, start);
            }
            HomeApi::home_recommend(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start)
        }

        // 1004 同城 - 权限 >= 1
        "home_city" => {
            if !auth.has_permission_level(1) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足", None)
                    .finish(&req, start);
            }
            HomeApi::home_city(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start)
        }

        // 1005 分类 - 权限 >= 1
        "home_category" => {
            if !auth.has_permission_level(1) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足", None)
                    .finish(&req, start);
            }
            HomeApi::home_category(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start)
        }

        // 1006 精选 - 权限 >= 1
        "home_featured" => {
            if !auth.has_permission_level(1) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足", None)
                    .finish(&req, start);
            }
            HomeApi::home_featured(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start)
        }

        // 1007 搜索 - 权限 >= 1
        "home_search" => {
            if !auth.has_permission_level(1) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足", None)
                    .finish(&req, start);
            }
            HomeApi::home_search(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start)
        }

        //////// 视频操作 - 权限检查

        // 查看视频详情 - 权限 >= 1
        "view" => {
            if !auth.has_permission_level(1) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足", None)
                    .finish(&req, start);
            }
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

        // 发布视频(测试使用,不可删除) - 权限 >= 2
        "publish_video" => {
            if !auth.has_permission_level(2) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足：需要权限 >= 2", None)
                    .finish(&req, start);
            }
            // 发布视频接口转发
            let data = serde_json::json!({
                "video_id": 12345,
                "user_id": uid,
                "title": "示例视频标题",
                "status": "published"
            });
            AppData::ok(data).finish(&req, start)
        }

        // 发布视频 - 权限 >= 2
        "add_video" => {
            if !auth.has_permission_level(2) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足：需要权限 >= 2", None)
                    .finish(&req, start);
            }
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

        // 获取视频 - 权限 >= 1
        "get_video" => {
            if !auth.has_permission_level(1) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足", None)
                    .finish(&req, start);
            }
            VideoContentGetApi::get_video(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start)
        }

        //////// 评论 - 权限检查

        // 发送评论 - 权限 >= 2
        "send_comment" => {
            if !auth.has_permission_level(2) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足：需要权限 >= 2", None)
                    .finish(&req, start);
            }
            CommentAddApi::add_comment(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start)
        }

        // 获取评论 - 权限 >= 1
        "get_comment" => {
            if !auth.has_permission_level(1) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足", None)
                    .finish(&req, start);
            }
            CommentGetApi::get_comment(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start)
        }

        //////// 弹幕 - 权限检查

        // 发送弹幕 - 权限 >= 2
        "send_danmaku" => {
            if !auth.has_permission_level(2) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足：需要权限 >= 2", None)
                    .finish(&req, start);
            }
            DanmakuAddApi::add_danmaku(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start)
        }

        // 获取弹幕 - 权限 >= 1
        "get_danmaku" => {
            if !auth.has_permission_level(1) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足", None)
                    .finish(&req, start);
            }
            DanmakuGetApi::get_danmaku(auth.clone(), api_req.clone(), &state.ctx)
            .await
            .finish(&req, start)
        }

        //////// 测试接口 - 权限检查 (不可删除)

        // 发布评论 - 权限 >= 2
        "publish_comment" => {
            if !auth.has_permission_level(2) {
                return AppData::<()>::err(4003, "[🌐 GATEWAY]: ❌️ 权限不足：需要权限 >= 2", None)
                    .finish(&req, start);
            }
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
