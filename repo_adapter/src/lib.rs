// repo_adapter/src/lib.rs
// 🔌 适配器 - Adapter - Service layer adapter implementation - lib
// 2026/8/10 20:00 Updated.

////////

use port::app::ctx::AppContext;
use std::sync::Arc;

////////

pub mod auth; // Auth Center
pub mod dynamic; // Dynamic
pub mod gift; // Gift
pub mod gis; // GIS
pub mod im; // Instant Messaging
pub mod live; // Live
pub mod market; // Market
pub mod music; // 音乐
pub mod photo; // Photo
pub mod three; // Third-party
pub mod user; // User
pub mod video; // Video
pub mod wallet; // Wallet


////////

// Temporary Stub Adapter - Used as compilation placeholder, delete after implementing actual adapter
pub mod stub {
    use anyhow::Result;
    use async_trait::async_trait;

    #[derive(Debug, Default, Clone)]
    pub struct GeneralStubAdapter;

    // Common stub implementation (all methods return errors or empty values)
    macro_rules! stub_impl {
        ($trait:path) => {
            #[async_trait]
            impl $trait for GeneralStubAdapter {}
        };
    }
}

////////

/// # [BUILD] - Build APP Context
/// * `desc`: Aggregate all business module Ports by calling each module's builder function
pub fn build_app_context() -> AppContext {
    ////////

    let auth = auth::build_auth_port();
    let gis = gis::build_gis_port();
    let live = live::build_live_port();
    let market = market::build_market_port();
    let music = music::build_music_port();
    let three = three::build_three_port();
    let user = user::build_user_port();
    let video = video::build_video_port();
    let im = im::build_im_port();

    ////////

    AppContext::default(auth, gis, live, market, music, three, user, video, im)
}

//////// END
