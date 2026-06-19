// cola_data/src/market/handler/goods.rs  -- 可乐数据中心 - MARKET - handler - 商品表
// 2026/6/18 10:14

////////

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

////////

/// # [ENTITY] - 市场 - 商品表
/// * table name: shop_goods
#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
pub struct GoodsEntity {
    pub id: i64,                          // id
    pub uid: i64,                         // 操作者用户ID
    pub market_id: i64,                   // 市场ID
    pub name: String,                     // 市场ID
    pub name_en: String,                  // 市场ID
    pub no: String,                       // 市场ID
    pub one_classid: i64,                 // 一级分类
    pub two_classid: i64,                 // 二级分类
    pub three_classid: i64,               // 三级分类
    pub video_url: String,                // 视频地址
    pub video_thumb: String,              // 视频封面
    pub video_length: i32,                // 视频长度
    pub thumbs: String,                   // 封面
    pub content: String,                  // 内容
    pub pictures: String,                 // 内容图集
    pub specs: String,                    // 商品规格
    pub postage: i32,                     // 邮费
    pub hits: i32,                        // 点击数
    pub isrecom: i16,                     // 推荐，0否1是
    pub sale_nums: i32,                   // 总销量
    pub refuse_reason: String,            // 商品拒绝原因
    pub issale: i16,                      // 商品是否在直播间销售 0 否 1 是(针对用户自己发布的商品)
    pub r#type: i16,                      // 类型: 0. 站内 1. 站外 2. 自营 3. 虚拟
    pub original_price: Option<String>,   // 站外商品原价
    pub present_price: Option<String>,    // 站外商品现价
    pub goods_desc: Option<String>,       // 站外商品简介
    pub href: Option<String>,             // 站外商品链接
    pub live_isshow: Option<String>,      // 直播间是否展示商品简介 0 否 1 是 默认0
    pub low_price: Option<String>,        // 站外商品最低价
    pub admin_id: Option<String>,         // 发布自营商品的管理员id
    pub commission: Option<String>,       // 主播代卖平台商品的佣金
    pub share_income: Option<String>,     // 分享给其他用户购买后获得的佣金
    pub lat: String,                      // 纬度
    pub lng: String,                      // 经度
    pub city: Option<String>,             // 城市
    pub address: Option<String>,          // 地址
    pub label_id: i32,                    // 标签ID
    pub collects: i32,                    // 被收藏数量
    pub shares: i32,                      // 被分享数量
    pub add_time: i32,                    // 添加时间（机器）
    pub upd_time: i32,                    // 更新时间（机器）
    pub create_at: Option<DateTime<Utc>>, // 创建时间（人类）
    pub update_at: Option<DateTime<Utc>>, // 更新时间（人类）
}
