// repo_adapter/src/video/cola_video/add.rs
// 🔌 插头 - VIDEO - 视频 - 发布服务
// 2026-06-12 10:52 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_market::command::goods::GoodsCommand;
use cola_data::cola_market::info::express::express::ExpressInfo;
use cola_data::cola_video::command::video::edit::VideoUpdateCommand;
use cola_data::cola_video::command::video::new::VideoNewCommand;
use cola_data::cola_video::command::video::permission::VideoUpdatePermissionCommand;
use port::market::express::add::ExpressAddPort;
use port::cola_video::video::add::VideoAddPort;

////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `🔌 视频发布插头`
pub struct ExpressAddAdapter;

#[async_trait]
impl ExpressAddPort for ExpressAddAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 发布新视频
    async fn save_express(&self, uid: i64, cmd: GoodsCommand) -> Result<(ExpressInfo)> {
        todo!()
    }

    async fn update_express(&self, uid: i64, express_id: i64, cmd: GoodsCommand) -> Result<(ExpressInfo)> {
        todo!()
    }

    async fn change_status(&self, express_id: i64, status_code: i16) -> Result<()> {
        todo!()
    }

}

//////// END
