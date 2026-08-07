// cola_three/src/case/binding.rs  -- 第三方 - 用例 - 绑定
// 2026/6/18 16:10

////////

use cola_data::app::data::AppData;
use cola_data::cola_three::command::binding::UpsertBindingCommand;
use cola_data::cola_three::port::binding::BindingPort;
use cola_data::cola_three::vo::binding::BindingVo;

////////

/// # [CASE] - 业务绑定用例
pub struct BindingCase;

impl BindingCase {
    ////////

    /// 1. 新增或更新
    pub async fn upsert(port: &dyn BindingPort, cmd: UpsertBindingCommand) -> AppData<BindingVo> {
        let info = match port.upsert(cmd.into()).await {
            Ok(info) => info,
            Err(e) => return AppData::err(500, &format!("操作失败: {}", e), None),
        };
        AppData::ok(info.into())
    }

    ////////

    /// 2. 列表
    pub async fn list(port: &dyn BindingPort) -> AppData<Vec<BindingVo>> {
        let list = match port.list().await {
            Ok(list) => list,
            Err(e) => return AppData::err(500, &format!("查询失败: {}", e), None),
        };
        AppData::ok(list.into_iter().map(|i| i.into()).collect())
    }

    ////////

    /// 3. 按业务模块+类型查询
    pub async fn find_by_biz(
        port: &dyn BindingPort,
        biz_module: &str,
        biz_type: &str,
    ) -> AppData<BindingVo> {
        let info = match port.find_by_biz(biz_module, biz_type).await {
            Ok(Some(info)) => info,
            Ok(None) => return AppData::err(404, "未找到该绑定", None),
            Err(e) => return AppData::err(500, &format!("查询失败: {}", e), None),
        };
        AppData::ok(info.into())
    }
}

//////// END
