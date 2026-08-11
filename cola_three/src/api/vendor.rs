// cola_three/src/api/vendor.rs
// 三方 - API - 厂商
// 2026/6/18

////////

use crate::case::vendor::VendorCase;
use cola_data::app::data::AppData;
use cola_data::cola_three::command::vendor::UpsertVendorCommand;
use cola_data::cola_three::vo::vendor::VendorVo;
use port::cola_three::vendor::VendorPort;

////////

/// # [API] - 厂商 API
pub struct VendorApi;

impl VendorApi {
    //

    ///////

    /// # 1. [API HANDLER] - 插入
    /// * `desc`: `添加厂商`
    /// * `condition`: `⚠️ ADMIN / REVIEWER`
    pub async fn upsert(port: &dyn VendorPort, cmd: UpsertVendorCommand) -> AppData<VendorVo> {
        VendorCase::upsert(port, cmd).await
    }

    ///////

    /// # 2. [API HANDLER] - 列表
    /// * `desc`: `添加厂商`
    /// * `condition`: `⚠️ ADMIN / REVIEWER`
    pub async fn list(port: &dyn VendorPort) -> AppData<Vec<VendorVo>> {
        VendorCase::list(port).await
    }

    ///////

    /// # 3. [API HANDLER] - 查找
    /// * `desc`: `添加厂商`
    /// * `condition`: `⚠️ ADMIN / REVIEWER`
    pub async fn find_by_code(port: &dyn VendorPort, code: &str) -> AppData<VendorVo> {
        VendorCase::find_by_code(port, code).await
    }
}

//////// END
