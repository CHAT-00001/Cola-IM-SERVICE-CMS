// cola_three/src/api/three_config.rs  -- API - 配置
// 2026/6/18

//////

use crate::case::three_config::ConfigCase;
use crate::model::command::three_config::ConfigCommand;
use crate::model::vo::three_config::ConfigVO;
use cola_data::app::data::AppData;
use cola_data::three::port::three_config::ConfigPort;

//////

/// # [API] - 配置 API
pub struct ConfigApi;

impl ConfigApi {

    pub async fn upsert(port: &dyn ConfigPort, cmd: ConfigCommand) -> AppData<ConfigVO> {
        ConfigCase::upsert(port, cmd).await
    }

    pub async fn list_by_type(port: &dyn ConfigPort, type_id: i64) -> AppData<Vec<ConfigVO>> {
        ConfigCase::list_by_type(port, type_id).await
    }

    pub async fn find_by_id(port: &dyn ConfigPort, id: i64) -> AppData<ConfigVO> {
        ConfigCase::find_by_id(port, id).await
    }

    pub async fn find_binded(port: &dyn ConfigPort, biz_module: &str, biz_type: &str) -> AppData<ConfigVO> {
        ConfigCase::find_binded(port, biz_module, biz_type).await
    }
}
