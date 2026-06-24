// cola_three/src/case/three_vendor.rs  -- 用例 - 厂商
// 2026/6/18

////////

use crate::model::command::three_vendor::VendorCommand;
use crate::model::vo::three_vendor::VendorVO;
use cola_data::three::port::three_vendor::VendorPort;
use cola_data::app::data::AppData;

////////

/// # [CASE] - 厂商用例
pub struct VendorCase;

impl VendorCase {

    ////////

    /// 1. 新增或更新
    pub async fn upsert(port: &dyn VendorPort, cmd: VendorCommand) -> AppData<VendorVO> {
        let info = match port.upsert(cmd.into()).await {
            Ok(info) => info,
            Err(e) => return AppData::err(5000, &format!("操作失败: {}", e), None),
        };
        AppData::ok(info.into())
    }

    ////////

    /// 2. 列表
    pub async fn list(port: &dyn VendorPort) -> AppData<Vec<VendorVO>> {
        let list = match port.list().await {
            Ok(list) => list,
            Err(e) => return AppData::err(5000, &format!("查询失败: {}", e), None),
        };
        AppData::ok(list.into_iter().map(|i| i.into()).collect())
    }

    ////////

    /// 3. 按 code 查询
    pub async fn find_by_code(port: &dyn VendorPort, code: &str) -> AppData<VendorVO> {
        let info = match port.find_by_code(code).await {
            Ok(Some(info)) => info,
            Ok(None) => return AppData::err(4004, "未找到该厂商", None),
            Err(e) => return AppData::err(5000, &format!("查询失败: {}", e), None),
        };
        AppData::ok(info.into())
    }
}


//////// END