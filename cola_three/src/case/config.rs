// cola_three/src/case/config  -- 用例 - 配置
// 2026/6/18 16:01

////////

use cola_data::app::data::AppData;
use cola_data::cola_three::command::config::UpsertConfigCommand;
use cola_data::cola_three::port::config::ConfigPort;
use cola_data::cola_three::vo::config::ConfigVo;

////////

/// # [CASE] - 配置用例
pub struct ConfigCase;

impl ConfigCase {
    ////////

    /// 1. 新增或更新
    pub async fn upsert(port: &dyn ConfigPort, cmd: UpsertConfigCommand) -> AppData<ConfigVo> {
        let info = match port.upsert(cmd.into()).await {
            Ok(info) => info,
            Err(e) => return AppData::err(500, &format!("操作失败: {}", e), None),
        };
        AppData::ok(info.into())
    }

    ////////

    /// 2. 按类型查询列表
    pub async fn list_by_type(port: &dyn ConfigPort, type_id: i64) -> AppData<Vec<ConfigVo>> {
        let list = match port.list_by_type(type_id).await {
            Ok(list) => list,
            Err(e) => return AppData::err(500, &format!("查询失败: {}", e), None),
        };
        AppData::ok(list.into_iter().map(|i| i.into()).collect())
    }

    ////////

    /// 3. 按 ID 查询
    pub async fn find_by_id(port: &dyn ConfigPort, id: i64) -> AppData<ConfigVo> {
        let info = match port.find_by_id(id).await {
            Ok(Some(info)) => info,
            Ok(None) => return AppData::err(404, "未找到该配置", None),
            Err(e) => return AppData::err(500, &format!("查询失败: {}", e), None),
        };
        AppData::ok(info.into())
    }

    ////////

    /// 4. 查询绑定配置
    pub async fn find_binded(
        port: &dyn ConfigPort,
        biz_module: &str,
        biz_type: &str,
    ) -> AppData<ConfigVo> {
        let info = match port.find_binded(biz_module, biz_type).await {
            Ok(Some(info)) => info,
            Ok(None) => return AppData::err(404, "未找到绑定配置", None),
            Err(e) => return AppData::err(500, &format!("查询失败: {}", e), None),
        };
        AppData::ok(info.into())
    }
}

//////// END
