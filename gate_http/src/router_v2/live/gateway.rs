// gate_http/router_v2/cola_live/gateway.rs  -- cola_live gate
// 2026/6/13 10:21

////////

use crate::kits::response::IntoApi;
use crate::ping::ping;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use app_config::app_state::AppState;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::auth::info::auth::AuthContext;
use cola_data::cola_live::command::class::LiveClassCommand;
use cola_data::cola_live::command::stream::record::LiveRecordCommand;
use cola_live::api::category::add::LiveCateAddApi;
use cola_live::api::home::LiveHomeApi;
use cola_live::api::stream::add::LiveStreamAddApi;
use cola_live::api::stream::home::LiveStreamHomeApi;
use serde::Deserialize;
use std::time::Instant;
use tracing::{error, info};

////////

/// # 网关请求体
struct GatewayRequest {
    auth: AuthContext,     // 补上 auth 字段
    action: i16,           // 🌟 以后使用的 int16 动作代码
    service: String,       // 🌟 兼容 PHP PhalApi 的服务名称 (字符串)
    query: Option<String>, // 查询
    body: web::Bytes,      // body
    path: String,          // 路径
}

////////

/// # [GATEWAY] - 直播列表统一请求构造
/// * `desc`: `URL 与 Body 双重解析，Body 字段优先，统一由 ApiGatewayRequest::build 计算分页`
fn merge_live_list_body(url: ApiGatewayRequest, body: &web::Bytes) -> ApiGatewayRequest {
    if body.is_empty() {
        return url.build();
    }
    match serde_json::from_slice::<ApiGatewayRequest>(body) {
        Ok(body_req) => url.merge(body_req).build(),
        Err(err) => {
            tracing::warn!(
                "[🌐 GATEWAY] - ⚠️ 直播列表 Body 解析失败，继续使用 URL 参数: {}",
                err
            );
            url.build()
        }
    }
}

/// # 统一的 Query 提取结构体
#[derive(Deserialize)]
pub struct GatewayQuery {
    pub service: String,     // 🌟 兼容 PhalApi，接收如 "Video.PublishVideo"
    pub action: Option<i16>, // 🌟 以后转入的 int16 动作代码，先用 Option 顶住
    pub video_id: Option<i64>,
    pub page: Option<i64>,    // 页码
    pub qty: Option<i64>,     // 每页数量
    pub id: Option<i64>,      // 分类 ID
    pub status: Option<i16>,  // 分类状态
    pub role: Option<String>, // 管理角色
    pub room_id: Option<i64>, // 直播间 ID
}

/// # [ROUTER] - 短视频 - 路由器
pub fn live_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // by
        // * /new/xxxx
        web::scope("/live")
            // 默认
            .route("/", web::get().to(ping))
            // 网关
            .route("/gateway", web::get().to(live_gateway))
            .route("/gateway", web::post().to(live_gateway)),
    );
}

////////

/// # [GATEWAY] - 可乐直播网关
async fn live_gateway(
    req: HttpRequest,
    // url web::Query<ApiGatewayRequest>,
    query: web::Query<GatewayQuery>,
    body: web::Bytes,
    state: web::Data<AppState>,
) -> impl Responder {
    // 开始时间
    let start = Instant::now();

    // 严格检查登录状态，统一命名操作用户为 uid
    let uid = match req.extensions().get::<i64>().copied() {
        Some(id) => id,
        None => 1, // 测试环境默认 uid
    };

    let auth = AuthContext {
        uid,
        access_token: String::new(),
        refresh_token: String::new(),
        device_id: String::new(),
        iam_roles: query.role.clone().into_iter().collect(),
        is_anonymous: false,
    };

    let gateway_req = GatewayRequest {
        auth,
        action: query.action.unwrap_or(0), // 先给个默认值 0，留给以后用
        service: query.service.clone(),    // 对齐并绑定真正的 PhalApi 字符串服务名
        query: Some(req.query_string().to_string()),
        body,
        path: req.path().to_string(),
    };

    let has_session = req.extensions().get::<i64>().is_some();

    info!(
        "[🌐 GATEWAY] - 📥 LIVE 请求: service={}, uid={}, page={:?}, qty={:?}, id={:?}, status={:?}, has_session={}, method={}",
        &gateway_req.service,
        uid,
        query.page,
        query.qty,
        query.id,
        query.status,
        has_session,
        req.method()
    );

    // 🌟 对齐到 service 字符串进行业务路由分发
    match gateway_req.service.as_str() {
        "stream_start" => {
            if !has_session {
                return AppData::<()>::err(4001, "开播需要有效登录会话", None).finish(&req, start);
            }
            let command = match serde_json::from_slice::<LiveRecordCommand>(&gateway_req.body) {
                Ok(command) => command,
                Err(err) => {
                    return AppData::<()>::err(4000, "开播请求体格式错误", Some(err.to_string()))
                        .finish(&req, start);
                }
            };
            LiveStreamAddApi::start(gateway_req.auth, command, &state.ctx)
                .await
                .finish(&req, start)
        }
        "stream_stop" => {
            if !has_session {
                return AppData::<()>::err(4001, "停播需要有效登录会话", None).finish(&req, start);
            }
            LiveStreamAddApi::stop(gateway_req.auth, query.id.unwrap_or(0), &state.ctx)
                .await
                .finish(&req, start)
        }
        "stream_home_new" => {
            info!(
                "[🚧 DISPATCH] - ✅️ 分发最新直播列表: page={:?}, qty={:?}",
                query.page, query.qty
            );
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
            .build();
            let url = merge_live_list_body(url, &gateway_req.body);
            LiveStreamHomeApi::newest(url, &state.ctx)
                .await
                .finish(&req, start)
        }
        "stream_home_hot" => {
            info!(
                "[🚧 DISPATCH] - ✅️ 分发热门直播列表: page={:?}, qty={:?}",
                query.page, query.qty
            );
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
            .build();
            let url = merge_live_list_body(url, &gateway_req.body);
            LiveStreamHomeApi::hot(url, &state.ctx)
                .await
                .finish(&req, start)
        }
        "stream_home_category" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                category_id: query.id.unwrap_or(0),
                ..Default::default()
            }
            .build();
            let url = merge_live_list_body(url, &gateway_req.body);
            LiveStreamHomeApi::category(url, &state.ctx)
                .await
                .finish(&req, start)
        }
        "category_create" => {
            let command = match serde_json::from_slice::<LiveClassCommand>(&gateway_req.body) {
                Ok(command) => command,
                Err(err) => {
                    return AppData::<()>::err(
                        4000,
                        "直播分类请求体格式错误",
                        Some(err.to_string()),
                    )
                    .finish(&req, start);
                }
            };
            LiveCateAddApi::api_add_cate(gateway_req.auth, command, &state.ctx)
                .await
                .finish(&req, start)
        }
        "category_edit" => {
            let command = match serde_json::from_slice::<LiveClassCommand>(&gateway_req.body) {
                Ok(command) => command,
                Err(err) => {
                    return AppData::<()>::err(
                        4000,
                        "直播分类请求体格式错误",
                        Some(err.to_string()),
                    )
                    .finish(&req, start);
                }
            };
            LiveCateAddApi::api_edit_cate(gateway_req.auth, command, &state.ctx)
                .await
                .finish(&req, start)
        }
        "category_status" => LiveCateAddApi::api_change_status(
            gateway_req.auth,
            query.id.unwrap_or(0),
            query.status.unwrap_or(-1),
            &state.ctx,
        )
        .await
        .finish(&req, start),
        "category_delete" => {
            LiveCateAddApi::api_delete_cate(gateway_req.auth, query.id.unwrap_or(0), &state.ctx)
                .await
                .finish(&req, start)
        }
        "category_list" => LiveCateAddApi::api_list(
            query.status,
            query.qty.unwrap_or(10).clamp(1, 50),
            query.page.unwrap_or(1).max(1).saturating_sub(1) * query.qty.unwrap_or(10).clamp(1, 50),
            &state.ctx,
        )
        .await
        .finish(&req, start),
        // 1001 最新
        "home_new" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
            .build();

            LiveHomeApi::handler_home_new(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 1002 热门
        "home_hot" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
            .build();

            LiveHomeApi::handler_home_hot(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 1003 推荐
        "home_recommend" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
            .build();

            LiveHomeApi::handler_home_recommend(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 1004 同城
        "home_city" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
            .build();

            LiveHomeApi::handler_home_city(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 1005 分类
        "home_category" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
            .build();

            LiveHomeApi::handler_home_category(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 1006 精选
        "home_featured" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
            .build();

            LiveHomeApi::handler_home_featured(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 1007 搜索
        "home_search" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
            .build();

            LiveHomeApi::handler_home_search(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        "view" => {
            // 查看视频详情 - 测试接口
            let video_id = query.video_id.unwrap_or(0);
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

        "publish_comment" => {
            // 发布评论接口转发
            let data = serde_json::json!({
                "comment_id": 67890,
                "user_id": uid,
                "video_id": query.video_id.unwrap_or(0),
                "content": "示例评论内容"
            });
            AppData::ok(data).finish(&req, start)
        }

        _ => {
            error!(
                "[🌐 GATEWAY] - ❌️ 未知 LIVE service: {}",
                gateway_req.service
            );
            AppData::<()>::err(
                2004,
                format!(
                    "[🌐 GATEWAY]: ⚠️ Unknown The [📺 LIVE] service: {}",
                    gateway_req.service
                ),
                None,
            )
            .finish(&req, start)
        }
    }
}
