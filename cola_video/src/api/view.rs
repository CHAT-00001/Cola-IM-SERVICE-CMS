// // cola_video/src/api/get -- 接口层 浏览
// // 2026-03-30 07:55
//
// ////////
//
// mod add;
//
// use crate::model::request::file::CommentRequest;
// use crate::model::vo::video::VideoSingleResponse;
// use cola_data::app::data::AppData;
// use cola_data::app::query::ApiGatewayRequest;
// use cola_data::auth::info::auth::AuthContext;
// use port::app::ctx::AppContext;
//
// ////////
//
// ////////
//
// /// # [API HANDLER] - 查看视频详情
// /// * 1001
// pub async fn handler_get_video_detail(
//     auth: AuthContext,
//     url: ApiGatewayRequest,
//     ctx: &AppContext,
// ) -> AppData<VideoSingleResponse> {
//     let uid = auth.uid;
//     let video_id = url.video_id;
//
//     match ViewCase::case_get_video_detail(uid, url, ctx).await {
//         Ok(resp) => AppData::ok(resp),
//         Err(e) => {
//             tracing::error!("VIDEO_DETAIL_ERROR: {:?}", e);
//             AppData::err(5001, "APP: 获取视频详情失败", None)
//         }
//     }
// }
//
// //////// END
