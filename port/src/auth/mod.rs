// port/src/auth/port/mod.rs
// 端口 - 认证 - 中心服务端口
// 2026/6/10 07:45

////////

use crate::auth::identity::AuthIdentityPort;
use crate::auth::session::{AuthSessionPort, SessionPort};
use std::sync::Arc;

////////
pub mod identity; // 身份识别
pub mod session; // 登录会话

////////

/// # AUTH 上下文模型
#[derive(Clone)]
pub struct AuthServicePorts {
    /// 会话校验端口（Token 验证 → SessionContext）
    pub session: Arc<dyn SessionPort + Send + Sync + 'static>,
}

//////// END

/// # [COLA AUTH PORTS] - 验证
/// * `desc`: `AUTH - Cola Auth Service Port`
#[derive(Clone)]
pub struct ColaVideoPort {
    pub identity: AuthIdentityPort, // 身份
    pub session: AuthSessionPort,   // 会话
}

//////// END
