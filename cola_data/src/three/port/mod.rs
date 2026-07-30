// cola_data/src/three/port/mod.rs  -- THREE 端口 mod
// 2026/6/18

////////

use std::sync::Arc;
use crate::three::port::category::TypePort;
use crate::three::port::vendor::VendorPort;
use crate::three::port::config::ConfigPort;
use crate::three::port::binding::BindingPort;

////////

pub mod category;
pub mod vendor;
pub mod config;
pub mod binding;

////////

/// # [SERVICE] - 第三方服务 Port
#[derive(Clone)]
pub struct ColaThreePort {
    pub r#type: Arc<dyn TypePort + Send + Sync + 'static>,
    pub vendor: Arc<dyn VendorPort + Send + Sync + 'static>,
    pub config: Arc<dyn ConfigPort + Send + Sync + 'static>,
    pub binding: Arc<dyn BindingPort + Send + Sync + 'static>,
}

//////// END
