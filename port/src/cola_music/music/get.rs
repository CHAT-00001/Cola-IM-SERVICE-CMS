// port/src/music/music/get.rs
// ⏩️ 端口 - 可乐音乐 - 音乐 - 获取
// 2026/8/22 23:22 Created.

////////

use cola_data::music::info::music::MusicInfo;

////////

/// # [GET PORTS] - 获取
#[async_trait::async_trait]
pub trait MusicGetPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 单个获取音乐信息
    async fn get_music_info(
        &self,
        music_id: i64, // 音乐 ID
    ) -> anyhow::Result<(MusicInfo)> {
        Err(anyhow::anyhow!("音乐获取适配器尚未装配"))
    }

    ////////

    /// # 2. [PORT] - 批量获取音乐信息
    async fn batch_get_music_infos(
        &self,
        music_ids: Vec<i64>, // 音乐 IDs
    ) -> anyhow::Result<(Vec<MusicInfo>)> {
        Err(anyhow::anyhow!("音乐获取适配器尚未装配"))
    }
}

//////// END
