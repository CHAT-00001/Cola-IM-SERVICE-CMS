// repo_adapter/src/music/user/list.rs -- 🔌 适配器 - MUSIC - 用户资料 - 列表适配器
// 2026/8/31 21:48 Created.

////////

use cola_data::music::info::user::MusicUserInfo;
use port::cola_music::user::list::MusicUserListPort;

////////

/// # [LIST ADAPTER] - 音乐用户资料列表适配器
/// * `desc`: `COLA MUSIC - Profile List Adapter.`
pub struct MusicUserListAdapter;

#[async_trait::async_trait]
impl MusicUserListPort for MusicUserListAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 最新
    async fn get_new_list_infos(
        &self,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<MusicUserInfo>)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 热门
    async fn get_hot_list_infos(
        &self,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<MusicUserInfo>)> {
        todo!()
    }

    ////////

    /// # 3. [ADAPTER] - 推荐
    async fn get_recommend_list_infos(
        &self,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<MusicUserInfo>)> {
        todo!()
    }

    ////////

    /// # 4. [ADAPTER] - 分类
    async fn get_cate_list_infos(
        &self,
        cate_id: i64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<MusicUserInfo>)> {
        todo!()
    }

    ////////

    /// # 5. [ADAPTER] - 搜索
    async fn get_keyword_list_infos(
        &self,
        key: Option<String>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<MusicUserInfo>)> {
        todo!()
    }
}

//////// END
