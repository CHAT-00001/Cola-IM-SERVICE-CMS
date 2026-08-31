// repo_adapter/src/music/album/add.rs
// 🔌 适配器 - 可乐音乐 - 专辑 - 发布
// 2026/8/23 00:20 Created.

////////

use cola_data::music::command::album::add::CreateMusicAlbumCmd;
use cola_data::music::command::album::edit::UpdateMusicAlbumCmd;
use cola_data::music::info::album::MusicAlbumInfo;
use port::cola_music::album::add::MusicAlbumAddPort;

////////

/// # [ADD ADAPTER] - 音乐专辑发布适配器
pub struct MusicAlbumAddAdapter;

#[async_trait::async_trait]
impl MusicAlbumAddPort for MusicAlbumAddAdapter {
    ////////

    /// # 1. [ADAPTER] - 创建专辑
    async fn create_album(
        &self,
        uid: i64,
        ctx: CreateMusicAlbumCmd, // 创建命令
        visibility: i16,          // 可见权限
    ) -> anyhow::Result<(MusicAlbumInfo)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 更新专辑
    async fn update_album(
        &self,
        uid: i64,
        album_id: i64,
        cmd: UpdateMusicAlbumCmd,
        visibility: i16, // 可见权限
    ) -> anyhow::Result<(MusicAlbumInfo)> {
        todo!()
    }

    ////////

    /// # 3. [ADAPTER] - 修改专辑权限
    async fn change_permission(
        &self,
        uid: i64,             // 操作者 ID
        album_id: i64,        // 专辑 ID
        visibility_perm: i16, // 可见权限
    ) -> anyhow::Result<(bool)> {
        todo!()
    }

    ////////

    /// # 4. [ADAPTER] - 删除专辑
    async fn delete_album(
        &self,
        uid: i64,      // 操作者 ID
        album_id: i64, // 专辑 ID
    ) -> anyhow::Result<(bool)> {
        todo!()
    }
}

//////// END
