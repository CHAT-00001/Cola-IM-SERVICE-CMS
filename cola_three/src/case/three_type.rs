// cola_three/src/case/three_type.rs  -- 用例 - 服务类型
// 2026/6/18

//////

use crate::model::command::three_type::TypeCommand;
use crate::model::vo::three_type::TypeVO;
use cola_data::three::port::three_type::TypePort;
use cola_data::app::data::AppData;

//////

/// # [CASE] - 服务类型用例
pub struct TypeCase;

impl TypeCase {

    /// 1. 新增或更新
    pub async fn upsert(port: &dyn TypePort, cmd: TypeCommand) -> AppData<TypeVO> {
        let info = match port.upsert(cmd.into()).await {
            Ok(info) => info,
            Err(e) => return AppData::err(500, &format!("操作失败: {}", e), None),
        };
        AppData::ok(info.into())
    }

    /// 2. 列表
    pub async fn list(port: &dyn TypePort) -> AppData<Vec<TypeVO>> {
        let list = match port.list().await {
            Ok(list) => list,
            Err(e) => return AppData::err(500, &format!("查询失败: {}", e), None),
        };
        AppData::ok(list.into_iter().map(|i| i.into()).collect())
    }

    /// 3. 按 code 查询
    pub async fn find_by_code(port: &dyn TypePort, code: &str) -> AppData<TypeVO> {
        let info = match port.find_by_code(code).await {
            Ok(Some(info)) => info,
            Ok(None) => return AppData::err(404, "未找到该服务类型", None),
            Err(e) => return AppData::err(500, &format!("查询失败: {}", e), None),
        };
        AppData::ok(info.into())
    }
}
