// data/src/market/info/goods/view.rs
// 响应 - MARKET - info - 商品 - 浏览记录详情
// 2026/8/11 22:09 Created.

////////

use crate::cola_market::entity::goods::view::GoodsViewEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [DTO] - 商品浏览记录 - 详情响应 (INFO)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GoodsViewInfo {
    pub id: i64,
    pub uuid: Option<String>,
    pub uid: i64,
    pub goods_id: i64,
    pub client_ip: Option<String>,
    pub ip_area: Option<String>,
    pub client_type: Option<i16>,
    pub device_os: Option<String>,
    pub device_model: Option<String>,
    pub status: i16,
    pub created_at: Option<DateTime<Utc>>,
}

// -----------------------------------------------------------------
// 类型转换实现 (Entity -> Info)
// -----------------------------------------------------------------

impl From<GoodsViewEntity> for GoodsViewInfo {
    fn from(entity: GoodsViewEntity) -> Self {
        Self {
            id: entity.id,
            uuid: entity.uuid,
            uid: entity.uid,
            goods_id: entity.goods_id,
            client_ip: entity.client_ip,
            ip_area: entity.ip_area,
            client_type: entity.client_type,
            device_os: entity.device_os,
            device_model: entity.device_model,
            status: entity.status,
            created_at: entity.created_at,
        }
    }
}

//////// END
