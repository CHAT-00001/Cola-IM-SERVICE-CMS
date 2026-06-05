// cola_data/src/app/cola_data.rs  -- 数据 - 全局应用中心 - cola_data  （统一应用响应壳）
// 2026/5/22 13:46 by wx: cestbon10080
// * --------
// * --------

use std::time::Instant;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

////////

/// # 统一应用数据体 (泛型版)
#[derive(Serialize, Debug)]
pub struct AppData<T> {
    pub code: i32,          // 0: 成功, 其他: 错误码
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub duration: String,
    pub request_id: String,
    pub at: String,
    pub log_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}


/// # [BUILD] - 构造响应壳
impl<T> AppData<T> {
    /// ✅ 成功响应（彻底泛型化）
    /// 现在它可以接收 String, AuthSessionResponse, VideoListResponse 等任何类型
    pub fn ok(data: T) -> Self {
        Self {
            code: 0,
            message: "success".into(),
            error: None,
            duration: "".into(),
            request_id: Uuid::new_v4().simple().to_string(),
            at: Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
            log_id: "".to_string(),
            data: Some(data),
        }
    }

    /// ✅ 成功响应（无数据返回）
    /// 改进：直接返回 AppData<T> 以适配上下文，不需要硬写死为 AppData<()>
    pub fn empty() -> Self {
        Self {
            code: 0,
            message: "success".into(),
            error: None,
            duration: "".into(),
            request_id: Uuid::new_v4().simple().to_string(),
            at: Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
            log_id: "".to_string(),
            data: None,
        }
    }

    /// 错误响应
    pub fn err(code: i32, message: impl Into<String>, detail: Option<String>) -> Self {
        Self {
            code,
            message: message.into(),
            error: detail,
            duration: "".into(),
            request_id: Uuid::new_v4().simple().to_string(),
            at: Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
            log_id: "".to_string(),
            data: None,
        }
    }

    /// 链式调用：修改消息内容
    pub fn with_msg(mut self, msg: impl Into<String>) -> Self {
        self.message = msg.into();
        self
    }

    /// 泛型重绑定：支持从 AppData<A> 转换到 AppData<B>
    pub fn rebind<U>(self) -> AppData<U> {
        AppData {
            code: self.code,
            message: self.message,
            error: self.error,
            duration: self.duration,
            request_id: self.request_id,
            at: self.at,
            log_id: self.log_id,
            data: None,
        }
    }

    /// 检查响应是否成功
    pub fn check(self) -> Result<T, AppData<T>> {
        if self.code == 0 {
            Ok(self.data.expect("Data missing"))
        } else {
            Err(self)
        }
    }
}

// ... 保持 ListQuery 和 Pagination 定义不变 ...

