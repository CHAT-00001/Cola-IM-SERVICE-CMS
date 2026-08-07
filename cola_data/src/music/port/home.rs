// cola_data/src/music/port/home2  -- 数据中心 - MUSIC - port - 主页
// 2026/7/7 13:24

////////

use crate::music::info::music::MusicInfo;

/// # [SERVICE PORT] - Home 服务
#[async_trait::async_trait]
pub trait HomePort: Send + Sync {
    ////////

    /// # 1. [PORT] - 最新
    async fn home_new(&self, uid: i64, limit: i64, offset: i64) -> anyhow::Result<(Vec<MusicInfo>)>;

    ////////

    /// # 2. [PORT] - 热门
    async fn home_hot(&self, uid: i64, limit: i64, offset: i64) -> anyhow::Result<(Vec<MusicInfo>)>;

    ////////

    /// # 3. [PORT] - 推荐
    async fn home_recommend(&self, uid: i64, limit: i64, offset: i64) -> anyhow::Result<(Vec<MusicInfo>)>;

    ////////

    /// # 4. [PORT] - 同城
    async fn home_city(&self, city_id: i64, limit: i64, offset: i64) -> anyhow::Result<(Vec<MusicInfo>)>;

    ////////

    /// # 5. [PORT] - 分类
    async fn home_classify(&self, classify_id: i16, limit: i64, offset: i64) -> anyhow::Result<(Vec<MusicInfo>)>;

    ////////

    /// # 6. [PORT] - 搜索
    async fn home_search(&self, key: String, limit: i64, offset: i64) -> anyhow::Result<(Vec<MusicInfo>)>;
}

//////// END
