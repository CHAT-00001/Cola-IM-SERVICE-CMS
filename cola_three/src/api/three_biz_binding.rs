// cola_three/src/api/three_biz_binding.rs  -- API - 绑定
// 2026/6/18 16:40

////////

use crate::case::three_biz_binding::BindingCase;
use crate::model::command::three_biz_binding::BindingCommand;
use crate::model::vo::three_biz_binding::BindingVO;
use cola_data::app::data::AppData;
use cola_data::three::port::three_biz_binding::BindingPort;

////////

/// # [API] - 绑定 API
pub struct BindingApi;

impl BindingApi {

    pub async fn upsert(port: &dyn BindingPort, cmd: BindingCommand) -> AppData<BindingVO> {
        BindingCase::upsert(port, cmd).await
    }

    pub async fn list(port: &dyn BindingPort) -> AppData<Vec<BindingVO>> {
        BindingCase::list(port).await
    }

    pub async fn find_by_biz(port: &dyn BindingPort, biz_module: &str, biz_type: &str) -> AppData<BindingVO> {
        BindingCase::find_by_biz(port, biz_module, biz_type).await
    }
}


//////// END