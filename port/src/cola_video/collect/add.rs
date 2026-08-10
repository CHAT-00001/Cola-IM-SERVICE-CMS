// cola_data/src/cola_video/collect/add.rs
// ▶ 可乐视频 - port - 收藏 - 发布
// 2026/8/5 00:04 Created.

////////

/// # [CHECK SERVICE] - 发布
/// * `desc`: `可乐视频 - 视频收藏发布端口`
#[async_trait::async_trait]
pub trait VideoCollectAddPort: Send + Sync {
    //

    ////////

    /// # [PORT] - 保存
    async fn save_collect_record(
        &self,
        uid: i64,
        video_id: i64,
        //is_liked: bool,
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

//////// END
