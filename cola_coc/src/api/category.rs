// // /cola_fs.rs  --
// // 2026/5/20 02:00
//
// ////////
//
// use cola_data::api::data::AppData;
// use cola_data::api::request::ApiUrlParamsQuery;
// use crate::case;
// use crate::case::cola_fs::logic_check_category_status;
// use crate::model::vo::new::VideoListResponse;
//
// ////////
//
// /// # 1. [APP USE CASE] - 获取分类
// pub async fn case_get_category(
//     query: ApiUrlParamsQuery,
//     category_id: i16,
// ) -> AppData<VideoListResponse> {
//     // 1. 检查分类ID是否有效
//     if category_id <= 0 {
//         return AppData::err(4002, "参数错误：非法的 category_id", None);
//     }
//
//     // 2. 调用具体的业务校验函数
//     if !logic_check_category_status(category_id).await {
//         return AppData::err(4004, "参数错误：分类ID不存在", None);
//     }
//
//     // 3. 执行业务逻辑
//     match case::home::logic_get_category_list(
//         query.clone(), category_id,
//
//     ).await {
//         Ok(resp) => AppData::ok(resp),
//         Err(e) => {
//             tracing::error!("Category List Error: {:?}", e);
//             AppData::err(5006, format!("获取分类视频失败: {}", e), None)
//         }
//     }
// }
//
// /// # CASE 10. 精选
// pub async fn case_best(
//     query: ApiUrlParamsQuery,
// ) -> AppData<VideoListResponse> {
//     // 1. 进入精选视频业务
//     match case::home::logic_get_best_list(
//         query.clone(),
//     ).await {
//         Ok(resp) => AppData::ok(resp),
//         Err(e) => AppData::err(5006, format!("获取用户视频失败: {}", e), None),
//     }
// }
//
// //////// END
