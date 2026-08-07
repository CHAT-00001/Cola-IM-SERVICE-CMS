// /count  -- 收藏 服务端口
// 2026/6/10 08:23

////////

/// # [PORT] - 收藏
#[async_trait::async_trait]
pub trait CollectRepo: Send + Sync {
    ////////

    /// # [PORT] - 保存
    async fn save_collect_record(
        &self,
        uid: i64,
        video_id: i64,
        is_liked: bool,
    ) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 编辑
    async fn edit_collect_record(
        &self,
        uid: i64,
        video_id: i64,
        is_unliked: bool,
    ) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 删除
    async fn del_collect_record(
        &self,
        uid: i64,
        video_id: i64,
        is_unliked: bool,
    ) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 获取用户收藏的IDs
    async fn get_collect_ids_by_user_id(
        &self,
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<(Vec<i64>)>;
}
