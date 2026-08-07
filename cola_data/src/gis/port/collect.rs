// /count  -- 收藏 服务端口
// 2026/7/7

//////

use crate::gis::command::collect::PoiCollectCommand;

/// # [PORT] - 收藏
#[async_trait::async_trait]
pub trait CollectRepo: Send + Sync {
    ////////

    /// # [PORT] - 保存收藏记录
    async fn save_collect_record(
        &self,
        uid: i64,
        poi_id: i64,
        cmd: PoiCollectCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 删除收藏记录
    async fn del_collect_record(
        &self,
        uid: i64,
        poi_id: i64,
    ) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 获取用户收藏的IDs
    async fn get_collect_ids_by_user_id(
        &self,
        user_id: i64,
        keyword: Option<String>,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<i64>>;
}