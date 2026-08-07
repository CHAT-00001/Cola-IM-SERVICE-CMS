// repo_adapter/src/user/black/list.rs
// 🔌 适配器 - USER - 黑名单 - 列表查询
// 2026/8/6 解耦: 列表查询接口

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::user::info::black::UserBlackInfo;
use cola_data::user::port::black::list::BlackListPort;
use repository::user::pg::black::list::UserBlackListRepo;
use tracing::{error, info};

////////

/// # [LIST ADAPTER] - 获取TA的黑名单
/// * `desc`: `用户黑名单记录的列表服务适配器`
/// * `⚠️ 注意`: `因为黑名单通常需要显示用户资料,所以这个列表仅仅是记录`
pub struct BlackListAdapter;

#[async_trait]
impl BlackListPort for BlackListAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 获取TA的黑名单
    async fn get_black_list(
        &self,
        actor_id: Option<i64>,
        target_id: Option<i64>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: i64,
        offset: i64,
    ) -> Result<(i64, Vec<UserBlackInfo>)> {
        info!(
            "[🔌 ADAPTER] - 🔍 开始查询黑名单审计日志: actor_id = {:?}, target_id = {:?}, start = {:?}, end = {:?}, limit = {}, offset = {}",
            actor_id, target_id, start_time, end_time, limit, offset
        );

        // 🚧 1. 旁路缓存检查 (Cache-Aside: 缓存读取阶段 - 暂时注释)
        // let cache_key = format!(
        //     "user:black:list:{:?}:{:?}:{}:{}:{}:{}",
        //     actor_id, target_id, start_time.unwrap_or(0), end_time.unwrap_or(0), limit, offset
        // );
        // if let Some(cached_data) = CacheClient::get::<(i64, Vec<UserBlackInfo>)>(&cache_key).await? {
        //     info!("[🔌 ADAPTER] - ⚡ 缓存命中黑名单审计日志列表");
        //     return Ok(cached_data);
        // }

        // 🚧 2. PG 检查 (Cache-Aside: 数据库查询阶段)
        let (total, entities) = UserBlackListRepo::find_black_logs(
            actor_id,
            target_id,
            start_time,
            end_time,
            limit,
            offset,
        )
            .await
            .map_err(|e| {
                error!(
                "[🔌 ADAPTER] - ❌️ PG 查询黑名单审计日志列表失败: err = {:?}",
                e
            );
                e
            })?;

        // 3. 实体转视图模型 (Entity -> Info)
        let list = entities
            .into_iter()
            .map(UserBlackInfo::from)
            .collect::<Vec<UserBlackInfo>>();

        info!(
            "[🔌 ADAPTER] - 🗄️ PG 查询黑名单审计日志成功: total = {}, fetched = {}",
            total,
            list.len()
        );

        let result = (total, list);

        // 🚧 3. 回填缓存 (Cache-Aside: 缓存回填阶段 - 暂时注释)
        // let _ = CacheClient::set(&cache_key, &result, Some(Duration::from_secs(300))).await;

        Ok(result)
    }
}

//////// END