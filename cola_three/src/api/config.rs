// cola_three/src/api/config.rs
// 可乐三方 - API - 配置
// 2026/6/18

////////

use crate::case::config::ConfigCase;
use cola_data::app::data::AppData;
use cola_data::cola_three::command::config::UpsertConfigCommand;
use cola_data::cola_three::vo::config::ConfigVo;
use port::cola_three::config::ConfigPort;

////////

/// # [API] - 配置 API
pub struct ConfigApi;

impl ConfigApi {
    //

    ////////

    /// # 1. [API] - 更插
    /// * `desc`: `🗣 ADMIN - 更新/插入` - `新的配置`
    /// * `condition`: `⚠️ ADMIN / `
    pub async fn upsert(port: &dyn ConfigPort, cmd: UpsertConfigCommand) -> AppData<ConfigVo> {
        ConfigCase::upsert(port, cmd).await
    }

    pub async fn list_by_type(port: &dyn ConfigPort, type_id: i64) -> AppData<Vec<ConfigVo>> {
        ConfigCase::list_by_type(port, type_id).await
    }

    pub async fn find_by_id(port: &dyn ConfigPort, id: i64) -> AppData<ConfigVo> {
        ConfigCase::find_by_id(port, id).await
    }

    pub async fn find_binded(
        port: &dyn ConfigPort,
        biz_module: &str,
        biz_type: &str,
    ) -> AppData<ConfigVo> {
        ConfigCase::find_binded(port, biz_module, biz_type).await
    }
}

//////// END
