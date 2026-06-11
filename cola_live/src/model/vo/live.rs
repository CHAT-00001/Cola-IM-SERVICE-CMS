// // cola_live/src/vo/vo/live  -- LIVE - 视图模型
// // 2026/5/19 21:17 by wx: cestbon10080
//
// ////////
//
// use serde::{Deserialize, Serialize};
// use cola_data::api::page::PageInfo;
// use crate::vo::info::live::UserInfo;
// ////////
//
// /// # [VO] - 音乐 - 视图模型
// #[derive(Serialize, Deserialize, Debug, Clone, Default)]
// pub struct LiveVo {
//     pub author_info: Option<UserInfo>, // 动态组装作者信息
//     pub is_liked: bool,                // 是否点赞
//     pub is_disliked: bool,             // 是否不喜欢
//     pub is_collect: bool,              // 是否收藏
// }
//
// /// # 构造音乐视图模型
// impl LiveVo {
//
//     /// # 1. 显式定义 empty 方法 (彻底解决 E0599 报错)
//     /// 当视频完全没有音乐信息，且也不想载入作者信息时的全套异常兜底
//     pub fn empty() -> Self {
//         Self {
//
//             author_info: None,
//             is_liked: false,
//             is_disliked: false,
//             is_collect: false,
//         }
//     }
//
//     /// # 2. 自动载入逻辑 (from_user)
//     /// 当临时提取原声或特殊场景下，传入 UserInfo，自动拼装出带作者元信息的视图模型
//     pub fn from_user(user: UserInfo) -> Self {
//         let now_timestamp = chrono::Utc::now().timestamp();
//
//         // 🌟 将原本错乱的字段，规范地组装进 MusicInfo 内部
//
//         Self {
//             author_info: Some(user), // 🌟 自动载入整个用户模型
//             ..Self::default()        // 交互状态（is_liked等）走 Default::default() 即为 false
//         }
//     }
// }
//
// /// # [RESPONSE] - 单音乐响应
// #[derive(Debug, Serialize)]
// pub struct MusicSingleResponse {
//     pub info: MusicVo, // 吐给前端完美的、组装好的单体 VO
// }
//
// /// # [RESPONSE] - 多音乐响应
// #[derive(Debug, Serialize)]
// pub struct MusicListResponse {
//     pub list: Vec<LiveVo>, // 吐给前端完美的、组装好的 VO 列表
//     pub page_info: PageInfo,
// }
//
// //////// END