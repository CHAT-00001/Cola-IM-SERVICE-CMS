// repo_adapter/src/video/hotlist/list_port.rs  -- Port Adapter
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::port::hotlist::list::VideoHotlistListPort;

////////

/// # [ADAPTER] - hotlist list
#[derive(Debug, Default, Clone)]
pub struct hotlistlistPortAdapter;

#[async_trait]
impl VideoHotlistListPort for hotlistlistPortAdapter {
    // TODO: 瀹炵幇鍏蜂綋鐨勪笟鍔￠€昏緫
}

//////// END
