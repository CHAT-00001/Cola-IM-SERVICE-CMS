// repo_adapter/src/cola_gis/add
// 🔌 适配器 - 可乐GIS - POI - 发布
// 2026-07-07 14:00 Created.

////////

use async_trait::async_trait;
use port::cola_gis::buy::BuyRepo;
use repository::cola_gis::service::poi_add::PoiAddService;

////////

/// # [BUY PORT] - 购买 端口 插头
pub struct BuyPortAdapter;

////////

#[async_trait]
impl BuyRepo for BuyPortAdapter {
    // 💡

    ////////

    /// # 1. [PORT] - 保存购买记录
    async fn save_buy_record(
        &self,
        uid: i64,
        poi_id: i64,
    ) -> anyhow::Result<()> {
        // TODO: implement actual buy logic with GIS service
        Ok(())
    }

    ////////

    /// # 2. [PORT] - 删除购买记录
    async fn del_buy_record(
        &self,
        _uid: i64,
        _poi_id: i64,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    ////////

    /// # 3. [PORT] - 根据用户ID获取购买记录IDs
    async fn get_buy_ids_by_user_id(
        &self,
        _user_id: i64,
        _offset: i64,
        _limit: i64,
    ) -> anyhow::Result<Vec<i64>> {
        Ok(vec![])
    }
}

//////// END