// gate_http/src/router_v2/fs/gateway.rs  -- HTTP - 可乐FS - 路由器与网关
// 2026/5/25 06:49 by wx: cestbon10080

////////

use crate::kits::response::IntoApi;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use app_config::app_state::AppState;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_fs::command::bucket::CreateBucketCmd;
use cola_data::cola_fs::command::cdn::{CreateCdnDomainCmd, UpdateCdnDomainCmd};
use cola_fs::api::bucket::BucketApi;
use cola_fs::api::cdn::CdnApi;
use cola_fs::api::file::FileApi;
use cola_fs::api::media::MediaApi;
use std::time::Instant;

////////

/// # [ROUTER] - FS 文件存储 - 路由器
pub fn fs_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/fs")
            .route("/", web::get().to(root))
            .route("/gateway", web::get().to(fs_gateway))
            .route("/gateway", web::post().to(fs_gateway)),
    );
}

// ROOT
pub async fn root() -> HttpResponse {
    HttpResponse::Ok().json(vec!["Cola", "FS", "GATEWAY"])
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

    // 检查登录状态
    let uid = match req.extensions().get::<i64>().copied() {
        Some(id) => id,
        None => 1, // 测试环境默认 uid
    };

    // 🌟 1. URL 与 Body(JSON) 双重命中，Body 为主：url_req.merge(body_req)
    let url_req = url.into_inner();
    let mut api_req = if !body.is_empty() {
        match serde_json::from_slice::<ApiGatewayRequest>(&body) {
            Ok(body_req) => url_req.merge(body_req),
            Err(_) => url_req,
        }
    } else {
        url_req
    };
    api_req.uid = Some(uid);
    api_req = api_req.build();

    let service_name = api_req.service.clone().unwrap_or_default();

    // 2. 业务路由分发（按 service/action 分发到 bucket, cdn, file, media）
    match service_name.as_str() {
        // -------- 存储桶 (Bucket) --------
        "bucket_new" | "bucket_get" => {
            let app_id = api_req.params.get("app_id").cloned().unwrap_or_default();
            BucketApi::api_get_bucket(app_id, &state.ctx)
                .await
                .finish(&req, start)
        }
        "bucket_add" => {
            let cmd = if !body.is_empty() {
                serde_json::from_slice::<CreateBucketCmd>(&body).unwrap_or_default()
            } else {
                CreateBucketCmd::default()
            };
            BucketApi::api_add_bucket(uid, cmd, &state.ctx)
                .await
                .finish(&req, start)
        }
        "bucket_del" => {
            let id = api_req.id;
            BucketApi::api_del_bucket(uid, id, &state.ctx)
                .await
                .finish(&req, start)
        }
        "bucket_search" => {
            let keyword = api_req.keyword.clone();
            BucketApi::api_search_bucket(uid, keyword, &state.ctx)
                .await
                .finish(&req, start)
        }

        // -------- CDN 域名 --------
        "cdn_new" => {
            let app_id = api_req.params.get("app_id").cloned();
            CdnApi::api_get_cdn_list(app_id, api_req.limit, api_req.offset, &state.ctx)
                .await
                .finish(&req, start)
        }
        "cdn_get" => {
            let app_id = api_req.params.get("app_id").cloned().unwrap_or_default();
            CdnApi::api_get_cdn(app_id, &state.ctx)
                .await
                .finish(&req, start)
        }
        "cdn_bucket_get" => {
            CdnApi::api_get_cdn_by_bucket_id(api_req.id, &state.ctx)
                .await
                .finish(&req, start)
        }
        "cdn_id_get" => {
            CdnApi::api_get_cdn_by_id(api_req.id, &state.ctx)
                .await
                .finish(&req, start)
        }
        "cdn_add" => {
            let cmd = match serde_json::from_slice::<CreateCdnDomainCmd>(&body) {
                Ok(cmd) => cmd,
                Err(error) => {
                    return AppData::<()>::err(
                        4001,
                        format!("[🌐 GATEWAY]: ❌️ CDN创建参数解析失败: {}", error),
                        None,
                    )
                    .finish(&req, start);
                }
            };
            CdnApi::api_add_cdn(uid, cmd, &state.ctx)
                .await
                .finish(&req, start)
        }
        "cdn_update" => {
            let cmd = match serde_json::from_slice::<UpdateCdnDomainCmd>(&body) {
                Ok(cmd) => cmd,
                Err(error) => {
                    return AppData::<()>::err(
                        4001,
                        format!("[🌐 GATEWAY]: ❌️ CDN更新参数解析失败: {}", error),
                        None,
                    )
                    .finish(&req, start);
                }
            };
            CdnApi::api_update_cdn(uid, api_req.id, cmd, &state.ctx)
                .await
                .finish(&req, start)
        }
        "cdn_status" => {
            let status = if api_req.status != 0 {
                api_req.status
            } else {
                api_req
                    .params
                    .get("status")
                    .and_then(|value| value.parse::<i16>().ok())
                    .unwrap_or(1)
            };
            if status != 0 && status != 1 {
                return AppData::<()>::err(
                    4001,
                    "[🌐 GATEWAY]: ❌️ CDN状态参数必须是 0 或 1",
                    None,
                )
                .finish(&req, start);
            }
            CdnApi::api_change_cdn_status(uid, api_req.id, status, &state.ctx)
                .await
                .finish(&req, start)
        }
        "cdn_delete" => {
            CdnApi::api_delete_cdn(uid, api_req.id, &state.ctx)
                .await
                .finish(&req, start)
        }

        // -------- 文件对象 (File) --------
        "file_new" | "file_get" => {
            let app_id = api_req.params.get("app_id").cloned().unwrap_or_default();
            FileApi::api_get_file(app_id, &state.ctx)
                .await
                .finish(&req, start)
        }
        "file_add" => {
            let cmd = if !body.is_empty() {
                serde_json::from_slice::<CreateBucketCmd>(&body).unwrap_or_default()
            } else {
                CreateBucketCmd::default()
            };
            FileApi::api_add_file(uid, cmd, &state.ctx)
                .await
                .finish(&req, start)
        }

        // -------- 对象媒体 (Media) --------
        "media_new" | "media_get" => {
            let app_id = api_req.params.get("app_id").cloned().unwrap_or_default();
            MediaApi::api_get_media(app_id, &state.ctx)
                .await
                .finish(&req, start)
        }
        "media_add" => {
            let cmd = if !body.is_empty() {
                serde_json::from_slice::<CreateBucketCmd>(&body).unwrap_or_default()
            } else {
                CreateBucketCmd::default()
            };
            MediaApi::api_add_media(uid, cmd, &state.ctx)
                .await
                .finish(&req, start)
        }

        _ => {
            AppData::<()>::err(
                4000,
                format!("[🌐 GATEWAY]: ⚠️ Unknown FS service: {}", service_name),
                None,
            )
            .finish(&req, start)
        }
    }
}

//////// END
