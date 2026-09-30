// port/src/basic/mod.rs -- 端口 - BASIC - mod
// 2026/6/10 07:45

////////

use crate::auth::session::SessionPort;
use crate::basic::app::AppPort;
use crate::basic::bonding::BondingPort;
use crate::basic::node::NodePort;
use crate::basic::media::MediaPort;
use crate::basic::upload::UploadSessionPort;
use std::sync::Arc;

////////
pub mod app; // 应用
pub mod bonding; // 绑定
pub mod node; // 节点
pub mod media; // 备用
pub mod session; // 登录会话
pub mod upload; // 通用上传


////////

/// # AUTH 上下文模型
#[derive(Clone)]
pub struct AuthServicePorts {
    /// 会话校验端口（Token 验证 → SessionContext）
    pub session: Arc<dyn SessionPort + Send + Sync + 'static>,
}

/// # [COLA FS PORTS] - 验证
/// * `desc`: `FS - Cola BASIC Service Port`
#[derive(Clone)]
pub struct ColaBasicPort {
    pub app: AppPort,         // 应用
    pub node: NodePort,       // 节点
    pub bonding: BondingPort, // 绑定
    pub media: MediaPort,     // 媒体
}

//////// END
