// repo_adapter/src/three/mod.rs
// 🔌 插头 - 可乐第三方 - 模块
// 2026/8/10 20:00 Updated.

////////

use port::cola_three::ColaThreePort;
use std::sync::Arc;

////////

pub mod three_app;
pub mod three_type;
pub mod three_vendor;
pub mod three_config;
pub mod three_biz_binding;

////////

/// # [BUILD] - 构建 THREE Port
/// * `desc`: 构建第三方 Port 聚合体，包含应用管理、类型、厂商、配置和绑定功能
pub fn build_three_port() -> ColaThreePort {
    ColaThreePort {
        app: Arc::new(three_app::AppAdapter),
        r#type: Arc::new(three_type::TypeAdapter),
        vendor: Arc::new(three_vendor::VendorAdapter),
        config: Arc::new(three_config::ConfigAdapter),
        binding: Arc::new(three_biz_binding::BindingAdapter),
    }
}

//////// END
