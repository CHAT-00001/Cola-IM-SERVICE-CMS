// cola_data/src/app/ctx.rs  -- 数据 - APP - 全局应用上下文
// 2026/4/23 07:03

////////

use crate::auth::port::AuthServicePorts;
use crate::gis::port::ColaGisPort;
use crate::im::port::ColaImPort;
use crate::live::port::ColaLivePort;
use crate::market::port::ColaMarketPort;
use crate::music::port::MusicServicePorts;
use crate::three::port::ColaThreePort;
use crate::user::port::ColaUserPort;
use crate::video::port::ColaVideoPort;

////////

/// # [CONTEXT] - 应用上下文
/// * `描述` - 全局注入
#[derive(Clone)]
pub struct AppContext {
    //pub api: AppServicePorts,
    pub auth: AuthServicePorts,
    // pub dynamic: DynamicServicePorts,
    pub gis: ColaGisPort,
    pub live: ColaLivePort,
    pub market: ColaMarketPort,
    pub music: MusicServicePorts,
    pub three: ColaThreePort,
    pub user: ColaUserPort,
    pub video: ColaVideoPort,
    pub im: ColaImPort,
}

impl AppContext {
    pub fn default(
        auth: AuthServicePorts,
        gis: ColaGisPort,
        live: ColaLivePort,
        market: ColaMarketPort,
        music: MusicServicePorts,
        three: ColaThreePort,
        user: ColaUserPort,
        video: ColaVideoPort,
        im: ColaImPort,
    ) -> Self {
        Self {
            auth,
            gis,
            live,
            market,
            music,
            three,
            user,
            video,
            im,
        }
    }
}

//////// END