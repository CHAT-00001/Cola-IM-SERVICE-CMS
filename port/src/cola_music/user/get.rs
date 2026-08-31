// port/src/music/music/get.rs -- 端口 - 可乐音乐 - 音乐 - 获取
// 2026/8/22 23:22 Created.

////////

use cola_data::music::info::user::MusicUserInfo;
////////

/// # [GET PORTS] - 获取
#[async_trait::async_trait]
pub trait MusicUserGetPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 单个获取音乐用户资料信息
    async fn get_profile_info(
        &self,
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<(MusicUserInfo)> {
        Err(anyhow::anyhow!("音乐获取适配器尚未装配"))
    }

    ////////

    /// # 2. [PORT] - 批量获取音乐用户资料信息
    async fn batch_get_profile_infos(
        &self,
        user_ids: Vec<i64>, // 用户 IDs
    ) -> anyhow::Result<(Vec<MusicUserInfo>)> {
        Err(anyhow::anyhow!("音乐获取适配器尚未装配"))
    }
}

//////// END
