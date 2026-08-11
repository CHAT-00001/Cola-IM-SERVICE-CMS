// port/src/ctx.rs
// 🕳️ 端口 - CTX 全局应用上下文
// 2026/4/23 07:03

////////

use crate::cola_auth::AuthServicePorts;
use crate::cola_gis::ColaGisPort;
use crate::cola_im::ColaImPort;
use crate::cola_live::ColaLivePort;
use crate::market::ColaMarketPort;
use crate::cola_music::ColaMusicPort;
use crate::cola_three::ColaThreePort;
use crate::cola_user::ColaUserPort;
use crate::cola_video::ColaVideoPort;

////////

/// # [CONTEXT] - 应用上下文
/// * `desc`: `全局注入`
#[derive(Clone)]
pub struct AppContext {
    //pub api: AppServicePorts,
    pub auth: AuthServicePorts, // IAM验证中心
    // pub cola_dynamic: ColaDynamicPort,  // 动态
    // pub cola_fs: ColaFsPort,  // 文件存储
    pub gis: ColaGisPort,       // GIS
    pub live: ColaLivePort,     // 直播
    pub market: ColaMarketPort, // 市场
    pub music: ColaMusicPort,   // 音乐
    pub three: ColaThreePort,   // 三方
    pub user: ColaUserPort,     // 用户
    pub video: ColaVideoPort,   // 视频
    pub im: ColaImPort,         // 即时通讯
}

impl AppContext {
    pub fn default(
        auth: AuthServicePorts,
        gis: ColaGisPort,
        live: ColaLivePort,
        market: ColaMarketPort,
        music: ColaMusicPort,
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
