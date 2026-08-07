// cola_three/src/case/server_type.rs  --第三方 -  用例 - 服务类型
// 2026/6/18 16:40

////////

use cola_data::app::data::AppData;
use cola_data::cola_three::command::category::{ThreeServerTypeCommand};
use cola_data::cola_three::port::category::TypePort;
use cola_data::cola_three::vo::category::CategoryVo;

////////

/// # [CASE] - 服务类型用例
pub struct TypeCase;

impl TypeCase {
    // 💡

    ////////

    /// # 1. [API HANDLER] - 新增或更新
    pub async fn upsert(port: &dyn TypePort, cmd: ThreeServerTypeCommand) -> AppData<CategoryVo> {
        let info = match port.upsert(cmd.into()).await {
            Ok(info) => info,
            Err(e) => return AppData::err(500, &format!("操作失败: {}", e), None),
        };
        AppData::ok(info.into())
    }

    ////////

    /// # 2. [API HANDLER] - 列表
    pub async fn list(port: &dyn TypePort) -> AppData<Vec<CategoryVo>> {
        let list = match port.list().await {
            Ok(list) => list,
            Err(e) => return AppData::err(500, &format!("查询失败: {}", e), None),
        };
        AppData::ok(list.into_iter().map(|i| i.into()).collect())
    }

    ////////

    /// # 3. [API HANDLER] - 按 code 查询
    pub async fn find_by_code(port: &dyn TypePort, code: &str) -> AppData<CategoryVo> {
        let info = match port.find_by_code(code).await {
            Ok(Some(info)) => info,
            Ok(None) => return AppData::err(404, "未找到该服务类型", None),
            Err(e) => return AppData::err(500, &format!("查询失败: {}", e), None),
        };
        AppData::ok(info.into())
    }
}

//////// END
