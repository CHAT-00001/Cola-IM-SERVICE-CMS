// port/src/cola_three/mod.rs
// ⏩️ 端口 - 可乐三方 - mod
// 2026/6/18

////////

use std::sync::Arc;
use crate::cola_three::app::AppPort;
use crate::cola_three::binding::BindingPort;
use crate::cola_three::category::TypePort;
use crate::cola_three::config::ConfigPort;
use crate::cola_three::vendor::VendorPort;

////////

pub mod app;
pub mod category;
pub mod vendor;
pub mod config;
pub mod binding;

////////

/// # [COLA THREE PORTS] -
/// * `desc`: `🍚可乐三方` - `第三方服务 Ports`
#[derive(Clone)]
pub struct ColaThreePort {
    pub app: Arc<dyn AppPort + Send + Sync + 'static>,
    pub r#type: Arc<dyn TypePort + Send + Sync + 'static>,
    pub vendor: Arc<dyn VendorPort + Send + Sync + 'static>,
    pub config: Arc<dyn ConfigPort + Send + Sync + 'static>,
    pub binding: Arc<dyn BindingPort + Send + Sync + 'static>,
}

//////// END
