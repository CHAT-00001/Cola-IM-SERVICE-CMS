use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use uuid::Uuid;

/// 统一响应体
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub request_id: String,
    pub data: Option<T>,
    pub msg: String,
    pub at: String, // UTC 时间 ISO8601 精确到毫秒
    pub time: u128, // 请求耗时（ms）
}

impl<T> ApiResponse<T> {
    /// 构造成功响应
    pub fn ok(data: T, start: Instant) -> Self {
        Self {
            code: 0,
            request_id: Uuid::new_v4().simple().to_string(), // 无分隔符 uuidv4
            data: Some(data),
            msg: "success".to_string(),
            at: Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
            time: start.elapsed().as_millis(),
        }
    }

    /// 构造失败响应
    pub fn err(code: i32, msg: impl Into<String>, start: Instant) -> Self {
        Self {
            code,
            request_id: Uuid::new_v4().simple().to_string(),
            data: None,
            msg: msg.into(),
            at: Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
            time: start.elapsed().as_millis(),
        }
    }
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
    pub page: i64,
    pub size: i64,
}

#[derive(Serialize)]
pub struct ListResponse<T> {
    pub list: Vec<T>,
    pub pagination: Pagination,
}
