// repo_adapter/src/video/hotlist/get_port.rs  -- Port Adapter
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::port::hotlist::get::VideoHotlistGetPort;

////////

/// # [ADAPTER] - hotlist get
#[derive(Debug, Default, Clone)]
pub struct hotlistgetPortAdapter;

#[async_trait]
impl VideoHotlistGetPort for hotlistgetPortAdapter {
    // TODO: 瀹炵幇鍏蜂綋鐨勪笟鍔￠€昏緫
}

//////// END
