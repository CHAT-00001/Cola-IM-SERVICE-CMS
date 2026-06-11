// gate_http/src/models/home  --
// 2026/4/12 02:04 by wx: cestbon10080

//////

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// # 视频发布模型
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoPublishSchema {
    pub uuid: Uuid,                      // uuid v4
    pub uid: i64,                        // 作者uid
    pub title: String,                   // 标题
    pub title_at_uids: Option<Vec<i64>>, // 标题艾特的用户ids
    pub desc: Option<String>,            // 描述
    pub desc_at_uids: Option<Vec<i64>>,  // 描述艾特的用户ids
    pub thumb: String,                   // 封面a
    pub thumb_s: Option<String>,         // 水印封面
    pub href: String,                    // 视频链接a
    pub href_w: Option<String>,          // 视频链接b
    pub city: Option<String>,            // 城市
    pub lat: Option<f64>,                // 纬度
    pub lng: Option<f64>,                // 经度
    pub danmaku_swt: bool,               // 弹幕开关
    pub comment_swt: bool,               // 评论开关
    pub display_range: Option<i16>,      // 可见范围
}
