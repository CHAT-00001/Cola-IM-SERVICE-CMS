// // service/src/cola_video/buy/manage.rs
// // 仓储 - ▶ 可乐视频 - 购买记录 - 管理
// // 2026/8/2 16:49 Created.
//
// ////////
//
// use cola_data::cola_video::info::buy::VideoBuyInfo;
// use repository::cola_video::pg::buy::manage::VideoBuyManageRepo;
//
// ////////
//
// /// # [MANAGE SERVICE] - 管理
// /// * `desc`: `▶ 可乐视频 - 👤 购买记录管理服务`
// pub struct VideoBuyManageService;
//
// impl VideoBuyManageService {
//     //
//
//     ////////
//
//     /// # 1. [SERVICE] - 管理员列表
//     /// * `desc`: `🗣 ADMIN` - `获取所有收藏记录`
//     /// * `condition`: `⚠️ 需要管理员身份 / 运营人员身份`
//     pub async fn get_admin_list(
//         user_id: Option<i64>,    // 用户 ID
//         video_id: Option<i64>,   // 视频 ID
//         start_time: Option<i64>, // 开始时间
//         end_time: Option<i64>,   // 结束时间
//         status_code: i16,        // 状态码
//         limit: i64,              // 数量
//         offset: i64,             // 页码
//     ) -> Result<Vec<VideoBuyInfo>, anyhow::Error> {
//         // 1. 调用 REPOSITORY 获取仓储层实体列表
//         let entities = VideoBuyManageRepo::find_admin_list(
//             user_id,     // 用户 ID
//             video_id,    // 视频 ID
//             start_time,  // 开始时间
//             end_time,    // 结束时间
//             status_code, // 状态码
//             limit,       // 数量
//             offset,      // 页码
//         )
//             .await?;
//
//         // 2. 将实体列表转换为对应的 Info 层展示对象并返回
//         let infos = VideoBuyInfo::from_entity(entities);
//
//         Ok(infos)
//     }
// }
//
// //////// END
