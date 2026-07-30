// cola_three/src/api/fs  -- API - 服务类型
// 2026/6/18

////////

use crate::case::category::TypeCase;
use cola_data::app::data::AppData;
use cola_data::three::command::category::ThreeServerTypeCommand;
use cola_data::three::port::category::TypePort;
use cola_data::three::vo::category::CategoryVo;

////////

/// # [API] - 服务类型 API
pub struct ServerTypeApi;

// 构造实现

impl ServerTypeApi {
    ////////

    /// # 1. [API] - 添加
    pub async fn upsert(port: &dyn TypePort, cmd: ThreeServerTypeCommand) -> AppData<CategoryVo> {
        TypeCase::upsert(port, cmd).await
    }

    ////////

    /// # 2. [API] - 列表
    pub async fn list(port: &dyn TypePort) -> AppData<Vec<CategoryVo>> {
        TypeCase::list(port).await
    }

    ////////

    /// # 3. [API] - 根据编码查找
    pub async fn find_by_code(port: &dyn TypePort, code: &str) -> AppData<CategoryVo> {
        TypeCase::find_by_code(port, code).await
    }
}

//////// END
