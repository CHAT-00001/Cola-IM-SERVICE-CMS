// service/src/user/black/list.rs
// 👤 服务 - 🗣 可乐用户 - 黑名单 - 记录列表
// 2026/8/7 20:51 Created.

////////

use anyhow::{Result, anyhow};
use cola_data::cola_user::entity::black::UserBlackEntity;
use repository::user::pg::black::list::UserBlackListRepo;

////////

/// # [BLACK LIST SERVICE] - 列表
/// * `desc`: `🗣 可乐用户 - 👤 用户黑名单列表查询服务`
pub struct BlackListService;

// 构造实现
impl BlackListService {
    //

    ////////

    /// # 1. [SERVICE] - 黑名单记录
    /// * `desc`: `根据条件查询黑名单审计日志列表，返回 (总数, 实体列表)`
    pub async fn get_black_record_list(
        uid: i64,
        actor_id: Option<i64>,
        target_id: Option<i64>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: i64,
        offset: i64,
    ) -> Result<(i64, Vec<UserBlackEntity>)> {
        // 1. 调用底层仓储获取 (总数, 列表)
        let (total, entities) = UserBlackListRepo::find_black_record_list(
            actor_id, target_id, start_time, end_time, limit, offset,
        )
        .await
        .map_err(|e| anyhow!("[BLACK SERVICE]: 查询黑名单记录失败: {}", e))?;

        // 2. 记录日志
        tracing::info!(
            "[🗣️ BLACK LIST SERVICE]: ✅️ 查询黑名单记录成功, uid={}, 总数={}, 本页数量={}",
            uid,
            total,
            entities.len()
        );

        // 3. 返回元组
        Ok((total, entities))
    }
}

//////// END
