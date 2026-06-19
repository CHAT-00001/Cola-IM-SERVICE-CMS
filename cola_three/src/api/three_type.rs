// cola_three/src/api/three_type.rs  -- API - 服务类型
// 2026/6/18

//////

use crate::case::three_type::TypeCase;
use crate::model::command::three_type::TypeCommand;
use crate::model::vo::three_type::TypeVO;
use cola_data::app::data::AppData;
use cola_data::three::port::three_type::TypePort;

//////

/// # [API] - 服务类型 API
pub struct TypeApi;

impl TypeApi {

    pub async fn upsert(port: &dyn TypePort, cmd: TypeCommand) -> AppData<TypeVO> {
        TypeCase::upsert(port, cmd).await
    }

    pub async fn list(port: &dyn TypePort) -> AppData<Vec<TypeVO>> {
        TypeCase::list(port).await
    }

    pub async fn find_by_code(port: &dyn TypePort, code: &str) -> AppData<TypeVO> {
        TypeCase::find_by_code(port, code).await
    }
}
