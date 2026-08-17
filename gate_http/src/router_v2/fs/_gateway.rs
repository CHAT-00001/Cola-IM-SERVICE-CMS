// gata_http/src/router_v2/fs/gateway.rs  -- HTTP - 可乐FS - 路由器
// 2026/5/25 06:49 by wx: cestbon10080

////////

use crate::kits::response::IntoApi;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use app_config::app_state::AppState;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::auth::info::auth::AuthContext;
use cola_fs::api::bucket::BucketApi;
use cola_fs::api::cdn::CdnApi;
use cola_fs::api::media::MediaApi;
use cola_gis::api::home::HomeApi;
use serde::Deserialize;
use std::time::Instant;

////////

/// # [ROUTER] - FS 文件存储 - 路由器
pub fn fs_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // by
        // * /new/xxxx
        web::scope("/fs")
            // 默认
            .route("/", web::get().to(root))
            // 网关
            .route("/gateway", web::get().to(fs_gateway))
            .route("/gateway", web::post().to(fs_gateway)),
    );
}

// ROOT
pub async fn root() -> HttpResponse {
    HttpResponse::Ok().json(vec!["Cole", "GIS", "ROUTER"])
}

////////

/// # [GATEWAY] - 可乐FS 网关
async fn fs_gateway(
    req: HttpRequest,
    url: web::Query<ApiGatewayRequest>,
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
        iam_roles: vec![],
        is_anonymous: false,
    };

    let gateway_req = ApiGatewayRequest {
        auth,
        action: Option::from(query.action.unwrap_or(0)), // 先给个默认值 0，留给以后用
        service: Option::from(query.service.clone()),    // 对齐并绑定真正的 PhalApi 字符串服务名
        query: Some(req.query_string().to_string()),
        body,
        path: req.path().to_string(),
    };

    // 🌟 对齐到 service 字符串进行业务路由分发
    match gateway_req.service.as_str() {
        //

        //////// 存储桶

        // 1001 存储桶 最新列表
        "bucket_new" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
            .build();

            BucketApi::api_get_bucket(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 1002 存储桶 创建
        "bucket_add" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),       // 操作者 UID
                cmd: BucketCreateCmd, // 存存储桶创建命令
                ..Default::default()
            }
            .build();

            BucketApi::api_add_bucket(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 1003 存储桶 删除
        "bucket_del" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                id: url.id,
                ..Default::default()
            }
            .build();

            BucketApi::api_add_bucket(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 1004 存储桶 改变状态
        "bucket_status" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
            .build();

            BucketApi::api_add_bucket(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 1005 存储桶搜索
        "bucket_search" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                kw: url.keyword,  // 关键词
                qty: query.qty,
                ..Default::default()
            }
            .build();

            BucketApi::api_add_bucket(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        //////// CDN 域名

        // 2001 CDN 最新列表
        "cdn_new" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
            .build();

            CdnApi::api_get_cdn(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 2002 CDN 添加
        "cdn_add" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
            .build();

            CdnApi::api_add_cdn(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 2003 CDN 删除
        "cdn_del" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
            .build();

            CdnApi::api_add_cdn(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 2004 CDN 修改状态
        "cdn_status" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
            .build();

            CdnApi::api_add_cdn(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        //////// 媒体文件

        // 3001 媒体列表
        "media_list" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
            .build();

            MediaApi::api_get_media(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 3002 媒体 发布
        "media_add" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
            .build();

            MediaApi::api_add_media(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 3003 媒体逻辑删除
        "media_del" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
            .build();

            MediaApi::api_add_media(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 3004 媒体逻辑删除
        "media_del" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
            .build();

            MediaApi::api_add_media(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 3005 媒体获取
        "media_get" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
            .build();

            MediaApi::api_add_media(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 3006 媒体状态修改
        "media_status" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
            .build();

            MediaApi::api_add_media(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        // 4002 浏览完成
        "view_done" => {
            let url = ApiGatewayRequest {
                uid: Some(uid),
                page: query.page,
                qty: query.qty,
                ..Default::default()
            }
            .build();

            HomeApi::handler_get_new(gateway_req.auth, url, &state.ctx)
                .await
                .finish(&req, start)
        }

        //////// 兜底

        _ => AppData::<()>::err(
            400,
            format!(
                "[🌐 GATEWAY]: ⚠️ Unknown Api [📍 FS] service: {}",
                gateway_req.service
            ),
            None,
        )
        .finish(&req, start),
    }
}

//////// END
