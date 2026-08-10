// repo_adapter/src/video/danmaku/list_port.rs  -- Port Adapter
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::port::danmaku::list::DanmakuListPort;

////////

/// # [ADAPTER] - danmaku list
#[derive(Debug, Default, Clone)]
pub struct danmakulistPortAdapter;

#[async_trait]
impl DanmakuListPort for danmakulistPortAdapter {
    // TODO: 瀹炵幇鍏蜂綋鐨勪笟鍔￠€昏緫
}

//////// END
