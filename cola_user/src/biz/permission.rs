// // cola_user/src/biz/permission.rs  -- 可乐用户 - 逻辑层 - 用户权限
// // 2026/4/23 11:36 by wx: cestbon10080
//
// ////////
//
// use anyhow::Result;
//
// ////////
//
//
// /// # 1. [CHECK] - 检查用户权限
// pub async fn logic_check_user_publish_permission(
//     uid: i64,
// ) -> Result<(), anyhow::Error> {
//     VideoUserService.check_perm(uid).await
//         .map_err(|e| anyhow::anyhow!("权限检查失败: {}", e))?;
//     Ok(())
// }
//
// //////// END
//
//
