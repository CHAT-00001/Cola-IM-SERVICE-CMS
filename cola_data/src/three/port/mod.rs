// cola_data/src/three/port/mod.rs  -- THREE 端口 mod
// 2026/6/18

use std::sync::Arc;
use crate::three::port::three_type::TypePort;
use crate::three::port::three_vendor::VendorPort;
use crate::three::port::three_config::ConfigPort;
use crate::three::port::three_biz_binding::BindingPort;

pub mod three_type;
pub mod three_vendor;
pub mod three_config;
pub mod three_biz_binding;

//////

/// # [SERVICE] - 第三方服务 Port
#[derive(Clone)]
pub struct ColaThreePort {
    pub r#type: Arc<dyn TypePort + Send + Sync + 'static>,
    pub vendor: Arc<dyn VendorPort + Send + Sync + 'static>,
    pub config: Arc<dyn ConfigPort + Send + Sync + 'static>,
    pub binding: Arc<dyn BindingPort + Send + Sync + 'static>,
}
