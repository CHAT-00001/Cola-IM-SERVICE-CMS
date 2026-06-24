// dynamic/goods.rs  -- 命令 商品
// 2026/6/18 13:28

////////

use chrono::{TimeZone, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 商品创建命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoodsCommand {
    pub name: String,               // 中文名称
    pub name_en: String,            // 英文名称
    pub no: String,                 // 货号
    pub category_id: i64,           // 分类ID
    pub price: String,              // 价格
    pub original_price: String,     // 原始价格
    pub thumbs: String,             // 封面图集
    pub content: String,            // 内容
    pub r#type: i16,                // 类型
    pub video_url: Option<String>,  // 视频地址
    pub video_length: Option<i16>,  // 视频长度
    pub commission: Option<String>, // 佣金
}

// 构造函数
impl Default for GoodsCommand {
    /// # 默认值兜底
    /// 当管理员只输入极少量信息时，构造一个标准的空商品模板
    fn default() -> Self {
        Self {
            name: "新商品".to_string(),
            name_en: "New Product".to_string(),
            no: "00-0000".to_string(),
            category_id: 0,
            price: "0.00".to_string(),
            original_price: "0.00".to_string(),
            thumbs: "".to_string(),
            content: "".to_string(),
            r#type: 0,
            video_url: None,
            video_length: None,
            commission: Some("0.00".to_string()),
        }
    }
}

// 构造函数
impl GoodsCommand {
    /// # 映射到数据库实体
    /// 这里会自动填补所有必要的默认值，防止数据库写入失败
    pub fn to_entity(&self, admin_id: i64) -> crate::market::entity::goods::GoodsEntity {
        let mut entity = crate::market::entity::goods::GoodsEntity::default();

        // 核心业务字段
        entity.name = self.name.clone();
        entity.name_en = self.name_en.clone();
        entity.one_classid = self.category_id;
        entity.present_price = Some(self.price.clone());
        entity.original_price = Some(self.original_price.clone());
        entity.thumbs = self.thumbs.clone();
        entity.content = self.content.clone();
        entity.r#type = self.r#type;

        // 填充兜底值或可选值
        entity.video_url = self.video_url.clone().unwrap_or_default();
        entity.commission = self.commission.clone().or(Some("0.00".to_string()));
        entity.admin_id = Some(admin_id.to_string());

        // 机器时间戳
        let now = chrono::Utc::now().timestamp() as i32;
        entity.add_time = now;
        entity.upd_time = now;

        entity
    }

    /// # 数据校验
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.name.trim().is_empty() {
            return Err(anyhow::anyhow!("商品名称必填"));
        }
        if self.price.parse::<f64>().unwrap_or(0.0) <= 0.0 {
            return Err(anyhow::anyhow!("商品价格必须大于0"));
        }
        Ok(())
    }
}
