// /vo/danmaku  -- VO - 弹幕
// 2026-07-07

//////

use serde::Serialize;
use cola_data::app::page::PageInfo;
use cola_data::cola_gis::info::danmaku::PoiDanmakuInfo;

//////

/// # [VO] - 弹幕 视图对象
#[derive(Serialize, Debug)]
pub struct DanmakuVo {
    #[serde(flatten)]
    pub danmaku: PoiDanmakuInfo,
    pub is_liked: bool,
    pub is_disliked: bool,
    pub is_author: bool,
    pub is_own: bool,
}

/// # [BUILD] - 构造函数
impl DanmakuVo {
    /// 从已有的 PoiDanmakuInfo 组装成最终的 VO 对象
    pub fn from_info(
        danmaku: PoiDanmakuInfo,
        current_uid: Option<i64>,
        video_author_id: i64,
        is_liked: bool,
        is_disliked: bool,
    ) -> Self {
        let is_own = current_uid.map(|uid| uid == danmaku.user_id).unwrap_or(false);
        let is_author = danmaku.user_id == video_author_id;

        Self {
            danmaku,
            is_liked,
            is_disliked,
            is_author,
            is_own,
        }
    }
}

/// # [RESPONSE] - 单条弹幕响应
#[derive(Serialize, Debug)]
pub struct DanmakuSingleResponse {
    pub info: DanmakuVo,
}

/// # [RESPONSE] - 弹幕列表响应
#[derive(Serialize, Debug)]
pub struct DanmakuListResponse {
    pub danmakus: Vec<DanmakuVo>,
    pub page_info: PageInfo,
}

////// END