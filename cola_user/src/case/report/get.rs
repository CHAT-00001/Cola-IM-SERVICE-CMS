// // user/src/case/report/get.rs
// // core - USER - case - report - 获取举报列表 用例
// // 2026/8/2 23:15 Created.
//
// ////////
//
// use anyhow::{Result, anyhow};
// use cola_data::app::page::ListResponse;
// use cola_data::app::query::ApiGatewayRequest;
// use cola_data::cola_video::info::video::VideoInfo;
// use port::ctx::AppContext;
// use tracing::info;
//
// ////////
//
// /// # [REPORT CASE] - 举报列表 用例
// pub struct UserReportGetCase;
//
// impl UserReportGetCase {
//     //
//
//     ////////
//
//     /// # 1. [CASE] - 获取被举报的视频列表
//     /// * `desc`: 先从 report.add 获取举报记录IDs + 总数，再批量获取视频Info
//     pub async fn case_get_report_list(
//         _uid: i64,              // 操作者ID
//         url: ApiGatewayRequest, // 网关请求
//         ctx: &AppContext,       // 全局上下文
//     ) -> Result<ListResponse<VideoInfo>, anyhow::Error> {
//         // 1. 获取举报记录：总数 + 视频IDs
//         let (total, video_ids) = ctx
//             .video
//             .report
//             .add
//             .get_report_record_ids(url.offset, url.limit)
//             .await
//             .map_err(|e| anyhow!("[🤐 REPORT CASE]: ❌️ 获取举报记录IDs失败: {}", e))?;
//
//         if video_ids.is_empty() {
//             info!("[🗣️ REPORT CASE]: ✅️ 举报列表为空");
//             return Ok(ListResponse {
//                 list: Vec::new(),
//                 page: Some(1),
//                 size: Some(10),
//                 qty: Some(10),
//                 total: Some(total),
//                 has_more: Some(false),
//             });
//         }
//
//         // 2. 批量获取视频Info列表
//         let video_infos = ctx
//             .video
//             .view
//             .get
//             .get_video_list_by_ids(video_ids)
//             .await
//             .map_err(|e| anyhow!("[🤐 REPORT CASE]: ❌️ 批量获取视频信息失败: {}", e))?;
//
//         let response = ListResponse {
//             list: video_infos,
//             page: url.page,
//             size: url.qty,
//             qty: url.qty,
//             total: Some(total),
//             has_more: None,
//         };
//
//         info!(
//             "[🗣️ REPORT CASE]: ✅️ 举报列表查询成功, total={}, count={}",
//             total,
//             response.list.len()
//         );
//         Ok(response)
//     }
// }
//
// //////// END
