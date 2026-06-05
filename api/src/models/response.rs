// api src/models/response.rs -- Api 响应体
// 2026-01-04 07:10:10

use chrono::Utc;
use serde::Serialize;
use std::time::Instant;
use uuid::Uuid;

/// 统一响应体
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,          // 业务码
    pub status: i32,        // 状态码
    pub request_id: String, // 请求id
    pub msg: String,        // 消息
    pub at: String,         // UTC 时间 ISO8601 精确到毫秒
    pub latency: String,    // 请求耗时（ms）
    pub data: Option<T>,    // 数据体
}

impl<T> ApiResponse<T> {
    /// 构造成功响应，耗时精确到两位小数毫秒
    pub fn ok(data: T, start: Instant) -> Self {
        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0; // 毫秒，带小数
        Self {
            code: 0,
            status: 200,
            request_id: uuid::Uuid::new_v4().simple().to_string(),
            msg: "success".to_string(),
            at: chrono::Utc::now()
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
            latency: format!("{:.2}ms", elapsed_ms),
            data: Some(data),
        }
    }

    /// 构造失败响应，耗时精确到两位小数毫秒
    pub fn err(code: i32, msg: impl Into<String>, start: Instant) -> Self {
        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
        Self {
            code,
            status: 200,
            request_id: uuid::Uuid::new_v4().simple().to_string(),
            msg: msg.into(),
            at: chrono::Utc::now()
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
            latency: format!("{:.2}ms", elapsed_ms),
            data: None,
        }
    }
}
