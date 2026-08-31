// gate_http/src/router_v2/music/router2 -- 音乐 - 路由器
// 2026/5/25 03:08 by wx: cestbon10080

////////

use crate::ping::ping;
use crate::router_v1::response::ApiResponse;
use actix_web::web::{Data, get};
use actix_web::{HttpResponse, get, web};
use app_config::app_state::AppState;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::alloc::handle_alloc_error;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;
////////

/// # [ROUTER] - 音乐 - 路由器
pub fn music_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/music")
            .route("", web::get().to(ping))
            //  .route("/{id}", web::get().to(get_client_by_id))
            .route("", web::post().to(create))
            .route("/{id}", web::put().to(update))
            .route("/{id}", web::delete().to(del))
            // 可以添加更多子路由
            .service(
                web::scope("/cola_fs")
                    .route("", web::get().to(get_categories))
                    .route("/{id}", web::get().to(get_videos_by_category)),
            ),
    );
}
#[derive(Deserialize)]
pub struct VideoQuery {
    pub lang: Option<String>, // 可选参数，默认中文
}
#[derive(Deserialize)]
pub struct Messages(HashMap<String, HashMap<String, String>>);

// static MESSAGES: Lazy<Messages> = Lazy::new(|| {
//     let share = include_str!("messages.toml");
//     toml::from_str(share).expect("Failed to parse messages.toml")
// });

// pub fn get_message(key: &str, lang: &str) -> String {
//     MESSAGES.0
//         .get(key)
//         .and_then(|m| m.get(lang))
//         .cloned()
//         .unwrap_or_else(|| MESSAGES.0.get(key).and_then(|m| m.get("zh")).unwrap().clone())
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub id: i64,
    pub name: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub key: Option<String>,
    pub status: i64,
    pub add_time: Option<i64>,
}

pub async fn get_video(video_id: web::Path<i64>) -> HttpResponse {
    let video = Client {
        id: *video_id,
        name: format!("Video {}", video_id).into(),
        title: None,
        description: Option::from("Sample cache description".to_string()),
        key: format!("https://example.com/video{}", video_id).into(),
        status: 0,
        add_time: Some(0),
    };

    HttpResponse::Ok().json(video)
}

#[derive(Debug, Deserialize)]
pub struct CreateVideoRequest {
    pub title: String,
    pub description: String,
    pub url: String,
}

pub async fn create(video_req: web::Json<CreateVideoRequest>) -> HttpResponse {
    let new_video = Client {
        id: 100, // 模拟ID生成
        name: format!("Video {}", video_req.title).into(),
        title: Some(video_req.title.clone()),
        description: Some(video_req.description.clone()),
        key: None,
        status: 0,
        add_time: None,
    };

    HttpResponse::Created().json(new_video)
}

pub async fn update(
    video_id: web::Path<i64>,
    video_req: web::Json<CreateVideoRequest>,
) -> HttpResponse {
    let updated_video = Client {
        id: *video_id,
        name: format!("Video {}", video_req.title).into(),
        title: Some(video_req.title.clone()),
        description: Some(video_req.description.clone()),
        key: None,
        status: 0,
        add_time: None,
    };

    HttpResponse::Ok().json(updated_video)
}

pub async fn del(video_id: web::Path<u64>) -> HttpResponse {
    HttpResponse::NoContent().finish()
}

// 分类相关
pub async fn get_categories() -> HttpResponse {
    HttpResponse::Ok().json(vec!["Action", "Comedy", "Drama"])
}

pub async fn get_videos_by_category() -> HttpResponse {
    HttpResponse::Ok().json(vec!["Video1", "Video2"])
}

/// # 列表查询参数构建
/// 2025-09-11 09:48:10
#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub size: Option<i64>,
    pub page: Option<i64>,
}

#[derive(Serialize)]
pub struct Pagination {
    pub(crate) page: i64,
    pub(crate) size: i64,
}

#[derive(Serialize)]
pub struct ListResponse<T> {
    pub(crate) list: Vec<T>,
    pub(crate) pagination: Pagination,
}

///// # 根据ID获取单个客户端
///// 参数：id
///// 2025-09-16
// async fn get_client_by_id(
//     api: web::Data<AppState>,
//     client_id: web::Path<i64>,
//     query: web::Query<VideoQuery>,
//     path: web::Path<i64>,   // 直接从URL路径拿id
// ) -> Result<HttpResponse, actix_web::Error> {
//     let lang = query.lang.clone().unwrap_or("zh".to_string());
//     let msg = get_message("video_not_found", &lang);
//     let start = Instant::now();
//     let pool = &api.db.pg_pool;
//     let id = client_id;
//
//     // 构建SQL
//     let query = r#"
//         SELECT id, name, cname, description, key, add_time, status
//         FROM client
//         WHERE id = $1
//             AND status = 1
//         LIMIT 1
//     "#;
//
//     // 查询数据
//     let row = sqlx::query(query)
//         .bind(*id)
//         .fetch_optional(pool)
//         .await
//         .map_err(anyhow::Error::from)
//         .map_err(|e| {
//             tracing::error!("DB query failed: {:?}", e);
//             actix_web::error::ErrorInternalServerError("Database error")
//         })?;
//
//     // 是否存在
//     let msg = get_message("video_not_found", &lang);
//     let row = match row {
//         Some(r) => r,
//         None => {
//             let resp = ApiResponse::<Option<Client>>::err(
//                 404,
//                 msg,
//                 start,
//             );
//             return Ok(HttpResponse::Ok().json(resp));
//         }
//     };
//
//
//     // 拼接结果
//     let client = Client { id: val, name: val, title: val, description: val, key: val, status: val, add_time: val };
//
//
//     let resp = ApiResponse::ok(client, start);
//     Ok(HttpResponse::Ok().json(resp))
// }
//
