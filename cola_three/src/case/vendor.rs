// cola_three/src/case/vendor.rs
// core - 第三方 - 用例 - 厂商
// 2026/6/18

////////

use cola_data::app::data::AppData;
use cola_data::cola_three::command::vendor::UpsertVendorCommand;
use cola_data::cola_three::vo::vendor::VendorVo;
use port::cola_three::vendor::VendorPort;

////////

/// # [CASE] - 厂商用例
pub struct VendorCase;

impl VendorCase {
    ////////

    /// 1. 新增或更新
    pub async fn upsert(port: &dyn VendorPort, cmd: UpsertVendorCommand) -> AppData<VendorVo> {
        let info = match port.upsert(cmd.into()).await {
            Ok(info) => info,
            Err(e) => return AppData::err(5000, &format!("操作失败: {}", e), None),
        };
        AppData::ok(info.into())
    }

    ////////

    /// 2. 列表
    pub async fn list(port: &dyn VendorPort) -> AppData<Vec<VendorVo>> {
        let list = match port.list().await {
            Ok(list) => list,
            Err(e) => return AppData::err(5000, &format!("查询失败: {}", e), None),
        };
        AppData::ok(list.into_iter().map(|i| i.into()).collect())
    }

    ////////

    /// 3. 按 code 查询
    pub async fn find_by_code(port: &dyn VendorPort, code: &str) -> AppData<VendorVo> {
        let info = match port.find_by_code(code).await {
            Ok(Some(info)) => info,
            Ok(None) => return AppData::err(4004, "未找到该厂商", None),
            Err(e) => return AppData::err(5000, &format!("查询失败: {}", e), None),
        };
        AppData::ok(info.into())
    }
}

//////// END
