// src/middleware/session
// 2026-01-16 13:24:47

use actix_web::{FromRequest, HttpRequest, dev::Payload, HttpMessage};
use futures_util::future::{ready, Ready};
use crate::models::request::ApiRequest;
use uuid::Uuid;

impl FromRequest for ApiRequest {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let headers = req.headers();

        // 1. 获取或生成 Request ID
        let request_id = headers
            .get("X-Request-Id")
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_string())
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        // 2. 解析坐标 (假设 Header 传了 X-Lat, X-Lng)
        let lat = headers.get("X-Lat")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<f64>().ok());

        let lng = headers.get("X-Lng")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<f64>().ok());

        // 3. 解析 Token (这里只拿出来，具体的鉴权逻辑可以在中间件或 Service 做)
        let token = headers.get("Authorization")
            .and_then(|v| v.to_str().ok())
            .map(|v| v.replace("Bearer ", ""));

        // 4. 注意：user_id 这里改为从 HttpMessage 的 extensions 中尝试获取
        let user_id = req.extensions().get::<i64>().cloned();

        ready(Ok(ApiRequest {
            request_id,
            client_id: headers.get("X-Client-Id").and_then(|v| v.to_str().ok()).map(|v| v.to_string()),
            user_id, // 此时 user_id 来源于中间件注入
            token,
            lat,
            lng,
        }))
    }
}
