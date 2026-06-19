// // gate_http/src/handlers/gift.rs
// // 礼物 API
// // 2026-02-07
//
// use actix_web::{HttpResponse, web};
// use std::time::Instant;
//
// // AppState（数据库连接池）
// use api::app_state::AppState;
//
// // store 层：业务逻辑 + 数据模型
// use cola_video::gift::case::gift;
// use cola_video::gift::gate_grpc::gift::GiftEntity;
//
// // 核心 通用结构
// use network::router_v2::response::{ListQuery, ListResponse, ApiResponse};
//
//
// /// # 路由注册 - 礼物
// pub fn config(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::scope("/gift")
//             .route("", web::get().to(get_gift_list)),
//     );
// }
//
// /// ## API - 获取礼物列表（Explorer）
// ///
// /// GET /gift?page=1&size=20
// pub async fn get_gift_list(
//     api: web::Data<AppState>,
//     query: web::Query<ListQuery>,
// ) -> Result<HttpResponse, actix_web::Error> {
//     let start = Instant::now();
//     let pool = &api.db.pg_pool;
//
//     match case::get_gift_list_explorer(pool, query).await {
//         // 空列表（业务上不算错误）
//         Ok(resp) if resp.list.is_empty() => {
//             Ok(
//                 HttpResponse::Ok().json(
//                     ApiResponse::<ListResponse<GiftEntity>>::err(
//                         400,
//                         "啊~！这里还是空的啊！",
//                         start,
//                     )
//                 )
//             )
//         }
//
//         // 正常返回
//         Ok(resp) => {
//             Ok(
//                 HttpResponse::Ok().json(
//                     ApiResponse::ok(resp, start)
//                 )
//             )
//         }
//
//         // 数据库 / case 错误
//         Err(e) => {
//             Ok(
//                 HttpResponse::InternalServerError().json(
//                     ApiResponse::<ListResponse<GiftEntity>>::err(
//                         500,
//                         format!("数据库错误: {}", e),
//                         start,
//                     )
//                 )
//             )
//         }
//     }
// }
