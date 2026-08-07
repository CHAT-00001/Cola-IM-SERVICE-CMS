// cola_data/src/cola_three/command/binding  -- 数据中心 - 第三方 - command - 服务绑定
// 2026/6/18 16:40

////////

use crate::case::binding::BindingCase;
use cola_data::app::data::AppData;
use cola_data::cola_three::command::binding::UpsertBindingCommand;
use cola_data::cola_three::port::binding::BindingPort;
use cola_data::cola_three::vo::binding::BindingVo;

////////

/// # [API] - 绑定 API
pub struct BindingApi;

impl BindingApi {
    // 💡

    ////////

    /// # 1. [API HANDLER] - 创建
    pub async fn upsert(port: &dyn BindingPort, cmd: UpsertBindingCommand) -> AppData<BindingVo> {
        BindingCase::upsert(port, cmd).await
    }

    ////////

    /// # 2. [API HANDLER] - 列表
    pub async fn list(port: &dyn BindingPort) -> AppData<Vec<BindingVo>> {
        BindingCase::list(port).await
    }

    ////////

    /// # 3. [API HANDLER] - 根据业务查找
    pub async fn find_by_biz(
        port: &dyn BindingPort,
        biz_module: &str,
        biz_type: &str,
    ) -> AppData<BindingVo> {
        BindingCase::find_by_biz(port, biz_module, biz_type).await
    }
}

//////// END
