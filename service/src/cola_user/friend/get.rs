// // /get.rs
// //
// // 2026/8/3 19:24 Created.
//
// ////////
//
//
// // repository/src/cola_user/service/role/get.rs
// // 仓储 - USER - service - role - 添加
// // 2026/8/3 14:32 Created.
//
// ////////
//
// use cola_data::cola_user::entity::role::UserRoleEntity;
// use cola_data::cola_video::entity::cola_video::cola_video::VideoEntity;
// use crate::cola_user::pg::role::get::UserRoleGetRepo;
//
// /// # [GET SERVICE] - 用户 角色 前台 服务
// pub struct UserRoleGetService;
//
// // 构造函数
// impl UserRoleGetService {
//     //
//
//     ////////
//
//     /// # 1. [SERVICE] - 查找最新的角色列表
//     pub async fn find_new_role_list(
//         limit: i64,
//         offset: i64,
//     ) -> Result<Vec<UserRoleEntity>, anyhow::Error> {
//         UserRoleGetRepo::pg_find_new_role_list(limit, offset)
//             .await
//             .map_err(|e| anyhow::anyhow!("[👤 SERVICE]: 获取最新角色列表失败: {}", e))
//     }
//
//     ////////
//
//     /// # 2. [SERVICE] - 查找热门的角色列表
//     pub async fn find_hot_role_list(
//         limit: i64,
//         offset: i64,
//     ) -> Result<Vec<VideoEntity>, anyhow::Error> {
//         UserRoleGetRepo::find_hot_list(limit, offset)
//             .await
//             .map_err(|e| anyhow::anyhow!("[👤 SERVICE]: 获取热门角色列表失败: {}", e))
//     }
//
//     ////////
//
//     /// # 3. [SERVICE] - 查找推荐的角色列表
//     pub async fn find_recommend_role_list(
//         limit: i64,
//         offset: i64,
//     ) -> Result<Vec<VideoEntity>, anyhow::Error> {
//         UserRoleGetRepo::find_recommend_list(limit, offset)
//             .await
//             .map_err(|e| anyhow::anyhow!("[👤 SERVICE]: 获取推荐角色列表失败: {}", e))
//     }
//
//     ////////
// }
//
// //////// END
