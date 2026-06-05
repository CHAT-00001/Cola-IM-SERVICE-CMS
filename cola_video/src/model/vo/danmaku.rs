// cola_video/src/modelo/vo/danmaku  -- VIDEO - Model - VO - 弹幕
// 2026/5/22 15:23 by wx: cestbon10080

////////

use serde::Serialize;
use cola_data::app::page::PageInfo;
use crate::model::info::danmaku::DanmakuInfo;

////////

/// # [VO] - 弹幕 视图对象
#[derive(Serialize, Debug, Default)]
pub struct DanmakuVo {
    // 平铺弹幕
    #[serde(flatten)]
    pub Danmaku: DanmakuInfo, // 弹幕 元信息

    pub is_liked: bool,       // 是否点赞
    pub is_disliked: bool,    // 是否不喜欢
    pub is_author: bool,      // 是否是视频作者发的
    pub is_own: bool,         // 🚀 新增：是否是当前登录用户自己发的
}

/// # [BUILD] - 构造函数
impl DanmakuVo {
    /// 从已有的 DanmakuInfo 组装成最终的 VO 对象
    pub fn from_info(
        mut danmaku: DanmakuInfo,
        current_uid: Option<i64>,  // 🚀 传入当前登录用户的 UID (用来判断 is_own)
        video_author_id: i64,      // 🚀 传入视频作者的 UID (用来判断 is_author)
        is_liked: bool,            // 外部点赞状态
        is_disliked: bool,         // 外部不喜欢状态
    ) -> Self {
        // 1. 判断是不是当前登录用户自己发的弹幕
        let is_own = current_uid.map(|uid| uid == danmaku.user_id).unwrap_or(false);

        // 2. 判断是不是视频作者发的弹幕
        let is_author = danmaku.user_id == video_author_id;

        // 3. 同时对平铺的内部元信息进行安全修正（兼容你原本的内部逻辑）
        danmaku.is_author = is_author;

        Self {
            Danmaku: danmaku,
            is_liked,
            is_disliked,
            is_author,
            is_own,
        }
    }
}

/// # [RESPONSE] - 单条弹幕响应 (适用于：发布弹幕成功后返回、查看单条弹幕详情)
#[derive(Serialize, Debug)]
pub struct DanmakuSingleResponse {
    pub info: DanmakuVo,
}

/// # [RESPONSE] - 弹幕列表响应 (适用于：视频底下的弹幕区分页列表)
#[derive(Serialize, Debug)]
pub struct DanmakuListResponse {
    pub danmakus: Vec<DanmakuVo>, // 弹幕列表
    pub page_info: PageInfo,      // 分页信息
}

// * --------
//////// END
