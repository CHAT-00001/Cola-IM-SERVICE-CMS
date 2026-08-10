// repo_adapter/src/auth/mod.rs
// 🔌 插头 - 可乐验证中心 - 模块
// 2026/8/10 20:00 Updated.

////////

use port::cola_auth::AuthServicePorts;
use std::sync::Arc;

////////

pub mod client; // 客户端
pub mod device; // 设备
pub mod iam; // 身份验证管理
pub mod ip_addr; // IP地址
pub mod phone; // 电话
pub mod session; // 会话

////////

/// # [BUILD] - 构建 AUTH Port
/// * `desc`: 构建验证中心 Port 聚合体，包含会话接口
pub fn build_auth_port() -> AuthServicePorts {
    AuthServicePorts {
        session: Arc::new(session::SessionPortAdapter),
    }
}

//////// END
