// repo_adapter/src/video/comment/report.rs  -- 🔌 插头 - 桩适配器
// 2026/8/8 13:05 Created.

use anyhow::Result;
use async_trait::async_trait;

pub struct StubAdapter;

#[async_trait]
impl cola_data::cola_video::port::video::*::Port for StubAdapter {
    // TODO: implement later
}

//////// END
