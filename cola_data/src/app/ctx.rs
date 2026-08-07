// cola_data/src/app/ctx.rs
// 数据中心 - APP - 全局应用上下文
// 2026/4/23 07:03

////////

use crate::auth::port::AuthServicePorts;
use crate::gis::port::ColaGisPort;
use crate::im::port::ColaImPort;
use crate::live::port::ColaLivePort;
use crate::market::port::ColaMarketPort;
use crate::music::port::ColaMusicPort;
use crate::three::port::ColaThreePort;
use crate::user::port::ColaUserPort;
use crate::video::port::ColaVideoPort;

////////

/// # [CONTEXT] - 应用上下文
/// * `desc`: `全局注入`
#[derive(Clone)]
pub struct AppContext {
    //pub api: AppServicePorts,
    pub auth: AuthServicePorts, // IAM验证中心
    // pub dynamic: ColaDynamicPort,  // 动态
    // pub fs: ColaFsPort,  // 文件存储
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
