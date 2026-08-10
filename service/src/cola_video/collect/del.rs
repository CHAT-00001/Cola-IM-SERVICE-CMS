// // service/src/cola_video/collect/del.rs
// // 👤 服务 - ▶ 可乐视频 - 收藏记录 - 删除
// // 2026/8/10 00:36 Created.
//
// ////////
//
// use repository::cola_video::pg::collect::del::VideoCollectDelRepo;
// use repository::cola_video::pg::collect::manage::VideoCollectManageRepo; // 用于查询收藏关联的视频信息
// use repository::cola_video::pg::video::count::VideoCountRepo;
//
// ////////
//
// /// # [DELETE SERVICE] - 删除
// /// * `desc`: `▶ 可乐视频 - 👤 收藏删除 SERVICE`
// pub struct VideoCollectDelService;
//
// impl VideoCollectDelService {
//     //
//
//     ////////
//
//     /// # 1. [SERVICE] - 删除单条收藏记录 + 同步视频收藏数 - 1
//     /// * `uid`: 用户ID (或操作者ID/管理员ID)
//     /// * `collect_id`: 收藏 ID
//     pub async fn single_delete(
//         uid: i64,
//         collect_id: i64, // 收藏 ID
//     ) -> Result<(), anyhow::Error> {
//         // 1. 先查询该收藏记录对应的 video_id（因为删除 Repo 只返回受影响行数，需提前获取关联视频）
//         let video_id = VideoCollectManageRepo::find_video_id_by_collect_id(collect_id).await?;
//
//         // 2. 调用软删除单条记录 (下层 Repo 返回受影响行数 u64)
//         let rows_affected = VideoCollectDelRepo::single_delete_collects_by_id(collect_id).await?;
//
//         // 3. 如果确实删除了记录，且存在对应视频，才同步更新视频的收藏计数 - 1
//         if rows_affected > 0 {
//             if let Some(vid) = video_id {
//                 VideoCountRepo::pg_update_video_collects(vid, -1).await?;
//             }
//         }
//
//         Ok(())
//     }
//
//     ////////
//
//     /// # 2. [SERVICE] - 批量删除收藏记录 + 同步对应视频收藏数
//     /// * `uid`: 用户ID (或操作者ID/管理员ID)
//     /// * `collect_ids`: 收藏 IDs
//     pub async fn batch_delete(
//         uid: i64,            // 操作者 ID
//         collect_ids: &[i64], // 收藏 IDs
//     ) -> Result<(), anyhow::Error> {
//         if collect_ids.is_empty() {
//             return Ok(());
//         }
//
//         // 1. 在删除前，先查询这批 collect_ids 分别对应的视频及删除前的数据映射（用于统计每个视频各减少了多少个收藏）
//         let video_counts = VideoCollectManageRepo::find_video_counts_by_collect_ids(collect_ids).await?;
//
//         // 2. 调用下层 Repo 执行一次性批量删除，获取总共受影响的行数
//         let rows_affected = VideoCollectDelRepo::batch_delete_collect_by_ids(collect_ids).await?;
//
//         // 3. 如果确实删除了记录，遍历受影响的视频，分别扣减对应的收藏数量
//         if rows_affected > 0 {
//             for (video_id, count) in video_counts {
//                 // 根据该视频下被删除的收藏数量进行扣减
//                 VideoCountRepo::pg_update_video_collects(video_id, -(count as i16)).await?;
//             }
//         }
//
//         Ok(())
//     }
// }
//
// //////// END