// cola_gis/src/vo/poi.rs  -- GIS - model - vo - 短视频
// 2026/5/21 03:47

////////

use cola_data::app::page::PageInfo;
use cola_data::cola_gis::info::poi::PoiInfo;
use cola_data::cola_music::info::music::MusicInfo;
use cola_data::cola_user::info::user::UserInfo;
use serde::Serialize;

////////

/// # [MODEL] - 兴趣点视图模型
#[derive(Debug, Serialize, Clone)]
pub struct PoiVo {
    // 🌟 直接扁平化展开（Flatten），前端甚至感觉不到结构体拆分了！
    // 访问时依然是 new.id, new.title，不需要 new.info.title
    #[serde(flatten)]
    pub poi: PoiInfo,

    // 动态组装的关联数据
    pub user_info: UserInfo,
    pub music_info: MusicInfo,

    // 针对当前登录用户的动态交互显示
    pub is_visited: bool,
    pub is_like: bool,
    pub is_collect: bool,
    pub is_push: bool,
}

/// # [BUILD] - 构建兴趣点视图
impl PoiVo {
    /// 在 BIZ 层进行大聚合的构造函数
    pub fn combine(
        poi: PoiInfo,
        user: UserInfo,
        music: MusicInfo,
        // 也可以选择把交互状态作为参数传进来，或者提供 set 方法
    ) -> Self {
        Self {
            poi,
            user_info: user,
            music_info: music,
            is_visited: false,
            is_like: false,
            is_collect: false,
            is_push: false,
        }
    }
}

impl Default for PoiVo {
    fn default() -> Self {
        Self {
            poi: PoiInfo::empty(),
            user_info: UserInfo::default(),
            music_info: MusicInfo::default(),
            is_visited: false,
            is_like: false,
            is_collect: false,
            is_push: false,
        }
    }
}

/// # [RESPONSE] - 单兴趣点响应
#[derive(Debug, Serialize)]
pub struct PoiSingleResponse {
    pub info: PoiVo, // 吐给前端完美的、组装好的 VO 列表
}

/// # [RESPONSE] - 多兴趣点响应
#[derive(Debug, Serialize)]
pub struct PoiListResponse {
    pub list: Vec<PoiVo>, // 吐给前端完美的、组装好的 VO 列表
    pub page_info: PageInfo,
}

// 🌟🌟🌟 新增：最小侵入性扩展，为多视频响应提供空响应构造 🌟🌟🌟
impl PoiListResponse {
    /// ✅ 创建一个空的视频列表响应
    pub fn empty() -> Self {
        Self {
            list: Vec::new(),
            page_info: PageInfo::default(), // 借助 PageInfo 的 Default 规整分页
        }
    }
}

impl Default for PoiListResponse {
    fn default() -> Self {
        Self::empty()
    }
}

//////// END
