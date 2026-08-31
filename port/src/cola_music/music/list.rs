// port/src/music/music/list.rs
//  ⏩️ 端口 - 可乐音乐 - 音乐 - 列表
// 2026/8/22 23:25 Created.

////////

use cola_data::music::info::music::MusicInfo;

////////

/// # [GET PORTS] - 获取
#[async_trait::async_trait]
pub trait MusicListPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 获取最新的音乐信息列表
    async fn get_new_list_infos(
        &self,
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<MusicInfo>)> {
        Err(anyhow::anyhow!("音乐列表适配器尚未装配"))
    }

    ////////

    /// # 2. [PORT] - 获取热门的音乐信息列表
    async fn get_hot_list_infos(
        &self,
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<MusicInfo>)> {
        Err(anyhow::anyhow!("音乐列表适配器尚未装配"))
    }

    ////////

    /// # 3. [PORT] - 获取推荐的音乐信息列表
    async fn get_recommend_list_infos(
        &self,
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<MusicInfo>)> {
        Err(anyhow::anyhow!("音乐列表适配器尚未装配"))
    }

    ////////

    /// # 4. [PORT] - 获取最新的音乐信息列表
    async fn get_cate_list_infos(
        &self,
        cate_id: i64, // 分类 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<MusicInfo>)> {
        Err(anyhow::anyhow!("音乐列表适配器尚未装配"))
    }

    ////////

    /// # 5. [PORT] - 获取关键词音乐信息列表
    async fn get_keyword_list_infos(
        &self,
        key: Option<String>, // 关键词
        limit: i64,          // 数量
        offset: i64,         // 页码
    ) -> anyhow::Result<(Vec<MusicInfo>)> {
        Err(anyhow::anyhow!("音乐列表适配器尚未装配"))
    }

    ////////

    /// # 6. [PORT] - 获取用户的音乐信息列表
    async fn get_user_list_infos(
        &self,
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<MusicInfo>)> {
        Err(anyhow::anyhow!("音乐列表适配器尚未装配"))
    }
}

//////// END
