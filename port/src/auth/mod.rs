// port/src/auth/port/mod.rs
// 🕳️ 端口 - 认证中心服务端口
// 2026/6/10 07:45

////////

pub mod session;
use std::sync::Arc;
use crate::auth::session::SessionPort;

////////

/// # AUTH 上下文模型
#[derive(Clone)]
pub struct AuthServicePorts {
    /// 会话校验端口（Token 验证 → SessionContext）
    pub session: Arc<dyn SessionPort + Send + Sync + 'static>,
}

//////// END
