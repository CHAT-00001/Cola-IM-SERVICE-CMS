// // cola_data/src/cola_dynamic/vo/cola_dynamic.rs  -- DYNAMIC - Model - 动态响应体
// // 2026/5/22 20:08 by wx: cestbon10080
// // * --------
// // * --------
//
// ////////
//
// use crate::cola_dynamic::info::cola_dynamic::DynamicInfo;
// use crate::user::info::music::UserInfo;
// use serde::{Deserialize, Serialize};
// use crate::app::page::PageInfo;
// ////////
//
// /// # [VO] - 动态 - 视图对象
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct DynamicVo {
//     // 平铺动态信息
//     #[serde(flatten)]
//     pub dynamic_info: DynamicInfo, // ID
//     pub user_info: UserInfo,       // 作者id
//     pub is_liked: bool,            // 是否点赞
//     pub is_collect: bool,          // 是否收藏
// }
//
// ////////
//
// /// # [RESPONSE] -动态单个返回体
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct DynamicSingleResponse {
//     pub info: DynamicVo,
// }
//
//
// /// # [RESPONSE] - 动态列表响应体
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct DynamicListResponse {
//     pub dynamics: Vec<DynamicVo>,
//     pub page_info: PageInfo,
// }
//
//
// //////// END
