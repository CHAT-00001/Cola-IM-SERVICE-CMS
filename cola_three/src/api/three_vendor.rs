// cola_three/src/api/three_vendor.rs  -- API - 厂商
// 2026/6/18

//////

use crate::case::three_vendor::VendorCase;
use crate::model::command::three_vendor::VendorCommand;
use crate::model::vo::three_vendor::VendorVO;
use cola_data::app::data::AppData;
use cola_data::three::port::three_vendor::VendorPort;

//////

/// # [API] - 厂商 API
pub struct VendorApi;

impl VendorApi {

    pub async fn upsert(port: &dyn VendorPort, cmd: VendorCommand) -> AppData<VendorVO> {
        VendorCase::upsert(port, cmd).await
    }

    pub async fn list(port: &dyn VendorPort) -> AppData<Vec<VendorVO>> {
        VendorCase::list(port).await
    }

    pub async fn find_by_code(port: &dyn VendorPort, code: &str) -> AppData<VendorVO> {
        VendorCase::find_by_code(port, code).await
    }
}
