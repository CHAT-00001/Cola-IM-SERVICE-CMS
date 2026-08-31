// port/src/music/album/check.rs
//  ⏩️ 端口 - 可乐音乐 - 专辑 - 检查端口
// 2026/8/24 12:13 Created.

////////

use std::sync::Arc;

////////

/// # [MUSIC ALBUM CHECK PORTS] - 音乐专辑检查端口
#[async_trait::async_trait]
pub trait MusicAlbumCheckPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 检查健康分
    async fn health(
        &self,
        uid: i64,      // 操作者 ID
        album_id: i64, // 专辑 ID
    ) -> anyhow::Result<(u16)> {
        Err(anyhow::anyhow!("检查专辑的健康分"))
    }

    ////////

    /// # 2. [PORT] - 检查状态码
    async fn status(
        &self,
        uid: i64,      // 操作者 ID
        album_id: i64, // 专辑 ID
    ) -> anyhow::Result<(i16)> {
        Err(anyhow::anyhow!("检查专辑的状态码"))
    }
}

//////// END
