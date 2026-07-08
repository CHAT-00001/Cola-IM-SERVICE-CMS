// cola_data/src/music/port/collect.rs  -- 数据中心 - MUSIC - port - 收藏
// 2026/7/7 13:48

////////

/// # [SERVICE PORT] - 音乐 收藏 服务端口
#[async_trait::async_trait]
pub trait CollectRepo: Send + Sync {
    ////////

    /// # 1. [PORT] - 💾 添加收藏记录
    async fn save_collect_record(&self, uid: i64, music_id: i64, status: i16)
    -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - ⚙️ 编辑收藏记录
    async fn update_collect_record(
        &self,
        uid: i64,
        music_id: i64,
        status: i16,
    ) -> anyhow::Result<()>;

    ////////

    /// # 3. [PORT] - 🆔 👤 根据用户ID获取收藏的IDs
    async fn get_collect_ids_user_id(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<i64>)>;

    ////////

    /// # 4. [PORT] - ❌️ 👤 用户软删除音乐收藏记录(支持批量)
    async fn user_delete_collect_record(&self, uid: i64, music_ids: Vec<i64>)
    -> anyhow::Result<()>;

    ////////

    /// # 5. [PORT] - ❌️ 👤 根据用户ID软删除音乐收藏记录(用户注销/永封/删除时)
    async fn sync_delete_collect_record_by_user_id(&self, user_id: i64) -> anyhow::Result<()>;

    ////////

    /// # 6. [PORT] - ❌️ 🎶 根据音乐ID软删除音乐收藏记录(音乐失效时)
    async fn sync_delete_collect_record_by_music_id(&self, music_id: i64) -> anyhow::Result<()>;
}
