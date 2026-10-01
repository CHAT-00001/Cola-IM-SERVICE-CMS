// repo_adapter/src/aaaa/mod.rs -- 适配器 - BASIC - mod
// 2026/5/10 20:00 Updated.

////////

use port::basic::ColaBasicPort;
use std::sync::Arc;

////////
pub mod app; // 应用
pub mod bonding; // 绑定
pub mod node; // 节点
pub mod media;
pub mod session; // 会话
pub mod upload; // 通用上传

////////

/// # [BUILD] - 构建 BASIC Port
/// * `desc`: `COLA BASIC - Service Ports`
pub fn build_cola_basic_port() -> ColaBasicPort {
    ColaBasicPort {
        app: app::build_basic_app_port(),               // 应用
        node: node::build_basic_node_port(),              // 节点
        bonding: bonding::build_basic_bonding_port(),   // 绑定
        media: media::build_basic_meida_port(),         // 绑定
        //upload: Arc::new(upload::UploadSessionAdapter), // xx
    }
}

//////// END
