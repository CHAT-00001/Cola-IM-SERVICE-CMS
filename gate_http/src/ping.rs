// gate/src/ping.rs  --
// 2026/6/13 19:18

////////

use actix_web::{HttpResponse, Responder};
use chrono::Utc;
use uuid::Uuid;

////////

/// # [PING]
/// * `PONG`
pub async fn ping(name: Option<String>) -> impl Responder {

    let utc_time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let app_name = name.unwrap_or("默认应用名".to_string());

    let uuid = Uuid::new_v4();
    let uuid_prefix = &uuid.to_string()[0..8];

    // 重点：里面的双引号改成单引号 ' 就不会报错了
    let body = format!(
        "<h1 style='color: #000000'>Cola:{}</h1>
         <h4 style='color: #666666'>version: 0.1.0</h4>
         <h4 style='color: #333333'>build: 2026/06/12</h4>
         <h1 style='color: #0066ff'>pong</h1>
        <p style='color: #00dd00; font-size: 16px'>UTC Time: {} | UUID (first 8 chars): {}</p>",
        app_name, utc_time, uuid_prefix
    );

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

//////// END