// repo_adapter/src/video/hotlist/del_port.rs  -- Port Adapter
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::port::hotlist::del::VideoHotlistDelPort;

////////

/// # [ADAPTER] - hotlist del
#[derive(Debug, Default, Clone)]
pub struct hotlistdelPortAdapter;

#[async_trait]
impl VideoHotlistDelPort for hotlistdelPortAdapter {
    // TODO: 瀹炵幇鍏蜂綋鐨勪笟鍔￠€昏緫
}

//////// END
