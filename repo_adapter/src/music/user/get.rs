// repo_adapter/src/music/user/get.rs -- 🔌 适配器 - 可乐音乐 - 用户资料 - 管理
// 2026/8/24 23:11 Created.

////////

use cola_data::music::info::user::MusicUserInfo;
use port::cola_music::user::get::MusicUserGetPort;

////////

/// # [GET ADAPTER] - 音乐用户资料获取适配器
pub struct MusicUserGetAdapter;

#[async_trait::async_trait]
impl MusicUserGetPort for MusicUserGetAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 单个获取音乐用户资料
    async fn get_profile_info(
        &self,
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<(MusicUserInfo)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 批量获取音乐用户资料
    async fn batch_get_profile_infos(
        &self,
        user_ids: Vec<i64>, // 用户 IDs
    ) -> anyhow::Result<(Vec<MusicUserInfo>)> {
        todo!()
    }
}

//////// END
