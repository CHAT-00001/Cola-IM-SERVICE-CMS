// port/src/music/album/add.rs -- 端口 - MUSIC - 专辑 - 发布端口
// 2026/8/22 23:44 Created.

////////

use cola_data::music::command::album::add::CreateMusicAlbumCmd;
use cola_data::music::command::album::edit::UpdateMusicAlbumCmd;
use cola_data::music::info::album::MusicAlbumInfo;
use std::sync::Arc;

////////

/// # [MUSIC ALBUM ADD PORTS] - 音乐专辑发布端口
/// * `desc`: `COLA MUSIC - Album Add Ports.`
#[async_trait::async_trait]
pub trait MusicAlbumAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 创建
    async fn create_album(
        &self,
        uid: i64,                 // 操作者 ID
        ctx: CreateMusicAlbumCmd, // 新建命令
        visibility: i16,          // 内容风控等级生成的可见范围
    ) -> anyhow::Result<(MusicAlbumInfo)> {
        Err(anyhow::anyhow!("用户创建专辑"))
    }

    ////////

    /// # 2. [PORT] - 编辑
    async fn update_album(
        &self,
        uid: i64,                 // 操作者 ID
        album_id: i64,            // 专辑 ID
        cmd: UpdateMusicAlbumCmd, // 更新命令
        visibility: i16,          // 内容风控等级生成的可见范围
    ) -> anyhow::Result<(MusicAlbumInfo)> {
        Err(anyhow::anyhow!("用户更新专辑"))
    }

    ////////

    /// # 3. [PORT] - 修改权限
    async fn change_permission(
        &self,
        uid: i64,             // 操作者 ID
        album_id: i64,        // 专辑 ID
        visibility_perm: i16, // 可见权限
    ) -> anyhow::Result<(bool)> {
        Err(anyhow::anyhow!("修改可见权限"))
    }

    ////////

    /// # 4. [PORT] - 逻辑删除
    async fn delete_album(
        &self,
        uid: i64,      // 操作者 ID
        album_id: i64, // 专辑 ID
    ) -> anyhow::Result<(bool)> {
        Err(anyhow::anyhow!("用户删除专辑"))
    }

    ////////
}

//////// END
