// // cola_video/api/view/add.rs
// // 视频 - api - 浏览 - 发布
// // 2026/8/4 20:10 Created.
//
// ////////
//
// use port::app::ctx::AppContext;
// use cola_data::app::data::AppData;
// use cola_data::app::query::ApiGatewayRequest;
// use cola_data::auth::info::auth::AuthContext;
// use crate::case::view::add::VideoViewAddCase;
// ////////
//
// ////////
// pub struct VideoViewAddApi;
//
// // 构造实现
// impl VideoViewAddApi {
//     //
//
//     ////////
//
//     /// # [API HANDLER] - 发布
//     /// * `desc`: `添加浏览记录`
//     pub async fn api_add_video_view(
//         auth: AuthContext,
//         url: ApiGatewayRequest,
//         ctx: &AppContext,
//     ) -> AppData<(VideoSingleResponse)> {
//         let uid = auth.uid;
//         let video_id = url.video_id;
//
//         match VideoViewAddCase::case_add_view(uid, url, ctx).await {
//             // ✅️ OK
//             Ok(resp) => AppData::ok(resp),
//             // ❌️ FAILE
//             Err(e) => {
//                 tracing::error!("[🤐 API] - ❌️ VIDEO_DETAIL_ERROR: {:?}", e);
//                 AppData::err(5001, "[🤐 API]: 发布视频浏览记录失败", None)
//             }
//         }
//     }
// }
//
// //////// END
