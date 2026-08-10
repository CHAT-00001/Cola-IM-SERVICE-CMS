// repo_adapter/src/video/hotlist/check_port.rs  -- Port Adapter
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::port::hotlist::check::VideoHotlistCheckPort;

////////

/// # [ADAPTER] - hotlist check
#[derive(Debug, Default, Clone)]
pub struct hotlistcheckPortAdapter;

#[async_trait]
impl VideoHotlistCheckPort for hotlistcheckPortAdapter {
    // TODO: 瀹炵幇鍏蜂綋鐨勪笟鍔￠€昏緫
}

//////// END
