// cola_date/src/app/ctx.rs  -- 全局应用上下文
// 2026/4/23 07:03 by wx: cestbon10080

////////

use crate::auth::port::AuthServicePorts;
use crate::live::port::ColaLivePort;
use crate::music::port::MusicServicePorts;
use crate::user::port::ColaUserPort;
use crate::video::port::ColaVideoPort;

////////


/// # [CONTEXT] - 应用上下文
/// * `描述` - 全局注入
#[derive(Clone)]
pub struct AppContext {
    //pub app: AppServicePorts,
    pub auth: AuthServicePorts,
   // pub dynamic: DynamicServicePorts,
    pub live: ColaLivePort,
    pub music: MusicServicePorts,
    pub user: ColaUserPort,
    pub video: ColaVideoPort,
}
