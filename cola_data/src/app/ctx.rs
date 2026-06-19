// cola_date/src/api/ctx.rs  -- 全局应用上下文
// 2026/4/23 07:03 by wx: cestbon10080

//////

use crate::auth::port::AuthServicePorts;
use crate::live::port::ColaLivePort;
use crate::market::port::ColaMarketPort;
use crate::music::port::MusicServicePorts;
use crate::three::port::ColaThreePort;
use crate::user::port::ColaUserPort;
use crate::video::port::ColaVideoPort;

//////

/// # [CONTEXT] - 应用上下文
/// * `描述` - 全局注入
#[derive(Clone)]
pub struct AppContext {
    //pub api: AppServicePorts,
    pub auth: AuthServicePorts,
    // pub dynamic: DynamicServicePorts,
    pub live: ColaLivePort,
    pub market: ColaMarketPort,
    pub music: MusicServicePorts,
    pub three: ColaThreePort,
    pub user: ColaUserPort,
    pub video: ColaVideoPort,
}

impl AppContext {
    pub fn default(
        auth: AuthServicePorts,
        live: ColaLivePort,
        market: ColaMarketPort,
        music: MusicServicePorts,
        three: ColaThreePort,
        user: ColaUserPort,
        video: ColaVideoPort,
    ) -> Self {
        Self {
            auth,
            live,
            market,
            music,
            three,
            user,
            video,
        }
    }
}
